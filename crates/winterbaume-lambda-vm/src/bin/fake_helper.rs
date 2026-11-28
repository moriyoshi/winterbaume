//! A stand-in for the VM helper that boots no VM: it plays a Lambda
//! runtime on the host against the executor's Runtime API, so that the
//! executor can be tested where there is no hypervisor.
//!
//! The function's handler string says what the "function" does:
//!
//! | handler            | behaviour                                              |
//! |--------------------|--------------------------------------------------------|
//! | `echo`             | returns `{event, env, requestId, arn, deadlineMs, ...}` |
//! | `count`            | returns how many invocations this environment served   |
//! | `file:<path>`      | returns the content of `<root>/var/task/<path>`        |
//! | `log:<text>`       | prints `<text>` to stdout, then behaves like `echo`    |
//! | `error:<type>`     | posts an error of that type                            |
//! | `init-error`       | posts `/init/error` and exits                          |
//! | `exit:<code>`      | exits with that status after receiving the invocation  |
//! | `sleep`            | never answers                                          |
//! | `big`              | answers with more than 6 MB                            |
//! | `pid`              | returns its process id                                 |
//! | `aws:sts`          | calls STS `GetCallerIdentity` through the AWS endpoint |
//! | `no-bootstrap`     | exits 127 at once, as the launch script does           |
//! | `helper-fail:<r>`  | reports that the VM cannot start, with reason `<r>`    |
//!
//! The executor's probe gets a `booting` answer unless the helper was
//! started with `--probe-fail=<reason>`.

use std::io::{BufRead, BufReader, Read, Write};
use std::os::unix::net::UnixStream;

use winterbaume_lambda_vm::proto::{HelperStatus, NO_BOOTSTRAP_EXIT, VmSpec};

fn status(status: HelperStatus) {
    eprint!("{}", status.to_line());
    let _ = std::io::stderr().flush();
}

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let spec: VmSpec = serde_json::from_str(&input).expect("a VM spec");

    if spec.probe {
        let failure =
            std::env::args().find_map(|a| a.strip_prefix("--probe-fail=").map(str::to_string));
        match failure {
            Some(reason) => {
                status(HelperStatus::Failed {
                    reason,
                    message: "fake probe failure".to_string(),
                });
                std::process::exit(70);
            }
            None => {
                status(HelperStatus::Booting);
                std::process::exit(0);
            }
        }
    }

    let handler = spec.env.get("_HANDLER").cloned().unwrap_or_default();
    if let Some(reason) = handler.strip_prefix("helper-fail:") {
        status(HelperStatus::Failed {
            reason: reason.to_string(),
            message: "fake failure".to_string(),
        });
        std::process::exit(70);
    }
    status(HelperStatus::Booting);
    if handler == "no-bootstrap" {
        eprintln!(
            "Couldn't find valid bootstrap(s): [{}]",
            spec.bootstrap.join(" ")
        );
        std::process::exit(NO_BOOTSTRAP_EXIT);
    }
    println!("fake runtime starting pid={}", std::process::id());

    // The fake plays the guest agent too: it talks to the host ends of the
    // forwarded ports directly.
    let socket_for = |guest_port: u16| {
        spec.forwards
            .iter()
            .find(|f| f.guest_port == guest_port)
            .map(|f| f.host_socket.display().to_string())
    };
    let api = socket_for(winterbaume_lambda_vm::proto::RUNTIME_API_GUEST_PORT)
        .expect("the Runtime API is forwarded");
    let aws = socket_for(winterbaume_lambda_vm::proto::AWS_ENDPOINT_GUEST_PORT);
    if handler == "init-error" {
        post(
            &api,
            "/2018-06-01/runtime/init/error",
            Some("Runtime.ImportModuleError"),
            br#"{"errorType":"Runtime.ImportModuleError","errorMessage":"Error: Cannot find module 'index'"}"#,
        );
        std::process::exit(1);
    }

    let mut served = 0u64;
    loop {
        let (headers, event) = get(&api, "/2018-06-01/runtime/invocation/next");
        served += 1;
        let id = headers
            .get("lambda-runtime-aws-request-id")
            .cloned()
            .unwrap_or_default();
        let response_path = format!("/2018-06-01/runtime/invocation/{id}/response");
        let error_path = format!("/2018-06-01/runtime/invocation/{id}/error");
        let event_json: serde_json::Value =
            serde_json::from_slice(&event).unwrap_or(serde_json::Value::Null);

        let echo = || {
            serde_json::json!({
                "event": event_json,
                "requestId": id,
                "arn": headers.get("lambda-runtime-invoked-function-arn"),
                "deadlineMs": headers.get("lambda-runtime-deadline-ms"),
                "traceId": headers.get("lambda-runtime-trace-id"),
                "clientContext": headers.get("lambda-runtime-client-context"),
                "env": spec.env,
                "root": spec.root,
                "vcpus": spec.vcpus,
                "ramMib": spec.ram_mib,
                "workdir": spec.workdir,
                "network": spec.network,
                "forwards": spec.forwards.iter().map(|f| (f.guest_port, f.vsock_port)).collect::<Vec<_>>(),
                "agent": spec.agent,
                "bootstrap": spec.bootstrap,
            })
            .to_string()
            .into_bytes()
        };

        if let Some(text) = handler.strip_prefix("log:") {
            println!("{text}");
            let _ = std::io::stdout().flush();
            post(&api, &response_path, None, &echo());
        } else if let Some(path) = handler.strip_prefix("file:") {
            let content = std::fs::read_to_string(spec.root.join("var/task").join(path))
                .unwrap_or_else(|e| format!("error: {e}"));
            post(
                &api,
                &response_path,
                None,
                serde_json::to_string(&content).unwrap().as_bytes(),
            );
        } else if let Some(error_type) = handler.strip_prefix("error:") {
            let document = serde_json::json!({
                "errorType": error_type,
                "errorMessage": "fake failure",
                "trace": ["at fake"],
            });
            post(
                &api,
                &error_path,
                Some(error_type),
                document.to_string().as_bytes(),
            );
        } else if let Some(code) = handler.strip_prefix("exit:") {
            std::process::exit(code.parse().unwrap_or(1));
        } else {
            match handler.as_str() {
                "count" => post(&api, &response_path, None, served.to_string().as_bytes()),
                "pid" => post(
                    &api,
                    &response_path,
                    None,
                    std::process::id().to_string().as_bytes(),
                ),
                "sleep" => loop {
                    std::thread::sleep(std::time::Duration::from_secs(3600));
                },
                "big" => post(&api, &response_path, None, &vec![b'1'; 7 * 1024 * 1024]),
                // An STS GetCallerIdentity call through the AWS endpoint, as
                // an SDK would send it ( the signature is not checked ).
                "aws:sts" => {
                    let body = match &aws {
                        Some(aws) => {
                            let (status, _, body) = request(
                                aws,
                                "POST",
                                "/",
                                &[
                                    ("Content-Type", "application/x-www-form-urlencoded"),
                                    (
                                        "Authorization",
                                        "AWS4-HMAC-SHA256 Credential=winterbaume/20260101/eu-west-1/sts/aws4_request, SignedHeaders=host, Signature=0",
                                    ),
                                ],
                                b"Action=GetCallerIdentity&Version=2011-06-15",
                            );
                            serde_json::json!({
                                "status": status,
                                "body": String::from_utf8_lossy(&body),
                            })
                        }
                        None => serde_json::json!({ "status": 0, "body": "no AWS endpoint" }),
                    };
                    post(&api, &response_path, None, body.to_string().as_bytes())
                }
                _ => post(&api, &response_path, None, &echo()),
            }
        }
    }
}

