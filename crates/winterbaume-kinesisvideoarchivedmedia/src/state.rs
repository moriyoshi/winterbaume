use std::collections::HashMap;

use chrono::Utc;
use thiserror::Error;

use crate::types::*;

#[derive(Debug, Default)]
pub struct KinesisVideoArchivedMediaState {
    /// Map from stream name to stream data
    pub streams: HashMap<String, StreamData>,
    /// Map from stream ARN to stream name
    pub arn_to_name: HashMap<String, String>,
    /// Map from session URL to streaming session
    pub sessions: HashMap<String, StreamingSession>,
}

#[derive(Debug, Error)]
pub enum KinesisVideoArchivedMediaError {
    #[error("Either StreamName or StreamARN must be provided")]
    MissingStreamIdentifier,

    #[error("Fragments list must not be empty")]
    EmptyFragmentList,

    #[error("Fragment number {fragment_number} not found")]
    FragmentNotFound { fragment_number: String },
}

impl KinesisVideoArchivedMediaState {
    pub fn ensure_stream(
        &mut self,
        stream_name: Option<&str>,
        stream_arn: Option<&str>,
    ) -> Result<&StreamData, KinesisVideoArchivedMediaError> {
        let name = match (stream_name, stream_arn) {
            (Some(n), _) => {
                if !self.streams.contains_key(n) {
                    // Auto-create stream for mock purposes
                    let arn = format!(
                        "arn:aws:kinesisvideo:us-east-1:123456789012:stream/{}/0000000000000",
                        n
                    );
                    let stream = StreamData {
                        stream_name: n.to_string(),
                        stream_arn: arn.clone(),
                        fragments: generate_mock_fragments(5),
                    };
                    self.arn_to_name.insert(arn, n.to_string());
                    self.streams.insert(n.to_string(), stream);
                }
                n.to_string()
            }
            (None, Some(a)) => {
                if let Some(n) = self.arn_to_name.get(a) {
                    n.clone()
                } else {
                    // Auto-create stream from ARN
                    let name = a.split('/').nth(1).unwrap_or("unknown-stream").to_string();
                    let stream = StreamData {
                        stream_name: name.clone(),
                        stream_arn: a.to_string(),
                        fragments: generate_mock_fragments(5),
                    };
                    self.arn_to_name.insert(a.to_string(), name.clone());
                    self.streams.insert(name.clone(), stream);
                    name
                }
            }
            (None, None) => {
                return Err(KinesisVideoArchivedMediaError::MissingStreamIdentifier);
            }
        };
        Ok(self.streams.get(&name).unwrap())
    }

    pub fn ensure_stream_mut(
        &mut self,
        stream_name: Option<&str>,
        stream_arn: Option<&str>,
    ) -> Result<&mut StreamData, KinesisVideoArchivedMediaError> {
        // First call ensure_stream to auto-create if needed
        let name = self
            .ensure_stream(stream_name, stream_arn)?
            .stream_name
            .clone();
        Ok(self.streams.get_mut(&name).unwrap())
    }

    pub fn list_fragments(
        &mut self,
        stream_name: Option<&str>,
        stream_arn: Option<&str>,
    ) -> Result<Vec<Fragment>, KinesisVideoArchivedMediaError> {
        let stream = self.ensure_stream(stream_name, stream_arn)?;
        Ok(stream.fragments.clone())
    }

    pub fn get_media_for_fragment_list(
        &mut self,
        stream_name: Option<&str>,
        stream_arn: Option<&str>,
        fragment_numbers: &[String],
    ) -> Result<(String, Vec<u8>), KinesisVideoArchivedMediaError> {
        let stream = self.ensure_stream(stream_name, stream_arn)?;

        if fragment_numbers.is_empty() {
            return Err(KinesisVideoArchivedMediaError::EmptyFragmentList);
        }

        // Check that requested fragment numbers exist
        let existing: std::collections::HashSet<&str> = stream
            .fragments
            .iter()
            .map(|f| f.fragment_number.as_str())
            .collect();

        for fnum in fragment_numbers {
            if !existing.contains(fnum.as_str()) {
                return Err(KinesisVideoArchivedMediaError::FragmentNotFound {
                    fragment_number: fnum.clone(),
                });
            }
        }

        // Return mock MKV data
        let mock_payload = b"mock-mkv-data".to_vec();
        Ok(("video/webm".to_string(), mock_payload))
    }

    pub fn get_hls_streaming_session_url(
        &mut self,
        stream_name: Option<&str>,
        stream_arn: Option<&str>,
    ) -> Result<String, KinesisVideoArchivedMediaError> {
        let stream = self.ensure_stream(stream_name, stream_arn)?;
        let session_id = uuid::Uuid::new_v4().to_string();
        let url = format!(
            "https://v-{}.kinesisvideo.us-east-1.amazonaws.com/hls/v1/getHLSMasterPlaylist.m3u8?SessionToken={}",
            stream.stream_name, session_id,
        );

        self.sessions.insert(
            session_id,
            StreamingSession {
                session_url: url.clone(),
                session_type: "HLS".to_string(),
                created_at: Utc::now(),
            },
        );

        Ok(url)
    }

    pub fn get_dash_streaming_session_url(
        &mut self,
        stream_name: Option<&str>,
        stream_arn: Option<&str>,
    ) -> Result<String, KinesisVideoArchivedMediaError> {
        let stream = self.ensure_stream(stream_name, stream_arn)?;
        let session_id = uuid::Uuid::new_v4().to_string();
        let url = format!(
            "https://v-{}.kinesisvideo.us-east-1.amazonaws.com/dash/v1/getDASHManifest.mpd?SessionToken={}",
            stream.stream_name, session_id,
        );

        self.sessions.insert(
            session_id,
            StreamingSession {
                session_url: url.clone(),
                session_type: "DASH".to_string(),
                created_at: Utc::now(),
            },
        );

        Ok(url)
    }

    pub fn get_clip(
        &mut self,
        stream_name: Option<&str>,
        stream_arn: Option<&str>,
    ) -> Result<ClipResult, KinesisVideoArchivedMediaError> {
        let _stream = self.ensure_stream(stream_name, stream_arn)?;
        Ok(ClipResult {
            content_type: "video/mp4".to_string(),
            payload: b"mock-mp4-clip-data".to_vec(),
        })
    }

    pub fn get_images(
        &mut self,
        stream_name: Option<&str>,
        stream_arn: Option<&str>,
        _format: Option<&str>,
        num_images: usize,
    ) -> Result<Vec<ImageResult>, KinesisVideoArchivedMediaError> {
        let _stream = self.ensure_stream(stream_name, stream_arn)?;

        let count = if num_images == 0 { 3 } else { num_images };
        let mut images = Vec::with_capacity(count);
        for _ in 0..count {
            images.push(ImageResult {
                image_content: base64::Engine::encode(
                    &base64::engine::general_purpose::STANDARD,
                    b"mock-image-data",
                ),
                timestamp: Utc::now(),
                error: None,
            });
        }

        Ok(images)
    }
}

fn generate_mock_fragments(count: usize) -> Vec<Fragment> {
    let mut fragments = Vec::with_capacity(count);
    let now = Utc::now();
    for i in 0..count {
        fragments.push(Fragment {
            fragment_number: format!("{}", 91343852333181432 + i as u64),
            fragment_size_in_bytes: 1024 * (i as i64 + 1),
            producer_timestamp: now - chrono::Duration::seconds((count - i) as i64 * 6),
            server_timestamp: now - chrono::Duration::seconds((count - i) as i64 * 6),
            fragment_length_in_milliseconds: 6000,
        });
    }
    fragments
}
