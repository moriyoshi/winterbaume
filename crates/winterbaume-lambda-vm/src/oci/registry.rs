//! A minimal client for the OCI distribution API: anonymous pulls of
//! manifests and blobs, with the bearer token challenge registries such as
//! `public.ecr.aws` use.

use std::collections::HashMap;
use std::path::Path;
use std::sync::Mutex;

use bytes::Bytes;
use futures_util::StreamExt as _;
use sha2::{Digest, Sha256};
use tokio::io::AsyncWriteExt as _;

use super::layout::{Document, digest_hex, sha256_digest};
use super::{Arch, ImageRef, OciError, ResolvedManifest, media_types};

/// How deeply nested indexes may be.
const MAX_INDEX_DEPTH: usize = 4;
/// Upper bound for a manifest or a configuration blob held in memory.
const MAX_DOCUMENT_BYTES: usize = 16 * 1024 * 1024;

/// Pulls images from one registry.
pub struct RegistryClient {
    base_url: String,
    http: reqwest::Client,
    /// Bearer tokens by repository.  Never logged or put in errors.
    tokens: Mutex<HashMap<String, String>>,
}

impl std::fmt::Debug for RegistryClient {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RegistryClient")
            .field("base_url", &self.base_url)
            .finish_non_exhaustive()
    }
}

/// The parameters of a `WWW-Authenticate: Bearer ...` challenge.
#[derive(Debug, Default, PartialEq, Eq)]
struct Challenge {
    realm: String,
    service: Option<String>,
}

fn parse_challenge(header: &str) -> Option<Challenge> {
    let rest = header.trim();
    let (scheme, params) = rest.split_once(' ')?;
    if !scheme.eq_ignore_ascii_case("bearer") {
        return None;
    }
    let mut challenge = Challenge::default();
    let mut remaining = params.trim();
    while !remaining.is_empty() {
        let (key, after) = remaining.split_once('=')?;
        let key = key.trim().trim_start_matches(',').trim();
        let after = after.trim_start();
        let (value, next) = if let Some(quoted) = after.strip_prefix('"') {
            let end = quoted.find('"')?;
            (&quoted[..end], &quoted[end + 1..])
        } else {
            match after.find(',') {
                Some(end) => (&after[..end], &after[end..]),
                None => (after, ""),
            }
        };
        match key {
            "realm" => challenge.realm = value.to_string(),
            "service" => challenge.service = Some(value.to_string()),
            _ => {}
        }
        remaining = next.trim_start_matches(',').trim();
    }
    (!challenge.realm.is_empty()).then_some(challenge)
}

impl RegistryClient {
    /// A client for the registry at `base_url`, such as
    /// `https://public.ecr.aws`.
    pub fn new(base_url: impl Into<String>) -> Self {
        let http = reqwest::Client::builder()
            .user_agent(concat!("winterbaume-lambda-vm/", env!("CARGO_PKG_VERSION")))
            .build()
            .expect("the HTTP client configuration is valid");
        Self {
            base_url: base_url.into().trim_end_matches('/').to_string(),
            http,
            tokens: Mutex::new(HashMap::new()),
        }
    }

    fn url(&self, image: &ImageRef, kind: &str, reference: &str) -> String {
        format!(
            "{}/v2/{}/{kind}/{reference}",
            self.base_url, image.repository
        )
    }

    fn token(&self, repository: &str) -> Option<String> {
        self.tokens
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .get(repository)
            .cloned()
    }

    /// Obtain an anonymous pull token for `repository`.
    async fn authenticate(&self, repository: &str, challenge: &Challenge) -> Result<(), OciError> {
        let mut query = vec![("scope", format!("repository:{repository}:pull"))];
        if let Some(service) = &challenge.service {
            query.push(("service", service.clone()));
        }
        let response = self
            .http
            .get(&challenge.realm)
            .query(&query)
            .send()
            .await
            .map_err(|e| {
                OciError::Registry(format!("token request failed: {}", e.without_url()))
            })?;
        if !response.status().is_success() {
            return Err(OciError::Registry(format!(
                "token request for {repository} answered {}",
                response.status()
            )));
        }
        #[derive(serde::Deserialize)]
        struct TokenResponse {
            token: Option<String>,
            access_token: Option<String>,
        }
        let raw = Self::read_limited(response).await?;
        let body: TokenResponse = serde_json::from_slice(&raw)
            .map_err(|_| OciError::Registry("token response is not valid JSON".to_string()))?;
        let token = body
            .token
            .or(body.access_token)
            .ok_or_else(|| OciError::Registry("token response carries no token".to_string()))?;
        self.tokens
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .insert(repository.to_string(), token);
        Ok(())
    }

