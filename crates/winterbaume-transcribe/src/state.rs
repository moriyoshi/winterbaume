use std::collections::HashMap;

use thiserror::Error;

use crate::types::*;

#[derive(Debug, Default)]
pub struct TranscribeState {
    pub vocabularies: HashMap<String, Vocabulary>,
    pub transcription_jobs: HashMap<String, TranscriptionJobData>,
    pub medical_transcription_jobs: HashMap<String, MedicalTranscriptionJobData>,
    pub medical_vocabularies: HashMap<String, MedicalVocabularyData>,
}

#[derive(Debug, Error)]
pub enum TranscribeError {
    #[error("The requested vocabulary name already exists. Use a different vocabulary name.")]
    VocabularyAlreadyExists,

    #[error(
        "The requested vocabulary couldn't be found. Check the vocabulary name and try your request again."
    )]
    VocabularyNotFound,

    #[error("The requested job name already exists. Use a different job name.")]
    JobAlreadyExists,

    #[error("The requested job couldn't be found. Check the job name and try your request again.")]
    JobNotFound,

    #[error("Missing X-Amz-Target header")]
    MissingAction,

    #[error("Invalid JSON body")]
    InvalidJson,

    #[error("{field} is required")]
    MissingField { field: String },

    #[error("Unknown operation {action}")]
    InvalidAction { action: String },
}

impl TranscribeState {
    pub fn create_vocabulary(
        &mut self,
        name: &str,
        language_code: &str,
        phrases: Option<Vec<String>>,
        vocabulary_file_uri: Option<String>,
        now: f64,
    ) -> Result<&Vocabulary, TranscribeError> {
        if self.vocabularies.contains_key(name) {
            return Err(TranscribeError::VocabularyAlreadyExists);
        }

        let vocab = Vocabulary {
            vocabulary_name: name.to_string(),
            language_code: language_code.to_string(),
            vocabulary_state: "READY".to_string(),
            last_modified_time: now,
            phrases,
            vocabulary_file_uri,
            failure_reason: None,
            download_uri: None,
        };

        self.vocabularies.insert(name.to_string(), vocab);
        Ok(self.vocabularies.get(name).unwrap())
    }

    pub fn get_vocabulary(&self, name: &str) -> Result<&Vocabulary, TranscribeError> {
        self.vocabularies
            .get(name)
            .ok_or(TranscribeError::VocabularyNotFound)
    }

    pub fn delete_vocabulary(&mut self, name: &str) -> Result<(), TranscribeError> {
        if self.vocabularies.remove(name).is_none() {
            return Err(TranscribeError::VocabularyNotFound);
        }
        Ok(())
    }

    pub fn list_vocabularies(
        &self,
        state_equals: Option<&str>,
        name_contains: Option<&str>,
    ) -> Vec<&Vocabulary> {
        self.vocabularies
            .values()
            .filter(|v| {
                if let Some(state) = state_equals
                    && v.vocabulary_state != state
                {
                    return false;
                }
                if let Some(contains) = name_contains
                    && !v
                        .vocabulary_name
                        .to_lowercase()
                        .contains(&contains.to_lowercase())
                {
                    return false;
                }
                true
            })
            .collect()
    }

    // --- Transcription Jobs ---

    pub fn start_transcription_job(
        &mut self,
        name: &str,
        language_code: &str,
        media_uri: &str,
        media_format: Option<String>,
        media_sample_rate_hertz: Option<i32>,
        output_bucket_name: Option<String>,
        now: f64,
    ) -> Result<&TranscriptionJobData, TranscribeError> {
        if self.transcription_jobs.contains_key(name) {
            return Err(TranscribeError::JobAlreadyExists);
        }

        let job = TranscriptionJobData {
            transcription_job_name: name.to_string(),
            transcription_job_status: "COMPLETED".to_string(),
            language_code: language_code.to_string(),
            media_uri: media_uri.to_string(),
            media_format,
            media_sample_rate_hertz,
            output_bucket_name,
            creation_time: now,
            start_time: Some(now),
            completion_time: Some(now),
            failure_reason: None,
            transcript_file_uri: Some(format!("https://s3.amazonaws.com/output/{name}.json")),
        };

        self.transcription_jobs.insert(name.to_string(), job);
        Ok(self.transcription_jobs.get(name).unwrap())
    }

    pub fn get_transcription_job(
        &self,
        name: &str,
    ) -> Result<&TranscriptionJobData, TranscribeError> {
        self.transcription_jobs
            .get(name)
            .ok_or(TranscribeError::JobNotFound)
    }

    pub fn delete_transcription_job(&mut self, name: &str) -> Result<(), TranscribeError> {
        if self.transcription_jobs.remove(name).is_none() {
            return Err(TranscribeError::JobNotFound);
        }
        Ok(())
    }

