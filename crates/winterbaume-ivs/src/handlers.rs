use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use http::header::HeaderName;
use serde_json::{Value, json};
use winterbaume_core::{
    BackendState, DEFAULT_ACCOUNT_ID, MockRequest, MockResponse, MockService, StateChangeNotifier,
    StatefulService, extract_path, rest_json_error,
};

use crate::state::{IvsError, IvsState};
use crate::views::IvsStateView;
use crate::wire;

const X_AMZN_ERRORTYPE: HeaderName = HeaderName::from_static("x-amzn-errortype");

pub struct IvsService {
    pub(crate) state: Arc<BackendState<IvsState>>,
    pub(crate) notifier: StateChangeNotifier<IvsStateView>,
}

impl IvsService {
    pub fn new() -> Self {
        Self {
            state: Arc::new(BackendState::new()),
            notifier: StateChangeNotifier::new(),
        }
    }
}

impl Default for IvsService {
    fn default() -> Self {
        Self::new()
    }
}

impl MockService for IvsService {
    fn service_name(&self) -> &str {
        "ivs"
    }

    fn url_patterns(&self) -> Vec<&str> {
        vec![
            r"https?://ivs\..*\.amazonaws\.com",
            r"https?://ivs\.amazonaws\.com",
        ]
    }

    fn handle(
        &self,
        request: MockRequest,
    ) -> Pin<Box<dyn Future<Output = MockResponse> + Send + '_>> {
        Box::pin(async move { self.dispatch(request).await })
    }
}

impl IvsService {
    async fn dispatch(&self, request: MockRequest) -> MockResponse {
        let region = winterbaume_core::auth::extract_region_from_uri(&request.uri);
        let account_id = DEFAULT_ACCOUNT_ID;
        let state = self.state.get(account_id, &region);

        let path = extract_path(&request.uri);
        let method = request.method.as_str();

        // Helper to parse JSON body
        let body: Value = match serde_json::from_slice(&request.body) {
            Ok(v) => v,
            Err(_) => {
                if !request.body.is_empty() {
                    return rest_json_error(400, "ValidationException", "Invalid JSON body");
                }
                Value::Object(serde_json::Map::new())
            }
        };

        let response = match (method, path.as_str()) {
            // --- Channels ---
            ("POST", "/CreateChannel") => {
                self.handle_create_channel(&state, &body, &region, account_id)
                    .await
            }
            ("POST", "/GetChannel") => self.handle_get_channel(&state, &body).await,
            ("POST", "/DeleteChannel") => self.handle_delete_channel(&state, &body).await,
            ("POST", "/ListChannels") => self.handle_list_channels(&state).await,
            ("POST", "/BatchGetChannel") => self.handle_batch_get_channel(&state, &body).await,
            ("POST", "/UpdateChannel") => self.handle_update_channel(&state, &body).await,

            // --- Stream keys ---
            ("POST", "/CreateStreamKey") => {
                self.handle_create_stream_key(&state, &body, &region, account_id)
                    .await
            }
            ("POST", "/GetStreamKey") => self.handle_get_stream_key(&state, &body).await,
            ("POST", "/DeleteStreamKey") => self.handle_delete_stream_key(&state, &body).await,
            ("POST", "/ListStreamKeys") => self.handle_list_stream_keys(&state, &body).await,
            ("POST", "/BatchGetStreamKey") => self.handle_batch_get_stream_key(&state, &body).await,

            // --- Recording configurations ---
            ("POST", "/CreateRecordingConfiguration") => {
                self.handle_create_recording_configuration(&state, &body, &region, account_id)
                    .await
            }
            ("POST", "/GetRecordingConfiguration") => {
                self.handle_get_recording_configuration(&state, &body).await
            }
            ("POST", "/DeleteRecordingConfiguration") => {
                self.handle_delete_recording_configuration(&state, &body)
                    .await
            }
            ("POST", "/ListRecordingConfigurations") => {
                self.handle_list_recording_configurations(&state).await
            }

            // --- Playback key pairs ---
            ("POST", "/ImportPlaybackKeyPair") => {
                self.handle_import_playback_key_pair(&state, &body, &region, account_id)
                    .await
            }
            ("POST", "/GetPlaybackKeyPair") => {
                self.handle_get_playback_key_pair(&state, &body).await
            }
            ("POST", "/DeletePlaybackKeyPair") => {
                self.handle_delete_playback_key_pair(&state, &body).await
            }
            ("POST", "/ListPlaybackKeyPairs") => self.handle_list_playback_key_pairs(&state).await,

            // --- Playback restriction policies ---
            ("POST", "/CreatePlaybackRestrictionPolicy") => {
                self.handle_create_playback_restriction_policy(&state, &body, &region, account_id)
                    .await
            }
            ("POST", "/GetPlaybackRestrictionPolicy") => {
                self.handle_get_playback_restriction_policy(&state, &body)
                    .await
            }
            ("POST", "/DeletePlaybackRestrictionPolicy") => {
                self.handle_delete_playback_restriction_policy(&state, &body)
                    .await
            }
            ("POST", "/ListPlaybackRestrictionPolicies") => {
                self.handle_list_playback_restriction_policies(&state).await
            }
            ("POST", "/UpdatePlaybackRestrictionPolicy") => {
                self.handle_update_playback_restriction_policy(&state, &body)
                    .await
            }

            // --- Streams (stateless stubs) ---
            ("POST", "/GetStream") => self.handle_get_stream(&state, &body).await,
            ("POST", "/GetStreamSession") => self.handle_get_stream_session(&state, &body).await,
            ("POST", "/ListStreams") => self.handle_list_streams(&state).await,
            ("POST", "/ListStreamSessions") => {
                self.handle_list_stream_sessions(&state, &body).await
            }
            ("POST", "/StopStream") => self.handle_stop_stream(&state, &body).await,

            // --- Tags (path-based routing) ---
            ("GET", path) if path.starts_with("/tags/") => {
                let resource_arn = decode_resource_arn(&path[6..]);
                self.handle_list_tags_for_resource(&state, &resource_arn)
                    .await
            }
            ("POST", path) if path.starts_with("/tags/") => {
                let resource_arn = decode_resource_arn(&path[6..]);
                self.handle_tag_resource(&state, &resource_arn, &body).await
            }
            ("DELETE", path) if path.starts_with("/tags/") => {
                let resource_arn = decode_resource_arn(&path[6..]);
                // tag keys come from query string but MockRequest doesn't separate them;
                // parse from the URI query string
                let tag_keys = extract_tag_keys_from_uri(&request.uri);
                self.handle_untag_resource(&state, &resource_arn, &tag_keys)
                    .await
            }

            // --- Other ---
            ("POST", "/PutMetadata") => self.handle_put_metadata(&state, &body).await,
            ("POST", "/BatchStartViewerSessionRevocation") => {
                self.handle_batch_start_viewer_session_revocation(&body)
                    .await
            }
            ("POST", "/StartViewerSessionRevocation") => {
                self.handle_start_viewer_session_revocation(&body).await
            }

            _ => rest_json_error(404, "ResourceNotFoundException", "Not found"),
        };

        // Notify on successful mutations
        let status = response.status;
        if matches!(method, "POST" | "DELETE")
            && status / 100 == 2
            && (path.starts_with("/tags/")
                || !matches!(
                    path.as_str(),
                    "/GetChannel"
                        | "/GetStreamKey"
                        | "/GetRecordingConfiguration"
                        | "/GetPlaybackKeyPair"
                        | "/GetPlaybackRestrictionPolicy"
                        | "/GetStream"
                        | "/GetStreamSession"
                        | "/ListChannels"
                        | "/ListStreamKeys"
                        | "/ListRecordingConfigurations"
                        | "/ListPlaybackKeyPairs"
                        | "/ListPlaybackRestrictionPolicies"
                        | "/ListStreams"
                        | "/ListStreamSessions"
                        | "/BatchGetChannel"
                        | "/BatchGetStreamKey"
                ))
        {
            self.notify_state_changed(account_id, &region).await;
        }

        response
    }