/// A minimal HTTP/1.1 exchange over a new connection.
fn request(
    api: &str,
    method: &str,
    path: &str,
    extra: &[(&str, &str)],
    body: &[u8],
) -> (u16, std::collections::HashMap<String, String>, Vec<u8>) {
    let mut stream = UnixStream::connect(api).expect("the forwarded port is reachable");
    let mut head = format!(
        "{method} {path} HTTP/1.1\r\nHost: 127.0.0.1\r\nConnection: close\r\nContent-Length: {}\r\n",
        body.len()
    );
    for (k, v) in extra {
        head.push_str(&format!("{k}: {v}\r\n"));
    }
    head.push_str("\r\n");
    stream.write_all(head.as_bytes()).unwrap();
    // A refused oversized body may close the connection early.
    let _ = stream.write_all(body);
    let mut reader = BufReader::new(stream);
    let mut status_line = String::new();
    reader.read_line(&mut status_line).unwrap();
    let status = status_line
        .split_whitespace()
        .nth(1)
        .and_then(|s| s.parse().ok())
        .unwrap_or(0);
    let mut headers = std::collections::HashMap::new();
    loop {
        let mut line = String::new();
        if reader.read_line(&mut line).unwrap_or(0) == 0 {
            break;
        }
        let line = line.trim_end();
        if line.is_empty() {
            break;
        }
        if let Some((k, v)) = line.split_once(':') {
            headers.insert(k.trim().to_ascii_lowercase(), v.trim().to_string());
        }
    }
    let mut body = Vec::new();
    let _ = reader.read_to_end(&mut body);
    (status, headers, body)
}

fn get(api: &str, path: &str) -> (std::collections::HashMap<String, String>, Vec<u8>) {
    let (status, headers, body) = request(api, "GET", path, &[], b"");
    if status != 200 {
        std::process::exit(2);
    }
    (headers, body)
}

fn post(api: &str, path: &str, error_type: Option<&str>, body: &[u8]) {
    let extra: Vec<(&str, &str)> = error_type
        .map(|t| vec![("Lambda-Runtime-Function-Error-Type", t)])
        .unwrap_or_default();
    request(api, "POST", path, &extra, body);
}
