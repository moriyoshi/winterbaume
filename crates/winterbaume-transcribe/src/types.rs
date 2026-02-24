#[derive(Debug, Clone)]
pub struct Vocabulary {
    pub vocabulary_name: String,
    pub language_code: String,
    pub vocabulary_state: String,
    pub last_modified_time: f64,
    pub phrases: Option<Vec<String>>,
    pub vocabulary_file_uri: Option<String>,
    pub failure_reason: Option<String>,
    pub download_uri: Option<String>,
}

#[derive(Debug, Clone)]
pub struct TranscriptionJobData {
    pub transcription_job_name: String,
    pub transcription_job_status: String,
    pub language_code: String,
    pub media_uri: String,
    pub media_format: Option<String>,
    pub media_sample_rate_hertz: Option<i32>,
    pub output_bucket_name: Option<String>,
    pub creation_time: f64,
    pub start_time: Option<f64>,
    pub completion_time: Option<f64>,
    pub failure_reason: Option<String>,
    pub transcript_file_uri: Option<String>,
}

#[derive(Debug, Clone)]
pub struct MedicalTranscriptionJobData {
    pub medical_transcription_job_name: String,
    pub transcription_job_status: String,
    pub language_code: String,
    pub media_uri: String,
    pub media_format: Option<String>,
    pub media_sample_rate_hertz: Option<i32>,
    pub output_bucket_name: String,
    pub specialty: String,
    pub r#type: String,
    pub creation_time: f64,
    pub start_time: Option<f64>,
    pub completion_time: Option<f64>,
    pub failure_reason: Option<String>,
    pub transcript_file_uri: Option<String>,
}

#[derive(Debug, Clone)]
pub struct MedicalVocabularyData {
    pub vocabulary_name: String,
    pub language_code: String,
    pub vocabulary_state: String,
    pub vocabulary_file_uri: String,
    pub last_modified_time: f64,
    pub failure_reason: Option<String>,
    pub download_uri: Option<String>,
}