    // -------------------------------------------------------------------------
    // Channel handlers
    // -------------------------------------------------------------------------

    async fn handle_create_channel(
        &self,
        state: &Arc<tokio::sync::RwLock<IvsState>>,
        body: &Value,
        region: &str,
        account_id: &str,
    ) -> MockResponse {
        let name = body
            .get("name")
            .and_then(|v| v.as_str())
            .unwrap_or("default");

        let latency_mode = body
            .get("latencyMode")
            .and_then(|v| v.as_str())
            .unwrap_or("LOW");

        let channel_type = body
            .get("type")
            .and_then(|v| v.as_str())
            .unwrap_or("STANDARD");

        let mut state = state.write().await;
        match state.create_channel(name, latency_mode, channel_type, region, account_id) {
            Ok(channel) => wire::serialize_create_channel_response(&wire::CreateChannelResponse {
                channel: Some(wire::Channel {
                    arn: Some(channel.arn.clone()),
                    name: Some(channel.name.clone()),
                    latency_mode: Some(channel.latency_mode.clone()),
                    r#type: Some(channel.channel_type.clone()),
                    authorized: Some(channel.authorized),
                    tags: if channel.tags.is_empty() {
                        None
                    } else {
                        Some(channel.tags.clone())
                    },
                    ..Default::default()
                }),
                ..Default::default()
            }),
            Err(e) => ivs_error_response(&e),
        }
    }

    async fn handle_get_channel(
        &self,
        state: &Arc<tokio::sync::RwLock<IvsState>>,
        body: &Value,
    ) -> MockResponse {
        let arn = match body.get("arn").and_then(|v| v.as_str()) {
            Some(a) if !a.is_empty() => a,
            _ => return rest_json_error(400, "ValidationException", "Missing or empty 'arn'"),
        };

        let state = state.read().await;
        match state.get_channel(arn) {
            Ok(channel) => wire::serialize_get_channel_response(&wire::GetChannelResponse {
                channel: Some(wire::Channel {
                    arn: Some(channel.arn.clone()),
                    name: Some(channel.name.clone()),
                    latency_mode: Some(channel.latency_mode.clone()),
                    r#type: Some(channel.channel_type.clone()),
                    authorized: Some(channel.authorized),
                    tags: if channel.tags.is_empty() {
                        None
                    } else {
                        Some(channel.tags.clone())
                    },
                    ..Default::default()
                }),
            }),
            Err(e) => ivs_error_response(&e),
        }
    }

    async fn handle_delete_channel(
        &self,
        state: &Arc<tokio::sync::RwLock<IvsState>>,
        body: &Value,
    ) -> MockResponse {
        let arn = match body.get("arn").and_then(|v| v.as_str()) {
            Some(a) if !a.is_empty() => a,
            _ => return rest_json_error(400, "ValidationException", "Missing or empty 'arn'"),
        };

        let mut state = state.write().await;
        match state.delete_channel(arn) {
            Ok(()) => wire::serialize_delete_channel_response(),
            Err(e) => ivs_error_response(&e),
        }
    }