    /// GET `url`, answering a bearer challenge once, and retrying with
    /// backoff when the registry throttles ( 429 ), fails ( 5xx ) or cannot
    /// be reached: anonymous pulls from `public.ecr.aws` are rate-limited.
    async fn get(
        &self,
        image: &ImageRef,
        url: &str,
        accept: Option<&str>,
    ) -> Result<reqwest::Response, OciError> {
        let mut challenged = false;
        let mut attempt = 0u32;
        loop {
            attempt += 1;
            let mut request = self.http.get(url);
            if let Some(accept) = accept {
                request = request.header(reqwest::header::ACCEPT, accept);
            }
            if let Some(token) = self.token(&image.repository) {
                request = request.bearer_auth(token);
            }
            let response = match request.send().await {
                Ok(response) => response,
                // A refused connection means no registry there: one retry.
                Err(e) if attempt < if e.is_connect() { 2 } else { MAX_ATTEMPTS } => {
                    tracing::debug!(error = %e.without_url(), attempt, "registry request failed; retrying");
                    tokio::time::sleep(backoff(attempt, None)).await;
                    continue;
                }
                Err(e) => {
                    return Err(OciError::Registry(format!(
                        "request failed: {}",
                        e.without_url()
                    )));
                }
            };
            let status = response.status();
            if (status == reqwest::StatusCode::TOO_MANY_REQUESTS || status.is_server_error())
                && attempt < MAX_ATTEMPTS
            {
                let retry_after = response
                    .headers()
                    .get(reqwest::header::RETRY_AFTER)
                    .and_then(|v| v.to_str().ok())
                    .and_then(|v| v.parse::<u64>().ok());
                tracing::debug!(%status, attempt, "registry asked to retry");
                tokio::time::sleep(backoff(attempt, retry_after)).await;
                continue;
            }
            if status == reqwest::StatusCode::UNAUTHORIZED && !challenged {
                let challenge = response
                    .headers()
                    .get(reqwest::header::WWW_AUTHENTICATE)
                    .and_then(|v| v.to_str().ok())
                    .and_then(parse_challenge)
                    .ok_or_else(|| {
                        OciError::Registry(format!(
                            "{} requires authentication this client cannot provide",
                            image.repository
                        ))
                    })?;
                self.authenticate(&image.repository, &challenge).await?;
                challenged = true;
                continue;
            }
            if status == reqwest::StatusCode::NOT_FOUND {
                return Err(OciError::NotFound(format!(
                    "{}: {}",
                    image.repository,
                    url.rsplit('/').next().unwrap_or_default()
                )));
            }
            if !status.is_success() {
                return Err(OciError::Registry(format!(
                    "{} answered {status} for {}",
                    self.base_url,
                    url.strip_prefix(&self.base_url).unwrap_or(url)
                )));
            }
            return Ok(response);
        }
    }

    async fn read_limited(response: reqwest::Response) -> Result<Bytes, OciError> {
        if response
            .content_length()
            .is_some_and(|len| len as usize > MAX_DOCUMENT_BYTES)
        {
            return Err(OciError::Invalid("document is too large".to_string()));
        }
        let mut stream = response.bytes_stream();
        let mut out = Vec::new();
        while let Some(chunk) = stream.next().await {
            let chunk = chunk
                .map_err(|e| OciError::Registry(format!("read failed: {}", e.without_url())))?;
            out.extend_from_slice(&chunk);
            if out.len() > MAX_DOCUMENT_BYTES {
                return Err(OciError::Invalid("document is too large".to_string()));
            }
        }
        Ok(Bytes::from(out))
    }

