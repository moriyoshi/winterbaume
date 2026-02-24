use std::collections::HashMap;

use chrono::Utc;
use thiserror::Error;

use crate::types::*;

#[derive(Debug, Default)]
pub struct PollyState {
    pub lexicons: HashMap<String, Lexicon>,
    pub synthesis_tasks: HashMap<String, SpeechSynthesisTask>,
}

#[derive(Debug, Error)]
pub enum PollyError {
    #[error("Unknown language code: {language_code}")]
    UnknownLanguageCode { language_code: String },

    #[error("Lexicon name must not be empty")]
    LexiconNameEmpty,

    #[error("Lexicon name must be at most 20 characters long")]
    LexiconNameTooLong,

    #[error("Lexicon name must contain only alphanumeric characters")]
    LexiconNameInvalidChars,

    #[error("Lexicon {name} not found")]
    LexiconNotFound { name: String },

    #[error("SpeechSynthesisTask {task_id} not found")]
    SynthesisTaskNotFound { task_id: String },

    #[error("Unknown voice id: {voice_id}")]
    UnknownVoiceId { voice_id: String },

    #[error("Invalid output format: {format}")]
    InvalidOutputFormat { format: String },

    #[error("Invalid text type: {text_type}")]
    InvalidTextType { text_type: String },

    #[error("The input text is too long")]
    TextLengthExceeded,

    #[error("Invalid sample rate: {sample_rate}")]
    InvalidSampleRate { sample_rate: String },

    #[error("Speech marks are only supported for JSON output format")]
    MarksNotSupportedForFormat,
}

impl PollyState {
    pub fn describe_voices(&self, language_code: Option<&str>) -> Result<Vec<Voice>, PollyError> {
        // Validate language_code if provided
        if let Some(lc) = language_code {
            let valid = VOICES
                .iter()
                .any(|(_, _, _, lang_code, _, _)| *lang_code == lc);
            if !valid {
                return Err(PollyError::UnknownLanguageCode {
                    language_code: lc.to_string(),
                });
            }
        }
        Ok(VOICES
            .iter()
            .filter(|(_, _, _, lang_code, _, _)| {
                language_code.is_none() || Some(*lang_code) == language_code
            })
            .map(|(id, name, gender, lang_code, lang_name, engines)| Voice {
                id: id.to_string(),
                name: name.to_string(),
                gender: gender.to_string(),
                language_code: lang_code.to_string(),
                language_name: lang_name.to_string(),
                supported_engines: engines.iter().map(|e| e.to_string()).collect(),
            })
            .collect())
    }

    pub fn put_lexicon(&mut self, name: &str, content: &str) -> Result<(), PollyError> {
        if name.is_empty() {
            return Err(PollyError::LexiconNameEmpty);
        }
        // Lexicon name pattern: [0-9A-Za-z]{1,20} per AWS docs
        if name.len() > 20 {
            return Err(PollyError::LexiconNameTooLong);
        }
        // Lexicon name must contain only alphanumeric characters
        if !name.chars().all(|c| c.is_ascii_alphanumeric()) {
            return Err(PollyError::LexiconNameInvalidChars);
        }

        // Extract language from PLS content, default to en-US
        let language_code =
            extract_language_from_pls(content).unwrap_or_else(|| "en-US".to_string());

        let lexicon = Lexicon {
            name: name.to_string(),
            content: content.to_string(),
            language_code,
            lexemes_count: count_lexemes(content),
            size: content.len() as i32,
            last_modified: Utc::now(),
        };

        self.lexicons.insert(name.to_string(), lexicon);
        Ok(())
    }

    pub fn get_lexicon(&self, name: &str) -> Result<&Lexicon, PollyError> {
        self.lexicons
            .get(name)
            .ok_or_else(|| PollyError::LexiconNotFound {
                name: name.to_string(),
            })
    }

    pub fn delete_lexicon(&mut self, name: &str) -> Result<(), PollyError> {
        if self.lexicons.remove(name).is_none() {
            return Err(PollyError::LexiconNotFound {
                name: name.to_string(),
            });
        }
        Ok(())
    }

    pub fn list_lexicons(&self) -> Vec<&Lexicon> {
        self.lexicons.values().collect()
    }

    pub fn start_speech_synthesis_task(
        &mut self,
        voice_id: &str,
        text: &str,
        output_format: &str,
        output_s3_bucket_name: &str,
        output_s3_key_prefix: &str,
        engine: &str,
    ) -> &SpeechSynthesisTask {
        let task_id = uuid::Uuid::new_v4().to_string();
        let output_uri =
            format!("s3://{output_s3_bucket_name}/{output_s3_key_prefix}{task_id}.{output_format}");
        let task = SpeechSynthesisTask {
            task_id: task_id.clone(),
            task_status: "completed".to_string(),
            output_uri,
            voice_id: voice_id.to_string(),
            text: text.to_string(),
            creation_time: Utc::now(),
            output_format: output_format.to_string(),
            engine: engine.to_string(),
        };
        self.synthesis_tasks.insert(task_id.clone(), task);
        self.synthesis_tasks.get(&task_id).unwrap()
    }

    pub fn get_speech_synthesis_task(
        &self,
        task_id: &str,
    ) -> Result<&SpeechSynthesisTask, PollyError> {
        self.synthesis_tasks
            .get(task_id)
            .ok_or_else(|| PollyError::SynthesisTaskNotFound {
                task_id: task_id.to_string(),
            })
    }

    pub fn list_speech_synthesis_tasks(&self) -> Vec<&SpeechSynthesisTask> {
        self.synthesis_tasks.values().collect()
    }
}

fn extract_language_from_pls(content: &str) -> Option<String> {
    // Simple extraction: look for xml:lang="..." in PLS content
    if let Some(idx) = content.find("xml:lang=\"") {
        let start = idx + "xml:lang=\"".len();
        if let Some(end) = content[start..].find('"') {
            return Some(content[start..start + end].to_string());
        }
    }
    None
}

fn count_lexemes(content: &str) -> i32 {
    // Count <lexeme> tags as a rough approximation
    content.matches("<lexeme").count() as i32
}