    async fn handle_list_channels(
        &self,
        state: &Arc<tokio::sync::RwLock<IvsState>>,
    ) -> MockResponse {
        let state = state.read().await;
        let channels = state.list_channels();
        wire::serialize_list_channels_response(&wire::ListChannelsResponse {
            channels: Some(
                channels
                    .iter()
                    .map(|ch| wire::ChannelSummary {
                        arn: Some(ch.arn.clone()),
                        name: Some(ch.name.clone()),
                        latency_mode: Some(ch.latency_mode.clone()),
                        r#type: Some(ch.channel_type.clone()),
                        authorized: Some(ch.authorized),
                        tags: if ch.tags.is_empty() {
                            None
                        } else {
                            Some(ch.tags.clone())
                        },
                        ..Default::default()
                    })
                    .collect(),
            ),
            ..Default::default()
        })
    }

    async fn handle_batch_get_channel(
        &self,
        state: &Arc<tokio::sync::RwLock<IvsState>>,
        body: &Value,
    ) -> MockResponse {
        let arns = match body.get("arns").and_then(|v| v.as_array()) {
            Some(arr) => arr
                .iter()
                .filter_map(|v| v.as_str().map(|s| s.to_string()))
                .collect::<Vec<_>>(),
            None => return rest_json_error(400, "ValidationException", "Missing or empty 'arns'"),
        };

        let state = state.read().await;
        let mut channels = Vec::new();
        let mut errors = Vec::new();

        for arn in &arns {
            match state.get_channel(arn) {
                Ok(channel) => {
                    channels.push(wire::Channel {
                        arn: Some(channel.arn.clone()),
                        name: Some(channel.name.clone()),
                        latency_mode: Some(channel.latency_mode.clone()),
                        r#type: Some(channel.channel_type.clone()),
                        authorized: Some(channel.authorized),
                        ..Default::default()
                    });
                }
                Err(_) => {
                    errors.push(wire::BatchError {
                        arn: Some(arn.clone()),
                        code: Some("404".to_string()),
                        message: Some("Channel not found".to_string()),
                    });
                }
            }
        }

        wire::serialize_batch_get_channel_response(&wire::BatchGetChannelResponse {
            channels: Some(channels),
            errors: Some(errors),
        })
    }

    async fn handle_update_channel(
        &self,
        state: &Arc<tokio::sync::RwLock<IvsState>>,
        body: &Value,
    ) -> MockResponse {
        let arn = match body.get("arn").and_then(|v| v.as_str()) {
            Some(a) if !a.is_empty() => a,
            _ => return rest_json_error(400, "ValidationException", "Missing or empty 'arn'"),
        };

        let latency_mode = body.get("latencyMode").and_then(|v| v.as_str());
        let channel_type = body.get("type").and_then(|v| v.as_str());
        let name = body.get("name").and_then(|v| v.as_str());
        let authorized = body.get("authorized").and_then(|v| v.as_bool());

        let mut state = state.write().await;
        match state.update_channel(arn, name, latency_mode, channel_type, authorized) {
            Ok(channel) => wire::serialize_update_channel_response(&wire::UpdateChannelResponse {
                channel: Some(wire::Channel {
                    arn: Some(channel.arn.clone()),
                    name: Some(channel.name.clone()),
                    latency_mode: Some(channel.latency_mode.clone()),
                    r#type: Some(channel.channel_type.clone()),
                    authorized: Some(channel.authorized),
                    tags: if channel.tags.is_empty() {
                        None
                    } else {
                        Some(channel.tags.clone())
                    },
                    ..Default::default()
                }),
            }),
            Err(e) => ivs_error_response(&e),
        }
    }

    // -------------------------------------------------------------------------
    // Stream key handlers
    // -------------------------------------------------------------------------

    async fn handle_create_stream_key(
        &self,
        state: &Arc<tokio::sync::RwLock<IvsState>>,
        body: &Value,
        region: &str,
        account_id: &str,
    ) -> MockResponse {
        let channel_arn = match body.get("channelArn").and_then(|v| v.as_str()) {
            Some(a) if !a.is_empty() => a,
            _ => {
                return rest_json_error(
                    400,
                    "ValidationException",
                    "Missing or empty 'channelArn'",
                );
            }
        };
        let tags = extract_tags(body);

        let mut state = state.write().await;
        match state.create_stream_key(channel_arn, tags, region, account_id) {
            Ok(sk) => wire::serialize_create_stream_key_response(&wire::CreateStreamKeyResponse {
                stream_key: Some(wire::StreamKey {
                    arn: Some(sk.arn.clone()),
                    channel_arn: Some(sk.channel_arn.clone()),
                    value: Some(sk.value.clone()),
                    tags: if sk.tags.is_empty() {
                        None
                    } else {
                        Some(sk.tags.clone())
                    },
                }),
            }),
            Err(e) => ivs_error_response(&e),
        }
    }

