use crate::harness::*;

// MediaLive terraform resources tested here:
//   aws_medialive_input
//   aws_medialive_channel (combined with aws_medialive_input)
//
// Background: MediaLive channels reference one or more inputs through
// `input_attachments`, so the basic channel scenario provisions both an
// input and a channel together. The standalone input test covers the
// simpler create/describe path.

// ---------------------------------------------------------------------------
// Input tests
// ---------------------------------------------------------------------------

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_medialive_input_basic() {
    let result = batch_apply(
        r#"
resource "aws_medialive_input" "medialive_input_basic" {
  name = "medialive-input-basic"
  type = "UDP_PUSH"

  tags = {
    Name = "medialive-input-basic"
  }
}
"#,
    )
    .await;
    assert!(result.success, "terraform apply failed:\n{}", result.stderr);
    assert!(result.state.contains("medialive-input-basic"));
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_medialive_input_rtp_push() {
    let result = batch_apply(
        r#"
resource "aws_medialive_input" "medialive_input_rtp" {
  name = "medialive-input-rtp"
  type = "RTP_PUSH"

  tags = {
    Environment = "test"
    Stream      = "medialive-input-rtp"
  }
}
"#,
    )
    .await;
    assert!(result.success, "terraform apply failed:\n{}", result.stderr);
    assert!(result.state.contains("medialive-input-rtp"));
}

// ---------------------------------------------------------------------------
// Channel tests (combined channel + input, since channels require an
// attached input).
// ---------------------------------------------------------------------------

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore]
async fn test_medialive_channel_basic() {
    let result = batch_apply(
        r#"
resource "aws_medialive_input" "medialive_channel_basic_input" {
  name = "medialive-channel-basic-input"
  type = "UDP_PUSH"

  tags = {
    Name = "medialive-channel-basic-input"
  }
}

resource "aws_medialive_channel" "medialive_channel_basic" {
  name          = "medialive-channel-basic"
  channel_class = "SINGLE_PIPELINE"
  role_arn      = "arn:aws:iam::123456789012:role/MediaLiveAccessRole"

  input_specification {
    codec            = "AVC"
    input_resolution = "HD"
    maximum_bitrate  = "MAX_20_MBPS"
  }

  input_attachments {
    input_attachment_name = "primary"
    input_id              = aws_medialive_input.medialive_channel_basic_input.id
  }

  destinations {
    id = "destination1"

    settings {
      url = "rtmp://example.com/live/stream"
    }
  }

  encoder_settings {
    timecode_config {
      source = "EMBEDDED"
    }

    audio_descriptions {
      audio_selector_name = "default"
      name                = "audio_1"
    }

    video_descriptions {
      name = "video_1"
    }

    output_groups {
      output_group_settings {
        archive_group_settings {
          destination {
            destination_ref_id = "destination1"
          }
        }
      }

      outputs {
        output_name             = "output_1"
        video_description_name  = "video_1"
        audio_description_names = ["audio_1"]

        output_settings {
          archive_output_settings {
            name_modifier = "_1"
            extension     = "m2ts"

            container_settings {
              m2ts_settings {
                audio_buffer_model = "ATSC"
              }
            }
          }
        }
      }
    }
  }

  tags = {
    Name = "medialive-channel-basic"
  }
}
"#,
    )
    .await;
    assert!(result.success, "terraform apply failed:\n{}", result.stderr);
    assert!(result.state.contains("medialive-channel-basic"));
}