    async fn manifest(&self, image: &ImageRef, reference: &str) -> Result<Bytes, OciError> {
        let accept = [
            media_types::OCI_INDEX,
            media_types::DOCKER_LIST,
            media_types::OCI_MANIFEST,
            media_types::DOCKER_MANIFEST,
        ]
        .join(", ");
        let url = self.url(image, "manifests", reference);
        let response = self.get(image, &url, Some(&accept)).await?;
        Self::read_limited(response).await
    }

    /// The `linux/<arch>` manifest that `image` resolves to.
    pub async fn resolve(
        &self,
        image: &ImageRef,
        arch: Arch,
    ) -> Result<ResolvedManifest, OciError> {
        let body = self.manifest(image, &image.tag).await?;
        let mut digest = sha256_digest(&body);
        let mut document = Document::parse(&body)?;
        for _ in 0..MAX_INDEX_DEPTH {
            if !document.is_index() {
                return document.into_resolved(digest);
            }
            let chosen = document.select(arch).ok_or_else(|| {
                OciError::NotFound(format!(
                    "{}:{} has no linux/{} image",
                    image.repository,
                    image.tag,
                    arch.oci_name()
                ))
            })?;
            let wanted = chosen.digest.clone();
            digest_hex(&wanted)?;
            let body = self.manifest(image, &wanted).await?;
            let actual = sha256_digest(&body);
            if actual != wanted {
                return Err(OciError::Invalid(format!(
                    "manifest {wanted} has digest {actual}"
                )));
            }
            digest = wanted;
            document = Document::parse(&body)?;
        }
        Err(OciError::Invalid(
            "image indexes nested too deeply".to_string(),
        ))
    }

    /// Fetch a small blob ( an image configuration ), checking its digest.
    pub async fn fetch_blob(&self, image: &ImageRef, digest: &str) -> Result<Bytes, OciError> {
        digest_hex(digest)?;
        let url = self.url(image, "blobs", digest);
        let response = self.get(image, &url, None).await?;
        let body = Self::read_limited(response).await?;
        let actual = sha256_digest(&body);
        if actual != digest {
            return Err(OciError::Invalid(format!(
                "blob {digest} has digest {actual}"
            )));
        }
        Ok(body)
    }

    /// Stream a blob ( a layer ) to `dest`, checking its digest.  The blob
    /// is written next to `dest` first and renamed into place only when it
    /// is complete and correct.  A download cut short is started again.
    pub async fn download_blob(
        &self,
        image: &ImageRef,
        digest: &str,
        dest: &Path,
    ) -> Result<(), OciError> {
        let mut attempt = 0;
        loop {
            attempt += 1;
            match self.download_blob_once(image, digest, dest).await {
                Err(OciError::Registry(message)) if attempt < MAX_ATTEMPTS => {
                    tracing::debug!(%message, attempt, "blob download failed; retrying");
                    tokio::time::sleep(backoff(attempt, None)).await;
                }
                other => return other,
            }
        }
    }

    async fn download_blob_once(
        &self,
        image: &ImageRef,
        digest: &str,
        dest: &Path,
    ) -> Result<(), OciError> {
        digest_hex(digest)?;
        let url = self.url(image, "blobs", digest);
        let response = self.get(image, &url, None).await?;

        let mut part = dest.as_os_str().to_os_string();
        part.push(".part");
        let part = std::path::PathBuf::from(part);
        let result = async {
            let mut file = tokio::fs::File::create(&part).await?;
            let mut hasher = Sha256::new();
            let mut stream = response.bytes_stream();
            while let Some(chunk) = stream.next().await {
                let chunk = chunk.map_err(|e| {
                    OciError::Registry(format!("download failed: {}", e.without_url()))
                })?;
                hasher.update(&chunk);
                file.write_all(&chunk).await?;
            }
            file.flush().await?;
            file.sync_all().await?;
            drop(file);
            let hex: String = hasher
                .finalize()
                .iter()
                .map(|b| format!("{b:02x}"))
                .collect();
            let actual = format!("sha256:{hex}");
            if actual != digest {
                return Err(OciError::Invalid(format!(
                    "blob {digest} has digest {actual}"
                )));
            }
            tokio::fs::rename(&part, dest).await?;
            Ok(())
        }
        .await;
        if result.is_err() {
            let _ = tokio::fs::remove_file(&part).await;
        }
        result
    }
}