    async fn handle_get_stream_key(
        &self,
        state: &Arc<tokio::sync::RwLock<IvsState>>,
        body: &Value,
    ) -> MockResponse {
        let arn = match body.get("arn").and_then(|v| v.as_str()) {
            Some(a) if !a.is_empty() => a,
            _ => return rest_json_error(400, "ValidationException", "Missing or empty 'arn'"),
        };
        let state = state.read().await;
        match state.get_stream_key(arn) {
            Ok(sk) => wire::serialize_get_stream_key_response(&wire::GetStreamKeyResponse {
                stream_key: Some(wire::StreamKey {
                    arn: Some(sk.arn.clone()),
                    channel_arn: Some(sk.channel_arn.clone()),
                    value: Some(sk.value.clone()),
                    tags: if sk.tags.is_empty() {
                        None
                    } else {
                        Some(sk.tags.clone())
                    },
                }),
            }),
            Err(e) => ivs_error_response(&e),
        }
    }

    async fn handle_delete_stream_key(
        &self,
        state: &Arc<tokio::sync::RwLock<IvsState>>,
        body: &Value,
    ) -> MockResponse {
        let arn = match body.get("arn").and_then(|v| v.as_str()) {
            Some(a) if !a.is_empty() => a,
            _ => return rest_json_error(400, "ValidationException", "Missing or empty 'arn'"),
        };
        let mut state = state.write().await;
        match state.delete_stream_key(arn) {
            Ok(()) => wire::serialize_delete_stream_key_response(),
            Err(e) => ivs_error_response(&e),
        }
    }

    async fn handle_list_stream_keys(
        &self,
        state: &Arc<tokio::sync::RwLock<IvsState>>,
        body: &Value,
    ) -> MockResponse {
        let channel_arn = match body.get("channelArn").and_then(|v| v.as_str()) {
            Some(a) if !a.is_empty() => a,
            _ => {
                return rest_json_error(
                    400,
                    "ValidationException",
                    "Missing or empty 'channelArn'",
                );
            }
        };
        let state = state.read().await;
        let keys = state.list_stream_keys(channel_arn);
        wire::serialize_list_stream_keys_response(&wire::ListStreamKeysResponse {
            stream_keys: Some(
                keys.iter()
                    .map(|sk| wire::StreamKeySummary {
                        arn: Some(sk.arn.clone()),
                        channel_arn: Some(sk.channel_arn.clone()),
                        tags: if sk.tags.is_empty() {
                            None
                        } else {
                            Some(sk.tags.clone())
                        },
                    })
                    .collect(),
            ),
            ..Default::default()
        })
    }

    async fn handle_batch_get_stream_key(
        &self,
        state: &Arc<tokio::sync::RwLock<IvsState>>,
        body: &Value,
    ) -> MockResponse {
        let arns = match body.get("arns").and_then(|v| v.as_array()) {
            Some(arr) => arr
                .iter()
                .filter_map(|v| v.as_str().map(|s| s.to_string()))
                .collect::<Vec<_>>(),
            None => return rest_json_error(400, "ValidationException", "Missing 'arns'"),
        };
        let state = state.read().await;
        let (found, errs) = state.batch_get_stream_keys(&arns);
        wire::serialize_batch_get_stream_key_response(&wire::BatchGetStreamKeyResponse {
            stream_keys: Some(
                found
                    .iter()
                    .map(|sk| wire::StreamKey {
                        arn: Some(sk.arn.clone()),
                        channel_arn: Some(sk.channel_arn.clone()),
                        value: Some(sk.value.clone()),
                        tags: if sk.tags.is_empty() {
                            None
                        } else {
                            Some(sk.tags.clone())
                        },
                    })
                    .collect(),
            ),
            errors: Some(
                errs.iter()
                    .map(|(arn, msg)| wire::BatchError {
                        arn: Some(arn.clone()),
                        code: Some("404".to_string()),
                        message: Some(msg.clone()),
                    })
                    .collect(),
            ),
        })
    }

    // -------------------------------------------------------------------------
    // Recording configuration handlers
    // -------------------------------------------------------------------------

    async fn handle_create_recording_configuration(
        &self,
        state: &Arc<tokio::sync::RwLock<IvsState>>,
        body: &Value,
        region: &str,
        account_id: &str,
    ) -> MockResponse {
        let name = body.get("name").and_then(|v| v.as_str());
        let s3_bucket_name = body
            .get("destinationConfiguration")
            .and_then(|dc| dc.get("s3"))
            .and_then(|s3| s3.get("bucketName"))
            .and_then(|v| v.as_str());
        let tags = extract_tags(body);

        let mut state = state.write().await;
        match state.create_recording_configuration(name, s3_bucket_name, tags, region, account_id) {
            Ok(rc) => wire::serialize_create_recording_configuration_response(
                &wire::CreateRecordingConfigurationResponse {
                    recording_configuration: Some(recording_config_to_wire(rc)),
                },
            ),
            Err(e) => ivs_error_response(&e),
        }
    }

    async fn handle_get_recording_configuration(
        &self,
        state: &Arc<tokio::sync::RwLock<IvsState>>,
        body: &Value,
    ) -> MockResponse {
        let arn = match body.get("arn").and_then(|v| v.as_str()) {
            Some(a) if !a.is_empty() => a,
            _ => return rest_json_error(400, "ValidationException", "Missing or empty 'arn'"),
        };
        let state = state.read().await;
        match state.get_recording_configuration(arn) {
            Ok(rc) => wire::serialize_get_recording_configuration_response(
                &wire::GetRecordingConfigurationResponse {
                    recording_configuration: Some(recording_config_to_wire(rc)),
                },
            ),
            Err(e) => ivs_error_response(&e),
        }
    }