    pub fn list_transcription_jobs(
        &self,
        status_equals: Option<&str>,
        job_name_contains: Option<&str>,
    ) -> Vec<&TranscriptionJobData> {
        self.transcription_jobs
            .values()
            .filter(|j| {
                if let Some(status) = status_equals
                    && j.transcription_job_status != status
                {
                    return false;
                }
                if let Some(contains) = job_name_contains
                    && !j
                        .transcription_job_name
                        .to_lowercase()
                        .contains(&contains.to_lowercase())
                {
                    return false;
                }
                true
            })
            .collect()
    }

    // --- Medical Transcription Jobs ---

    pub fn start_medical_transcription_job(
        &mut self,
        name: &str,
        language_code: &str,
        media_uri: &str,
        media_format: Option<String>,
        media_sample_rate_hertz: Option<i32>,
        output_bucket_name: &str,
        specialty: &str,
        r#type: &str,
        now: f64,
    ) -> Result<&MedicalTranscriptionJobData, TranscribeError> {
        if self.medical_transcription_jobs.contains_key(name) {
            return Err(TranscribeError::JobAlreadyExists);
        }

        let job = MedicalTranscriptionJobData {
            medical_transcription_job_name: name.to_string(),
            transcription_job_status: "COMPLETED".to_string(),
            language_code: language_code.to_string(),
            media_uri: media_uri.to_string(),
            media_format,
            media_sample_rate_hertz,
            output_bucket_name: output_bucket_name.to_string(),
            specialty: specialty.to_string(),
            r#type: r#type.to_string(),
            creation_time: now,
            start_time: Some(now),
            completion_time: Some(now),
            failure_reason: None,
            transcript_file_uri: Some(format!(
                "https://s3.amazonaws.com/{output_bucket_name}/{name}.json"
            )),
        };

        self.medical_transcription_jobs
            .insert(name.to_string(), job);
        Ok(self.medical_transcription_jobs.get(name).unwrap())
    }

    pub fn get_medical_transcription_job(
        &self,
        name: &str,
    ) -> Result<&MedicalTranscriptionJobData, TranscribeError> {
        self.medical_transcription_jobs
            .get(name)
            .ok_or(TranscribeError::JobNotFound)
    }

    pub fn delete_medical_transcription_job(&mut self, name: &str) -> Result<(), TranscribeError> {
        if self.medical_transcription_jobs.remove(name).is_none() {
            return Err(TranscribeError::JobNotFound);
        }
        Ok(())
    }

    pub fn list_medical_transcription_jobs(
        &self,
        status_equals: Option<&str>,
        job_name_contains: Option<&str>,
    ) -> Vec<&MedicalTranscriptionJobData> {
        self.medical_transcription_jobs
            .values()
            .filter(|j| {
                if let Some(status) = status_equals
                    && j.transcription_job_status != status
                {
                    return false;
                }
                if let Some(contains) = job_name_contains
                    && !j
                        .medical_transcription_job_name
                        .to_lowercase()
                        .contains(&contains.to_lowercase())
                {
                    return false;
                }
                true
            })
            .collect()
    }

    // --- Medical Vocabularies ---

    pub fn create_medical_vocabulary(
        &mut self,
        name: &str,
        language_code: &str,
        vocabulary_file_uri: &str,
        now: f64,
    ) -> Result<&MedicalVocabularyData, TranscribeError> {
        if self.medical_vocabularies.contains_key(name) {
            return Err(TranscribeError::VocabularyAlreadyExists);
        }

        let vocab = MedicalVocabularyData {
            vocabulary_name: name.to_string(),
            language_code: language_code.to_string(),
            vocabulary_state: "READY".to_string(),
            vocabulary_file_uri: vocabulary_file_uri.to_string(),
            last_modified_time: now,
            failure_reason: None,
            download_uri: None,
        };

        self.medical_vocabularies.insert(name.to_string(), vocab);
        Ok(self.medical_vocabularies.get(name).unwrap())
    }

    pub fn get_medical_vocabulary(
        &self,
        name: &str,
    ) -> Result<&MedicalVocabularyData, TranscribeError> {
        self.medical_vocabularies
            .get(name)
            .ok_or(TranscribeError::VocabularyNotFound)
    }

    pub fn delete_medical_vocabulary(&mut self, name: &str) -> Result<(), TranscribeError> {
        if self.medical_vocabularies.remove(name).is_none() {
            return Err(TranscribeError::VocabularyNotFound);
        }
        Ok(())
    }

    pub fn list_medical_vocabularies(
        &self,
        state_equals: Option<&str>,
        name_contains: Option<&str>,
    ) -> Vec<&MedicalVocabularyData> {
        self.medical_vocabularies
            .values()
            .filter(|v| {
                if let Some(state) = state_equals
                    && v.vocabulary_state != state
                {
                    return false;
                }
                if let Some(contains) = name_contains
                    && !v
                        .vocabulary_name
                        .to_lowercase()
                        .contains(&contains.to_lowercase())
                {
                    return false;
                }
                true
            })
            .collect()
    }
}