/// Attempts per request before giving up.
const MAX_ATTEMPTS: u32 = 6;

/// How long to wait before attempt `attempt + 1`: what the registry asked
/// for, capped, or 0.5 s doubling.
fn backoff(attempt: u32, retry_after_secs: Option<u64>) -> std::time::Duration {
    match retry_after_secs {
        Some(secs) => std::time::Duration::from_secs(secs.min(30)),
        None => std::time::Duration::from_millis(500 << attempt.min(6)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::oci::ImageConfig;
    use http_body_util::Full;
    use hyper::body::Incoming;
    use hyper::{Request, Response, StatusCode};
    use serde_json::json;
    use std::convert::Infallible;
    use std::io::Write;
    use std::sync::Arc;

    #[test]
    fn challenges_are_parsed() {
        assert_eq!(
            parse_challenge(
                r#"Bearer realm="https://public.ecr.aws/token/",service="public.ecr.aws",scope="aws""#
            ),
            Some(Challenge {
                realm: "https://public.ecr.aws/token/".to_string(),
                service: Some("public.ecr.aws".to_string()),
            })
        );
        assert_eq!(
            parse_challenge("bearer realm=http://x/t, service=reg"),
            Some(Challenge {
                realm: "http://x/t".to_string(),
                service: Some("reg".to_string()),
            })
        );
        assert_eq!(parse_challenge(r#"Basic realm="x""#), None);
        assert_eq!(parse_challenge("Bearer service=\"x\""), None);
    }

    /// A registry serving one repository, `lambda/nodejs:22`.
    struct FakeRegistry {
        /// Path to (content type, body).
        routes: HashMap<String, (String, Vec<u8>)>,
        token: String,
        base: String,
        /// Tag lookups still to answer with 429.
        throttle: std::sync::Mutex<u32>,
    }

    fn gzip_layer() -> Vec<u8> {
        let mut builder = tar::Builder::new(Vec::new());
        let mut header = tar::Header::new_gnu();
        header.set_entry_type(tar::EntryType::Regular);
        header.set_mode(0o644);
        header.set_size(2);
        builder
            .append_data(&mut header, "hello", &b"hi"[..])
            .unwrap();
        let tar = builder.into_inner().unwrap();
        let mut encoder = flate2::write::GzEncoder::new(Vec::new(), flate2::Compression::fast());
        encoder.write_all(&tar).unwrap();
        encoder.finish().unwrap()
    }

    struct Fixture {
        base: String,
        arm_manifest: String,
        config: String,
        layer: String,
        layer_bytes: Vec<u8>,
        tampered: String,
    }

    async fn start() -> Fixture {
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let base = format!("http://{}", listener.local_addr().unwrap());

        let mut routes = HashMap::new();
        let repo = "/v2/lambda/nodejs";
        let mut put = |path: String, content_type: &str, body: Vec<u8>| {
            routes.insert(path, (content_type.to_string(), body));
        };
        let config = json!({"config": {"Env": ["PATH=/usr/bin", "LAMBDA_TASK_ROOT=/var/task"],
                                       "Entrypoint": ["/lambda-entrypoint.sh"], "WorkingDir": "/var/task"}})
        .to_string()
        .into_bytes();
        let config_digest = sha256_digest(&config);
        let layer = gzip_layer();
        let layer_digest = sha256_digest(&layer);
        let tampered_digest = sha256_digest(b"original");
        put(
            format!("{repo}/blobs/{config_digest}"),
            "application/octet-stream",
            config,
        );
        // Layers are redirected, as public.ecr.aws does to its CDN.
        put(
            format!("/cdn/{layer_digest}"),
            "application/octet-stream",
            layer.clone(),
        );
        put(
            format!("{repo}/blobs/{tampered_digest}"),
            "application/octet-stream",
            b"changed".to_vec(),
        );

        let manifest = |arch: &str| {
            json!({
                "schemaVersion": 2,
                "mediaType": media_types::OCI_MANIFEST,
                "config": {"mediaType": "application/vnd.oci.image.config.v1+json",
                           "digest": config_digest, "size": 1},
                "layers": [{"mediaType": media_types::OCI_LAYER_GZIP, "digest": layer_digest,
                            "size": layer.len(), "annotations": {"arch": arch}}],
            })
            .to_string()
            .into_bytes()
        };
        let amd = manifest("amd64");
        let arm = manifest("arm64");
        let amd_digest = sha256_digest(&amd);
        let arm_digest = sha256_digest(&arm);
        put(
            format!("{repo}/manifests/{amd_digest}"),
            media_types::OCI_MANIFEST,
            amd,
        );
        put(
            format!("{repo}/manifests/{arm_digest}"),
            media_types::OCI_MANIFEST,
            arm,
        );
        let index = json!({
            "schemaVersion": 2,
            "mediaType": media_types::OCI_INDEX,
            "manifests": [
                {"mediaType": media_types::OCI_MANIFEST, "digest": amd_digest, "size": 1,
                 "platform": {"os": "linux", "architecture": "amd64"}},
                {"mediaType": media_types::OCI_MANIFEST, "digest": amd_digest, "size": 1,
                 "platform": {"os": "unknown", "architecture": "unknown"}},
                {"mediaType": media_types::OCI_MANIFEST, "digest": arm_digest, "size": 1,
                 "platform": {"os": "linux", "architecture": "arm64", "variant": "v8"}},
            ],
        })
        .to_string()
        .into_bytes();
        put(
            format!("{repo}/manifests/22"),
            media_types::OCI_INDEX,
            index,
        );

        let registry = Arc::new(FakeRegistry {
            routes,
            token: "secret-token".to_string(),
            base: base.clone(),
            throttle: std::sync::Mutex::new(2),
        });
        tokio::spawn(async move {
            loop {
                let Ok((stream, _)) = listener.accept().await else {
                    return;
                };
                let registry = registry.clone();
                tokio::spawn(async move {
                    let service = hyper::service::service_fn(move |req| {
                        let registry = registry.clone();
                        async move { Ok::<_, Infallible>(registry.handle(req)) }
                    });
                    let _ = hyper::server::conn::http1::Builder::new()
                        .serve_connection(hyper_util::rt::TokioIo::new(stream), service)
                        .await;
                });
            }
        });
        Fixture {
            base,
            arm_manifest: arm_digest,
            config: config_digest,
            layer: layer_digest,
            layer_bytes: layer,
            tampered: tampered_digest,
        }
    }

    impl FakeRegistry {
        fn handle(&self, req: Request<Incoming>) -> Response<Full<Bytes>> {
            let path = req.uri().path().to_string();
            let respond = |status: StatusCode, body: Vec<u8>| {
                Response::builder()
                    .status(status)
                    .body(Full::new(Bytes::from(body)))
                    .unwrap()
            };
            if path == "/token" {
                let query = req.uri().query().unwrap_or_default();
                assert!(
                    query.contains("scope=repository%3Alambda%2Fnodejs%3Apull"),
                    "{query}"
                );
                assert!(query.contains("service=fake"), "{query}");
                return respond(
                    StatusCode::OK,
                    json!({"token": self.token}).to_string().into_bytes(),
                );
            }
            if path.starts_with("/v2/") {
                let authorised = req
                    .headers()
                    .get("authorization")
                    .and_then(|v| v.to_str().ok())
                    == Some(&format!("Bearer {}", self.token));
                if !authorised {
                    return Response::builder()
                        .status(StatusCode::UNAUTHORIZED)
                        .header(
                            "www-authenticate",
                            format!(
                                r#"Bearer realm="{}/token",service="fake",scope="aws""#,
                                self.base
                            ),
                        )
                        .body(Full::new(Bytes::new()))
                        .unwrap();
                }
                // The first two tag lookups are throttled, as public.ecr.aws
                // throttles anonymous pulls.
                if path.ends_with("/manifests/22") && {
                    let mut left = self.throttle.lock().unwrap();
                    let throttled = *left > 0;
                    *left = left.saturating_sub(1);
                    throttled
                } {
                    return Response::builder()
                        .status(StatusCode::TOO_MANY_REQUESTS)
                        .header("retry-after", "0")
                        .body(Full::new(Bytes::new()))
                        .unwrap();
                }
                if let Some(digest) = path.strip_prefix("/v2/lambda/nodejs/blobs/")
                    && !self.routes.contains_key(&path)
                    && self.routes.contains_key(&format!("/cdn/{digest}"))
                {
                    return Response::builder()
                        .status(StatusCode::TEMPORARY_REDIRECT)
                        .header("location", format!("{}/cdn/{digest}", self.base))
                        .body(Full::new(Bytes::new()))
                        .unwrap();
                }
            }
            match self.routes.get(&path) {
                Some((content_type, body)) => Response::builder()
                    .status(StatusCode::OK)
                    .header("content-type", content_type)
                    .body(Full::new(Bytes::from(body.clone())))
                    .unwrap(),
                None => respond(StatusCode::NOT_FOUND, b"{}".to_vec()),
            }
        }
    }

    fn nodejs(tag: &str) -> ImageRef {
        ImageRef {
            repository: "lambda/nodejs".to_string(),
            tag: tag.to_string(),
        }
    }

    #[tokio::test]
    async fn pulls_the_requested_platform() {
        let fixture = start().await;
        let client = RegistryClient::new(&fixture.base);
        let resolved = client.resolve(&nodejs("22"), Arch::Arm64).await.unwrap();
        assert_eq!(resolved.digest, fixture.arm_manifest);
        assert_eq!(resolved.config_digest, fixture.config);
        assert_eq!(resolved.layers.len(), 1);
        assert_eq!(resolved.layers[0].digest, fixture.layer);
        assert_eq!(resolved.layers[0].media_type, media_types::OCI_LAYER_GZIP);

        let config = client
            .fetch_blob(&nodejs("22"), &fixture.config)
            .await
            .unwrap();
        let config = ImageConfig::from_config_blob(&config).unwrap();
        assert_eq!(config.entrypoint, vec!["/lambda-entrypoint.sh".to_string()]);
        assert_eq!(config.working_dir.as_deref(), Some("/var/task"));
        assert!(
            config
                .env_pairs()
                .any(|(k, v)| k == "PATH" && v == "/usr/bin")
        );

        let dir = tempfile::tempdir().unwrap();
        let dest = dir.path().join("layer");
        client
            .download_blob(&nodejs("22"), &fixture.layer, &dest)
            .await
            .unwrap();
        assert_eq!(std::fs::read(&dest).unwrap(), fixture.layer_bytes);

        let root = dir.path().join("rootfs");
        std::fs::create_dir(&root).unwrap();
        crate::oci::apply_layer(&root, &dest, media_types::OCI_LAYER_GZIP).unwrap();
        assert_eq!(std::fs::read(root.join("hello")).unwrap(), b"hi");
    }

    #[tokio::test]
    async fn digest_mismatches_and_missing_tags_are_reported() {
        let fixture = start().await;
        let client = RegistryClient::new(&fixture.base);
        let dir = tempfile::tempdir().unwrap();
        let dest = dir.path().join("blob");
        let err = client
            .download_blob(&nodejs("22"), &fixture.tampered, &dest)
            .await
            .unwrap_err();
        assert!(matches!(err, OciError::Invalid(_)), "{err}");
        assert!(!dest.exists());
        assert!(!dir.path().join("blob.part").exists());
        assert!(matches!(
            client
                .fetch_blob(&nodejs("22"), &fixture.tampered)
                .await
                .unwrap_err(),
            OciError::Invalid(_)
        ));

        let err = client
            .resolve(&nodejs("404"), Arch::Amd64)
            .await
            .unwrap_err();
        assert!(matches!(err, OciError::NotFound(_)), "{err}");
        assert!(!err.to_string().contains("secret-token"));

        assert!(matches!(
            client
                .fetch_blob(&nodejs("22"), "sha256:../../x")
                .await
                .unwrap_err(),
            OciError::Invalid(_)
        ));
    }
}