    async fn handle_delete_recording_configuration(
        &self,
        state: &Arc<tokio::sync::RwLock<IvsState>>,
        body: &Value,
    ) -> MockResponse {
        let arn = match body.get("arn").and_then(|v| v.as_str()) {
            Some(a) if !a.is_empty() => a,
            _ => return rest_json_error(400, "ValidationException", "Missing or empty 'arn'"),
        };
        let mut state = state.write().await;
        match state.delete_recording_configuration(arn) {
            Ok(()) => wire::serialize_delete_recording_configuration_response(),
            Err(e) => ivs_error_response(&e),
        }
    }

    async fn handle_list_recording_configurations(
        &self,
        state: &Arc<tokio::sync::RwLock<IvsState>>,
    ) -> MockResponse {
        let state = state.read().await;
        let configs = state.list_recording_configurations();
        wire::serialize_list_recording_configurations_response(
            &wire::ListRecordingConfigurationsResponse {
                recording_configurations: Some(
                    configs
                        .iter()
                        .map(|rc| wire::RecordingConfigurationSummary {
                            arn: Some(rc.arn.clone()),
                            name: Some(rc.name.clone()),
                            destination_configuration: Some(wire::DestinationConfiguration {
                                s3: rc.destination_configuration.s3_bucket_name.as_ref().map(
                                    |bucket| wire::S3DestinationConfiguration {
                                        bucket_name: bucket.clone(),
                                    },
                                ),
                            }),
                            state: Some(rc.state.clone()),
                            tags: if rc.tags.is_empty() {
                                None
                            } else {
                                Some(rc.tags.clone())
                            },
                        })
                        .collect(),
                ),
                ..Default::default()
            },
        )
    }

    // -------------------------------------------------------------------------
    // Playback key pair handlers
    // -------------------------------------------------------------------------

    async fn handle_import_playback_key_pair(
        &self,
        state: &Arc<tokio::sync::RwLock<IvsState>>,
        body: &Value,
        region: &str,
        account_id: &str,
    ) -> MockResponse {
        let public_key_material = match body.get("publicKeyMaterial").and_then(|v| v.as_str()) {
            Some(pk) if !pk.is_empty() => pk,
            _ => return rest_json_error(400, "ValidationException", "Missing 'publicKeyMaterial'"),
        };
        let name = body.get("name").and_then(|v| v.as_str());
        let tags = extract_tags(body);

        let mut state = state.write().await;
        match state.import_playback_key_pair(name, public_key_material, tags, region, account_id) {
            Ok(kp) => wire::serialize_import_playback_key_pair_response(
                &wire::ImportPlaybackKeyPairResponse {
                    key_pair: Some(wire::PlaybackKeyPair {
                        arn: Some(kp.arn.clone()),
                        name: Some(kp.name.clone()),
                        fingerprint: Some(kp.fingerprint.clone()),
                        tags: if kp.tags.is_empty() {
                            None
                        } else {
                            Some(kp.tags.clone())
                        },
                    }),
                },
            ),
            Err(e) => ivs_error_response(&e),
        }
    }

    async fn handle_get_playback_key_pair(
        &self,
        state: &Arc<tokio::sync::RwLock<IvsState>>,
        body: &Value,
    ) -> MockResponse {
        let arn = match body.get("arn").and_then(|v| v.as_str()) {
            Some(a) if !a.is_empty() => a,
            _ => return rest_json_error(400, "ValidationException", "Missing or empty 'arn'"),
        };
        let state = state.read().await;
        match state.get_playback_key_pair(arn) {
            Ok(kp) => {
                wire::serialize_get_playback_key_pair_response(&wire::GetPlaybackKeyPairResponse {
                    key_pair: Some(wire::PlaybackKeyPair {
                        arn: Some(kp.arn.clone()),
                        name: Some(kp.name.clone()),
                        fingerprint: Some(kp.fingerprint.clone()),
                        tags: if kp.tags.is_empty() {
                            None
                        } else {
                            Some(kp.tags.clone())
                        },
                    }),
                })
            }
            Err(e) => ivs_error_response(&e),
        }
    }

    async fn handle_delete_playback_key_pair(
        &self,
        state: &Arc<tokio::sync::RwLock<IvsState>>,
        body: &Value,
    ) -> MockResponse {
        let arn = match body.get("arn").and_then(|v| v.as_str()) {
            Some(a) if !a.is_empty() => a,
            _ => return rest_json_error(400, "ValidationException", "Missing or empty 'arn'"),
        };
        let mut state = state.write().await;
        match state.delete_playback_key_pair(arn) {
            Ok(()) => wire::serialize_delete_playback_key_pair_response(
                &wire::DeletePlaybackKeyPairResponse {},
            ),
            Err(e) => ivs_error_response(&e),
        }
    }

    async fn handle_list_playback_key_pairs(
        &self,
        state: &Arc<tokio::sync::RwLock<IvsState>>,
    ) -> MockResponse {
        let state = state.read().await;
        let pairs = state.list_playback_key_pairs();
        wire::serialize_list_playback_key_pairs_response(&wire::ListPlaybackKeyPairsResponse {
            key_pairs: Some(
                pairs
                    .iter()
                    .map(|kp| wire::PlaybackKeyPairSummary {
                        arn: Some(kp.arn.clone()),
                        name: Some(kp.name.clone()),
                        tags: if kp.tags.is_empty() {
                            None
                        } else {
                            Some(kp.tags.clone())
                        },
                    })
                    .collect(),
            ),
            ..Default::default()
        })
    }

    // -------------------------------------------------------------------------
    // Playback restriction policy handlers
    // -------------------------------------------------------------------------

    async fn handle_create_playback_restriction_policy(
        &self,
        state: &Arc<tokio::sync::RwLock<IvsState>>,
        body: &Value,
        region: &str,
        account_id: &str,
    ) -> MockResponse {
        let name = body.get("name").and_then(|v| v.as_str());
        let allowed_countries = extract_string_array(body, "allowedCountries");
        let allowed_origins = extract_string_array(body, "allowedOrigins");
        let enable_strict = body
            .get("enableStrictOriginEnforcement")
            .and_then(|v| v.as_bool())
            .unwrap_or(false);
        let tags = extract_tags(body);

        let mut state = state.write().await;
        match state.create_playback_restriction_policy(
            name,
            allowed_countries,
            allowed_origins,
            enable_strict,
            tags,
            region,
            account_id,
        ) {
            Ok(policy) => wire::serialize_create_playback_restriction_policy_response(
                &wire::CreatePlaybackRestrictionPolicyResponse {
                    playback_restriction_policy: Some(playback_restriction_policy_to_wire(policy)),
                },
            ),
            Err(e) => ivs_error_response(&e),
        }
    }

    async fn handle_get_playback_restriction_policy(
        &self,
        state: &Arc<tokio::sync::RwLock<IvsState>>,
        body: &Value,
    ) -> MockResponse {
        let arn = match body.get("arn").and_then(|v| v.as_str()) {
            Some(a) if !a.is_empty() => a,
            _ => return rest_json_error(400, "ValidationException", "Missing or empty 'arn'"),
        };
        let state = state.read().await;
        match state.get_playback_restriction_policy(arn) {
            Ok(policy) => wire::serialize_get_playback_restriction_policy_response(
                &wire::GetPlaybackRestrictionPolicyResponse {
                    playback_restriction_policy: Some(playback_restriction_policy_to_wire(policy)),
                },
            ),
            Err(e) => ivs_error_response(&e),
        }
    }

    async fn handle_delete_playback_restriction_policy(
        &self,
        state: &Arc<tokio::sync::RwLock<IvsState>>,
        body: &Value,
    ) -> MockResponse {
        let arn = match body.get("arn").and_then(|v| v.as_str()) {
            Some(a) if !a.is_empty() => a,
            _ => return rest_json_error(400, "ValidationException", "Missing or empty 'arn'"),
        };
        let mut state = state.write().await;
        match state.delete_playback_restriction_policy(arn) {
            Ok(()) => wire::serialize_delete_playback_restriction_policy_response(),
            Err(e) => ivs_error_response(&e),
        }
    }

    async fn handle_list_playback_restriction_policies(
        &self,
        state: &Arc<tokio::sync::RwLock<IvsState>>,
    ) -> MockResponse {
        let state = state.read().await;
        let policies = state.list_playback_restriction_policies();
        wire::serialize_list_playback_restriction_policies_response(
            &wire::ListPlaybackRestrictionPoliciesResponse {
                playback_restriction_policies: Some(
                    policies
                        .iter()
                        .map(|p| wire::PlaybackRestrictionPolicySummary {
                            arn: Some(p.arn.clone()),
                            name: Some(p.name.clone()),
                            allowed_countries: Some(p.allowed_countries.clone()),
                            allowed_origins: Some(p.allowed_origins.clone()),
                            enable_strict_origin_enforcement: Some(
                                p.enable_strict_origin_enforcement,
                            ),
                            tags: if p.tags.is_empty() {
                                None
                            } else {
                                Some(p.tags.clone())
                            },
                        })
                        .collect(),
                ),
                ..Default::default()
            },
        )
    }

    async fn handle_update_playback_restriction_policy(
        &self,
        state: &Arc<tokio::sync::RwLock<IvsState>>,
        body: &Value,
    ) -> MockResponse {
        let arn = match body.get("arn").and_then(|v| v.as_str()) {
            Some(a) if !a.is_empty() => a,
            _ => return rest_json_error(400, "ValidationException", "Missing or empty 'arn'"),
        };
        let name = body.get("name").and_then(|v| v.as_str());
        let allowed_countries =
            body.get("allowedCountries")
                .and_then(|v| v.as_array())
                .map(|arr| {
                    arr.iter()
                        .filter_map(|v| v.as_str().map(|s| s.to_string()))
                        .collect()
                });
        let allowed_origins = body
            .get("allowedOrigins")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(|s| s.to_string()))
                    .collect()
            });
        let enable_strict = body
            .get("enableStrictOriginEnforcement")
            .and_then(|v| v.as_bool());

        let mut state = state.write().await;
        match state.update_playback_restriction_policy(
            arn,
            name,
            allowed_countries,
            allowed_origins,
            enable_strict,
        ) {
            Ok(policy) => wire::serialize_update_playback_restriction_policy_response(
                &wire::UpdatePlaybackRestrictionPolicyResponse {
                    playback_restriction_policy: Some(playback_restriction_policy_to_wire(policy)),
                },
            ),
            Err(e) => ivs_error_response(&e),
        }
    }

    // -------------------------------------------------------------------------
    // Stream handlers (stateless stubs — IVS streams are ephemeral)
    // -------------------------------------------------------------------------

    // STUB[no-telemetry]: Stream state is driven by real IVS ingest telemetry; always returns OFFLINE.
    async fn handle_get_stream(
        &self,
        state: &Arc<tokio::sync::RwLock<IvsState>>,
        body: &Value,
    ) -> MockResponse {
        let channel_arn = match body.get("channelArn").and_then(|v| v.as_str()) {
            Some(a) if !a.is_empty() => a,
            _ => {
                return rest_json_error(
                    400,
                    "ValidationException",
                    "Missing or empty 'channelArn'",
                );
            }
        };
        let state = state.read().await;
        // Channel must exist
        if state.get_channel(channel_arn).is_err() {
            return ivs_error_response(&IvsError::ChannelNotFound {
                arn: channel_arn.to_string(),
            });
        }
        // Return an empty stream (not currently live)
        wire::serialize_get_stream_response(&wire::GetStreamResponse {
            stream: Some(wire::Stream {
                channel_arn: Some(channel_arn.to_string()),
                state: Some("OFFLINE".to_string()),
                ..Default::default()
            }),
        })
    }

    // STUB[no-telemetry]: Stream session history requires real IVS telemetry; always returns None.
    async fn handle_get_stream_session(
        &self,
        state: &Arc<tokio::sync::RwLock<IvsState>>,
        body: &Value,
    ) -> MockResponse {
        let channel_arn = match body.get("channelArn").and_then(|v| v.as_str()) {
            Some(a) if !a.is_empty() => a,
            _ => {
                return rest_json_error(
                    400,
                    "ValidationException",
                    "Missing or empty 'channelArn'",
                );
            }
        };
        let state = state.read().await;
        if state.get_channel(channel_arn).is_err() {
            return ivs_error_response(&IvsError::ChannelNotFound {
                arn: channel_arn.to_string(),
            });
        }
        wire::serialize_get_stream_session_response(&wire::GetStreamSessionResponse {
            stream_session: None,
        })
    }

    // STUB[no-telemetry]: Live stream list is driven by real IVS ingest telemetry; always returns empty.
    async fn handle_list_streams(
        &self,
        _state: &Arc<tokio::sync::RwLock<IvsState>>,
    ) -> MockResponse {
        wire::serialize_list_streams_response(&wire::ListStreamsResponse {
            streams: Some(vec![]),
            ..Default::default()
        })
    }

    // STUB[no-telemetry]: Stream session history requires real IVS telemetry; always returns empty.
    async fn handle_list_stream_sessions(
        &self,
        state: &Arc<tokio::sync::RwLock<IvsState>>,
        body: &Value,
    ) -> MockResponse {
        let channel_arn = match body.get("channelArn").and_then(|v| v.as_str()) {
            Some(a) if !a.is_empty() => a,
            _ => {
                return rest_json_error(
                    400,
                    "ValidationException",
                    "Missing or empty 'channelArn'",
                );
            }
        };
        let state = state.read().await;
        if state.get_channel(channel_arn).is_err() {
            return ivs_error_response(&IvsError::ChannelNotFound {
                arn: channel_arn.to_string(),
            });
        }
        wire::serialize_list_stream_sessions_response(&wire::ListStreamSessionsResponse {
            stream_sessions: Some(vec![]),
            ..Default::default()
        })
    }

    // STUB[no-telemetry]: Stopping a stream requires a live IVS ingest session; always succeeds as a no-op.
    async fn handle_stop_stream(
        &self,
        state: &Arc<tokio::sync::RwLock<IvsState>>,
        body: &Value,
    ) -> MockResponse {
        let channel_arn = match body.get("channelArn").and_then(|v| v.as_str()) {
            Some(a) if !a.is_empty() => a,
            _ => {
                return rest_json_error(
                    400,
                    "ValidationException",
                    "Missing or empty 'channelArn'",
                );
            }
        };
        let state = state.read().await;
        if state.get_channel(channel_arn).is_err() {
            return ivs_error_response(&IvsError::ChannelNotFound {
                arn: channel_arn.to_string(),
            });
        }
        wire::serialize_stop_stream_response(&wire::StopStreamResponse {})
    }

    // -------------------------------------------------------------------------
    // Tag handlers
    // -------------------------------------------------------------------------

    async fn handle_list_tags_for_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<IvsState>>,
        resource_arn: &str,
    ) -> MockResponse {
        let state = state.read().await;
        let tags = state.get_tags_for_resource(resource_arn);
        wire::serialize_list_tags_for_resource_response(&wire::ListTagsForResourceResponse {
            tags: Some(tags),
        })
    }

    async fn handle_tag_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<IvsState>>,
        resource_arn: &str,
        body: &Value,
    ) -> MockResponse {
        let tags = extract_tags(body);
        let mut state = state.write().await;
        match state.tag_resource(resource_arn, tags) {
            Ok(()) => wire::serialize_tag_resource_response(&wire::TagResourceResponse {}),
            Err(e) => ivs_error_response(&e),
        }
    }

    async fn handle_untag_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<IvsState>>,
        resource_arn: &str,
        tag_keys: &[String],
    ) -> MockResponse {
        let mut state = state.write().await;
        match state.untag_resource(resource_arn, tag_keys) {
            Ok(()) => wire::serialize_untag_resource_response(&wire::UntagResourceResponse {}),
            Err(e) => ivs_error_response(&e),
        }
    }

    // -------------------------------------------------------------------------
    // Other handlers
    // -------------------------------------------------------------------------

    async fn handle_put_metadata(
        &self,
        state: &Arc<tokio::sync::RwLock<IvsState>>,
        body: &Value,
    ) -> MockResponse {
        let channel_arn = match body.get("channelArn").and_then(|v| v.as_str()) {
            Some(a) if !a.is_empty() => a,
            _ => {
                return rest_json_error(
                    400,
                    "ValidationException",
                    "Missing or empty 'channelArn'",
                );
            }
        };
        let state = state.read().await;
        if state.get_channel(channel_arn).is_err() {
            return ivs_error_response(&IvsError::ChannelNotFound {
                arn: channel_arn.to_string(),
            });
        }
        wire::serialize_put_metadata_response()
    }

    async fn handle_batch_start_viewer_session_revocation(&self, body: &Value) -> MockResponse {
        let viewer_sessions = body
            .get("viewerSessions")
            .and_then(|v| v.as_array())
            .cloned()
            .unwrap_or_default();
        if viewer_sessions.is_empty() {
            return ivs_validation_error("viewerSessions must not be empty");
        }
        wire::serialize_batch_start_viewer_session_revocation_response(
            &wire::BatchStartViewerSessionRevocationResponse {
                errors: Some(vec![]),
            },
        )
    }

    async fn handle_start_viewer_session_revocation(&self, body: &Value) -> MockResponse {
        let channel_arn = body.get("channelArn").and_then(|v| v.as_str());
        if channel_arn.is_none() || channel_arn == Some("") {
            return ivs_validation_error("channelArn is required");
        }
        wire::serialize_start_viewer_session_revocation_response(
            &wire::StartViewerSessionRevocationResponse {},
        )
    }
}

// -------------------------------------------------------------------------
// Wire conversion helpers
// -------------------------------------------------------------------------

fn recording_config_to_wire(
    rc: &crate::types::RecordingConfiguration,
) -> wire::RecordingConfiguration {
    wire::RecordingConfiguration {
        arn: Some(rc.arn.clone()),
        name: Some(rc.name.clone()),
        destination_configuration: Some(wire::DestinationConfiguration {
            s3: rc
                .destination_configuration
                .s3_bucket_name
                .as_ref()
                .map(|bucket| wire::S3DestinationConfiguration {
                    bucket_name: bucket.clone(),
                }),
        }),
        state: Some(rc.state.clone()),
        tags: if rc.tags.is_empty() {
            None
        } else {
            Some(rc.tags.clone())
        },
        ..Default::default()
    }
}

fn playback_restriction_policy_to_wire(
    p: &crate::types::PlaybackRestrictionPolicy,
) -> wire::PlaybackRestrictionPolicy {
    wire::PlaybackRestrictionPolicy {
        arn: Some(p.arn.clone()),
        name: Some(p.name.clone()),
        allowed_countries: Some(p.allowed_countries.clone()),
        allowed_origins: Some(p.allowed_origins.clone()),
        enable_strict_origin_enforcement: Some(p.enable_strict_origin_enforcement),
        tags: if p.tags.is_empty() {
            None
        } else {
            Some(p.tags.clone())
        },
    }
}

fn extract_tags(body: &Value) -> HashMap<String, String> {
    body.get("tags")
        .and_then(|v| v.as_object())
        .map(|obj| {
            obj.iter()
                .filter_map(|(k, v)| v.as_str().map(|s| (k.clone(), s.to_string())))
                .collect()
        })
        .unwrap_or_default()
}

fn extract_string_array(body: &Value, key: &str) -> Vec<String> {
    body.get(key)
        .and_then(|v| v.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|v| v.as_str().map(|s| s.to_string()))
                .collect()
        })
        .unwrap_or_default()
}

/// Decode a percent-encoded ARN from the URL path.
fn decode_resource_arn(encoded: &str) -> String {
    // Simple percent-decode for %3A (colon) and %2F (slash) which appear in ARNs
    encoded
        .replace("%3A", ":")
        .replace("%2F", "/")
        .replace("%3a", ":")
        .replace("%2f", "/")
}

/// Extract `tagKeys` query params from a URI like `/tags/arn?tagKeys=key1&tagKeys=key2`.
fn extract_tag_keys_from_uri(uri: &str) -> Vec<String> {
    if let Some(query_start) = uri.find('?') {
        let query = &uri[query_start + 1..];
        query
            .split('&')
            .filter_map(|param| {
                let (key, value) = param.split_once('=')?;
                if key == "tagKeys" {
                    Some(value.replace("%3A", ":").replace("%2F", "/"))
                } else {
                    None
                }
            })
            .collect()
    } else {
        vec![]
    }
}

fn ivs_validation_error(message: &str) -> MockResponse {
    let body = json!({
        "Type": "User",
        "Message": message,
    });
    let mut resp = MockResponse::rest_json(400, body.to_string());
    resp.headers
        .insert(X_AMZN_ERRORTYPE, "ValidationException".parse().unwrap());
    resp
}

fn ivs_error_response(err: &IvsError) -> MockResponse {
    let (status, error_type) = match err {
        IvsError::ChannelConflict { .. } => (409, "ConflictException"),
        IvsError::ChannelNotFound { .. } => (404, "ResourceNotFoundException"),
        IvsError::StreamKeyNotFound { .. } => (404, "ResourceNotFoundException"),
        IvsError::RecordingConfigurationNotFound { .. } => (404, "ResourceNotFoundException"),
        IvsError::PlaybackKeyPairNotFound { .. } => (404, "ResourceNotFoundException"),
        IvsError::PlaybackRestrictionPolicyNotFound { .. } => (404, "ResourceNotFoundException"),
        IvsError::ResourceNotFound { .. } => (404, "ResourceNotFoundException"),
    };
    let body = json!({
        "Type": "User",
        "Message": err.to_string(),
    });
    let mut resp = MockResponse::rest_json(status, body.to_string());
    resp.headers
        .insert(X_AMZN_ERRORTYPE, error_type.parse().unwrap());
    resp
}
