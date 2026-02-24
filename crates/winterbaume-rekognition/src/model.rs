//! Auto-generated types from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-rekognition

#![allow(non_camel_case_types, clippy::upper_case_acronyms, dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AssociateFacesRequest {
    #[serde(rename = "ClientRequestToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_token: Option<String>,
    #[serde(rename = "CollectionId")]
    #[serde(default)]
    pub collection_id: String,
    #[serde(rename = "FaceIds")]
    #[serde(default)]
    pub face_ids: Vec<String>,
    #[serde(rename = "UserId")]
    #[serde(default)]
    pub user_id: String,
    #[serde(rename = "UserMatchThreshold")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_match_threshold: Option<f32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AssociateFacesResponse {
    #[serde(rename = "AssociatedFaces")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub associated_faces: Option<Vec<AssociatedFace>>,
    #[serde(rename = "UnsuccessfulFaceAssociations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unsuccessful_face_associations: Option<Vec<UnsuccessfulFaceAssociation>>,
    #[serde(rename = "UserStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AssociatedFace {
    #[serde(rename = "FaceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub face_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UnsuccessfulFaceAssociation {
    #[serde(rename = "Confidence")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confidence: Option<f32>,
    #[serde(rename = "FaceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub face_id: Option<String>,
    #[serde(rename = "Reasons")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reasons: Option<Vec<String>>,
    #[serde(rename = "UserId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CompareFacesRequest {
    #[serde(rename = "QualityFilter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quality_filter: Option<String>,
    #[serde(rename = "SimilarityThreshold")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub similarity_threshold: Option<f32>,
    #[serde(rename = "SourceImage")]
    #[serde(default)]
    pub source_image: Image,
    #[serde(rename = "TargetImage")]
    #[serde(default)]
    pub target_image: Image,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Image {
    #[serde(rename = "Bytes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bytes: Option<String>,
    #[serde(rename = "S3Object")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_object: Option<S3Object>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct S3Object {
    #[serde(rename = "Bucket")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Version")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CompareFacesResponse {
    #[serde(rename = "FaceMatches")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub face_matches: Option<Vec<CompareFacesMatch>>,
    #[serde(rename = "SourceImageFace")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_image_face: Option<ComparedSourceImageFace>,
    #[serde(rename = "SourceImageOrientationCorrection")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_image_orientation_correction: Option<String>,
    #[serde(rename = "TargetImageOrientationCorrection")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_image_orientation_correction: Option<String>,
    #[serde(rename = "UnmatchedFaces")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unmatched_faces: Option<Vec<ComparedFace>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CompareFacesMatch {
    #[serde(rename = "Face")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub face: Option<ComparedFace>,
    #[serde(rename = "Similarity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub similarity: Option<f32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ComparedFace {
    #[serde(rename = "BoundingBox")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bounding_box: Option<BoundingBox>,
    #[serde(rename = "Confidence")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confidence: Option<f32>,
    #[serde(rename = "Emotions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emotions: Option<Vec<Emotion>>,
    #[serde(rename = "Landmarks")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub landmarks: Option<Vec<Landmark>>,
    #[serde(rename = "Pose")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pose: Option<Pose>,
    #[serde(rename = "Quality")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quality: Option<ImageQuality>,
    #[serde(rename = "Smile")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub smile: Option<Smile>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BoundingBox {
    #[serde(rename = "Height")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<f32>,
    #[serde(rename = "Left")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub left: Option<f32>,
    #[serde(rename = "Top")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top: Option<f32>,
    #[serde(rename = "Width")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<f32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Emotion {
    #[serde(rename = "Confidence")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confidence: Option<f32>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Landmark {
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "X")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x: Option<f32>,
    #[serde(rename = "Y")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub y: Option<f32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Pose {
    #[serde(rename = "Pitch")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pitch: Option<f32>,
    #[serde(rename = "Roll")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub roll: Option<f32>,
    #[serde(rename = "Yaw")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub yaw: Option<f32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ImageQuality {
    #[serde(rename = "Brightness")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub brightness: Option<f32>,
    #[serde(rename = "Sharpness")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sharpness: Option<f32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Smile {
    #[serde(rename = "Confidence")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confidence: Option<f32>,
    #[serde(rename = "Value")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ComparedSourceImageFace {
    #[serde(rename = "BoundingBox")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bounding_box: Option<BoundingBox>,
    #[serde(rename = "Confidence")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confidence: Option<f32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CopyProjectVersionRequest {
    #[serde(rename = "DestinationProjectArn")]
    #[serde(default)]
    pub destination_project_arn: String,
    #[serde(rename = "KmsKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<String>,
    #[serde(rename = "OutputConfig")]
    #[serde(default)]
    pub output_config: OutputConfig,
    #[serde(rename = "SourceProjectArn")]
    #[serde(default)]
    pub source_project_arn: String,
    #[serde(rename = "SourceProjectVersionArn")]
    #[serde(default)]
    pub source_project_version_arn: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "VersionName")]
    #[serde(default)]
    pub version_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct OutputConfig {
    #[serde(rename = "S3Bucket")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_bucket: Option<String>,
    #[serde(rename = "S3KeyPrefix")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_key_prefix: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CopyProjectVersionResponse {
    #[serde(rename = "ProjectVersionArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_version_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateCollectionRequest {
    #[serde(rename = "CollectionId")]
    #[serde(default)]
    pub collection_id: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateCollectionResponse {
    #[serde(rename = "CollectionArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collection_arn: Option<String>,
    #[serde(rename = "FaceModelVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub face_model_version: Option<String>,
    #[serde(rename = "StatusCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_code: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateDatasetRequest {
    #[serde(rename = "DatasetSource")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dataset_source: Option<DatasetSource>,
    #[serde(rename = "DatasetType")]
    #[serde(default)]
    pub dataset_type: String,
    #[serde(rename = "ProjectArn")]
    #[serde(default)]
    pub project_arn: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DatasetSource {
    #[serde(rename = "DatasetArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dataset_arn: Option<String>,
    #[serde(rename = "GroundTruthManifest")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ground_truth_manifest: Option<GroundTruthManifest>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GroundTruthManifest {
    #[serde(rename = "S3Object")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_object: Option<S3Object>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateDatasetResponse {
    #[serde(rename = "DatasetArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dataset_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateFaceLivenessSessionRequest {
    #[serde(rename = "ClientRequestToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_token: Option<String>,
    #[serde(rename = "KmsKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<String>,
    #[serde(rename = "Settings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings: Option<CreateFaceLivenessSessionRequestSettings>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateFaceLivenessSessionRequestSettings {
    #[serde(rename = "AuditImagesLimit")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audit_images_limit: Option<i32>,
    #[serde(rename = "ChallengePreferences")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub challenge_preferences: Option<Vec<ChallengePreference>>,
    #[serde(rename = "OutputConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_config: Option<LivenessOutputConfig>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ChallengePreference {
    #[serde(rename = "Type")]
    #[serde(default)]
    pub r#type: String,
    #[serde(rename = "Versions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub versions: Option<Versions>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Versions {
    #[serde(rename = "Maximum")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum: Option<String>,
    #[serde(rename = "Minimum")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LivenessOutputConfig {
    #[serde(rename = "S3Bucket")]
    #[serde(default)]
    pub s3_bucket: String,
    #[serde(rename = "S3KeyPrefix")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_key_prefix: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateFaceLivenessSessionResponse {
    #[serde(rename = "SessionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateProjectRequest {
    #[serde(rename = "AutoUpdate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_update: Option<String>,
    #[serde(rename = "Feature")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feature: Option<String>,
    #[serde(rename = "ProjectName")]
    #[serde(default)]
    pub project_name: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateProjectResponse {
    #[serde(rename = "ProjectArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateProjectVersionRequest {
    #[serde(rename = "FeatureConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feature_config: Option<CustomizationFeatureConfig>,
    #[serde(rename = "KmsKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<String>,
    #[serde(rename = "OutputConfig")]
    #[serde(default)]
    pub output_config: OutputConfig,
    #[serde(rename = "ProjectArn")]
    #[serde(default)]
    pub project_arn: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "TestingData")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub testing_data: Option<TestingData>,
    #[serde(rename = "TrainingData")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub training_data: Option<TrainingData>,
    #[serde(rename = "VersionDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_description: Option<String>,
    #[serde(rename = "VersionName")]
    #[serde(default)]
    pub version_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CustomizationFeatureConfig {
    #[serde(rename = "ContentModeration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_moderation: Option<CustomizationFeatureContentModerationConfig>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CustomizationFeatureContentModerationConfig {
    #[serde(rename = "ConfidenceThreshold")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confidence_threshold: Option<f32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TestingData {
    #[serde(rename = "Assets")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assets: Option<Vec<Asset>>,
    #[serde(rename = "AutoCreate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_create: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Asset {
    #[serde(rename = "GroundTruthManifest")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ground_truth_manifest: Option<GroundTruthManifest>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TrainingData {
    #[serde(rename = "Assets")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assets: Option<Vec<Asset>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateProjectVersionResponse {
    #[serde(rename = "ProjectVersionArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_version_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateStreamProcessorRequest {
    #[serde(rename = "DataSharingPreference")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_sharing_preference: Option<StreamProcessorDataSharingPreference>,
    #[serde(rename = "Input")]
    #[serde(default)]
    pub input: StreamProcessorInput,
    #[serde(rename = "KmsKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "NotificationChannel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification_channel: Option<StreamProcessorNotificationChannel>,
    #[serde(rename = "Output")]
    #[serde(default)]
    pub output: StreamProcessorOutput,
    #[serde(rename = "RegionsOfInterest")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub regions_of_interest: Option<Vec<RegionOfInterest>>,
    #[serde(rename = "RoleArn")]
    #[serde(default)]
    pub role_arn: String,
    #[serde(rename = "Settings")]
    #[serde(default)]
    pub settings: StreamProcessorSettings,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StreamProcessorDataSharingPreference {
    #[serde(rename = "OptIn")]
    #[serde(default)]
    pub opt_in: bool,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StreamProcessorInput {
    #[serde(rename = "KinesisVideoStream")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kinesis_video_stream: Option<KinesisVideoStream>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct KinesisVideoStream {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StreamProcessorNotificationChannel {
    #[serde(rename = "SNSTopicArn")]
    #[serde(default)]
    pub s_n_s_topic_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StreamProcessorOutput {
    #[serde(rename = "KinesisDataStream")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kinesis_data_stream: Option<KinesisDataStream>,
    #[serde(rename = "S3Destination")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_destination: Option<S3Destination>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct KinesisDataStream {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct S3Destination {
    #[serde(rename = "Bucket")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket: Option<String>,
    #[serde(rename = "KeyPrefix")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_prefix: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RegionOfInterest {
    #[serde(rename = "BoundingBox")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bounding_box: Option<BoundingBox>,
    #[serde(rename = "Polygon")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub polygon: Option<Vec<Point>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Point {
    #[serde(rename = "X")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x: Option<f32>,
    #[serde(rename = "Y")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub y: Option<f32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StreamProcessorSettings {
    #[serde(rename = "ConnectedHome")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connected_home: Option<ConnectedHomeSettings>,
    #[serde(rename = "FaceSearch")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub face_search: Option<FaceSearchSettings>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ConnectedHomeSettings {
    #[serde(rename = "Labels")]
    #[serde(default)]
    pub labels: Vec<String>,
    #[serde(rename = "MinConfidence")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_confidence: Option<f32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FaceSearchSettings {
    #[serde(rename = "CollectionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collection_id: Option<String>,
    #[serde(rename = "FaceMatchThreshold")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub face_match_threshold: Option<f32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateStreamProcessorResponse {
    #[serde(rename = "StreamProcessorArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_processor_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateUserRequest {
    #[serde(rename = "ClientRequestToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_token: Option<String>,
    #[serde(rename = "CollectionId")]
    #[serde(default)]
    pub collection_id: String,
    #[serde(rename = "UserId")]
    #[serde(default)]
    pub user_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateUserResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteCollectionRequest {
    #[serde(rename = "CollectionId")]
    #[serde(default)]
    pub collection_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteCollectionResponse {
    #[serde(rename = "StatusCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_code: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteDatasetRequest {
    #[serde(rename = "DatasetArn")]
    #[serde(default)]
    pub dataset_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteDatasetResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteFacesRequest {
    #[serde(rename = "CollectionId")]
    #[serde(default)]
    pub collection_id: String,
    #[serde(rename = "FaceIds")]
    #[serde(default)]
    pub face_ids: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteFacesResponse {
    #[serde(rename = "DeletedFaces")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deleted_faces: Option<Vec<String>>,
    #[serde(rename = "UnsuccessfulFaceDeletions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unsuccessful_face_deletions: Option<Vec<UnsuccessfulFaceDeletion>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UnsuccessfulFaceDeletion {
    #[serde(rename = "FaceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub face_id: Option<String>,
    #[serde(rename = "Reasons")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reasons: Option<Vec<String>>,
    #[serde(rename = "UserId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteProjectPolicyRequest {
    #[serde(rename = "PolicyName")]
    #[serde(default)]
    pub policy_name: String,
    #[serde(rename = "PolicyRevisionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_revision_id: Option<String>,
    #[serde(rename = "ProjectArn")]
    #[serde(default)]
    pub project_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteProjectPolicyResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteProjectRequest {
    #[serde(rename = "ProjectArn")]
    #[serde(default)]
    pub project_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteProjectResponse {
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteProjectVersionRequest {
    #[serde(rename = "ProjectVersionArn")]
    #[serde(default)]
    pub project_version_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteProjectVersionResponse {
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteStreamProcessorRequest {
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteStreamProcessorResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteUserRequest {
    #[serde(rename = "ClientRequestToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_token: Option<String>,
    #[serde(rename = "CollectionId")]
    #[serde(default)]
    pub collection_id: String,
    #[serde(rename = "UserId")]
    #[serde(default)]
    pub user_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteUserResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeCollectionRequest {
    #[serde(rename = "CollectionId")]
    #[serde(default)]
    pub collection_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeCollectionResponse {
    #[serde(rename = "CollectionARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collection_a_r_n: Option<String>,
    #[serde(rename = "CreationTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_timestamp: Option<f64>,
    #[serde(rename = "FaceCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub face_count: Option<i64>,
    #[serde(rename = "FaceModelVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub face_model_version: Option<String>,
    #[serde(rename = "UserCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_count: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeDatasetRequest {
    #[serde(rename = "DatasetArn")]
    #[serde(default)]
    pub dataset_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeDatasetResponse {
    #[serde(rename = "DatasetDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dataset_description: Option<DatasetDescription>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DatasetDescription {
    #[serde(rename = "CreationTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_timestamp: Option<f64>,
    #[serde(rename = "DatasetStats")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dataset_stats: Option<DatasetStats>,
    #[serde(rename = "LastUpdatedTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_timestamp: Option<f64>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "StatusMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_message: Option<String>,
    #[serde(rename = "StatusMessageCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_message_code: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DatasetStats {
    #[serde(rename = "ErrorEntries")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_entries: Option<i32>,
    #[serde(rename = "LabeledEntries")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labeled_entries: Option<i32>,
    #[serde(rename = "TotalEntries")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_entries: Option<i32>,
    #[serde(rename = "TotalLabels")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_labels: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeProjectVersionsRequest {
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "ProjectArn")]
    #[serde(default)]
    pub project_arn: String,
    #[serde(rename = "VersionNames")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_names: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeProjectVersionsResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "ProjectVersionDescriptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_version_descriptions: Option<Vec<ProjectVersionDescription>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ProjectVersionDescription {
    #[serde(rename = "BaseModelVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_model_version: Option<String>,
    #[serde(rename = "BillableTrainingTimeInSeconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billable_training_time_in_seconds: Option<i64>,
    #[serde(rename = "CreationTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_timestamp: Option<f64>,
    #[serde(rename = "EvaluationResult")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evaluation_result: Option<EvaluationResult>,
    #[serde(rename = "Feature")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feature: Option<String>,
    #[serde(rename = "FeatureConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feature_config: Option<CustomizationFeatureConfig>,
    #[serde(rename = "KmsKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<String>,
    #[serde(rename = "ManifestSummary")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manifest_summary: Option<GroundTruthManifest>,
    #[serde(rename = "MaxInferenceUnits")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_inference_units: Option<i32>,
    #[serde(rename = "MinInferenceUnits")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_inference_units: Option<i32>,
    #[serde(rename = "OutputConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_config: Option<OutputConfig>,
    #[serde(rename = "ProjectVersionArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_version_arn: Option<String>,
    #[serde(rename = "SourceProjectVersionArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_project_version_arn: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "StatusMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_message: Option<String>,
    #[serde(rename = "TestingDataResult")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub testing_data_result: Option<TestingDataResult>,
    #[serde(rename = "TrainingDataResult")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub training_data_result: Option<TrainingDataResult>,
    #[serde(rename = "TrainingEndTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub training_end_timestamp: Option<f64>,
    #[serde(rename = "VersionDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_description: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EvaluationResult {
    #[serde(rename = "F1Score")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub f1_score: Option<f32>,
    #[serde(rename = "Summary")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<Summary>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Summary {
    #[serde(rename = "S3Object")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_object: Option<S3Object>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TestingDataResult {
    #[serde(rename = "Input")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input: Option<TestingData>,
    #[serde(rename = "Output")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output: Option<TestingData>,
    #[serde(rename = "Validation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation: Option<ValidationData>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ValidationData {
    #[serde(rename = "Assets")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assets: Option<Vec<Asset>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TrainingDataResult {
    #[serde(rename = "Input")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input: Option<TrainingData>,
    #[serde(rename = "Output")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output: Option<TrainingData>,
    #[serde(rename = "Validation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation: Option<ValidationData>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeProjectsRequest {
    #[serde(rename = "Features")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub features: Option<Vec<String>>,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "ProjectNames")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_names: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeProjectsResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "ProjectDescriptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_descriptions: Option<Vec<ProjectDescription>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ProjectDescription {
    #[serde(rename = "AutoUpdate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_update: Option<String>,
    #[serde(rename = "CreationTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_timestamp: Option<f64>,
    #[serde(rename = "Datasets")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub datasets: Option<Vec<DatasetMetadata>>,
    #[serde(rename = "Feature")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feature: Option<String>,
    #[serde(rename = "ProjectArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_arn: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DatasetMetadata {
    #[serde(rename = "CreationTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_timestamp: Option<f64>,
    #[serde(rename = "DatasetArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dataset_arn: Option<String>,
    #[serde(rename = "DatasetType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dataset_type: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "StatusMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_message: Option<String>,
    #[serde(rename = "StatusMessageCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_message_code: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeStreamProcessorRequest {
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeStreamProcessorResponse {
    #[serde(rename = "CreationTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_timestamp: Option<f64>,
    #[serde(rename = "DataSharingPreference")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_sharing_preference: Option<StreamProcessorDataSharingPreference>,
    #[serde(rename = "Input")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input: Option<StreamProcessorInput>,
    #[serde(rename = "KmsKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<String>,
    #[serde(rename = "LastUpdateTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_update_timestamp: Option<f64>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "NotificationChannel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification_channel: Option<StreamProcessorNotificationChannel>,
    #[serde(rename = "Output")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output: Option<StreamProcessorOutput>,
    #[serde(rename = "RegionsOfInterest")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub regions_of_interest: Option<Vec<RegionOfInterest>>,
    #[serde(rename = "RoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
    #[serde(rename = "Settings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings: Option<StreamProcessorSettings>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "StatusMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_message: Option<String>,
    #[serde(rename = "StreamProcessorArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_processor_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DetectCustomLabelsRequest {
    #[serde(rename = "Image")]
    #[serde(default)]
    pub image: Image,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "MinConfidence")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_confidence: Option<f32>,
    #[serde(rename = "ProjectVersionArn")]
    #[serde(default)]
    pub project_version_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DetectCustomLabelsResponse {
    #[serde(rename = "CustomLabels")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_labels: Option<Vec<CustomLabel>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CustomLabel {
    #[serde(rename = "Confidence")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confidence: Option<f32>,
    #[serde(rename = "Geometry")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub geometry: Option<Geometry>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Geometry {
    #[serde(rename = "BoundingBox")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bounding_box: Option<BoundingBox>,
    #[serde(rename = "Polygon")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub polygon: Option<Vec<Point>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DetectFacesRequest {
    #[serde(rename = "Attributes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<Vec<String>>,
    #[serde(rename = "Image")]
    #[serde(default)]
    pub image: Image,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DetectFacesResponse {
    #[serde(rename = "FaceDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub face_details: Option<Vec<FaceDetail>>,
    #[serde(rename = "OrientationCorrection")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub orientation_correction: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FaceDetail {
    #[serde(rename = "AgeRange")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub age_range: Option<AgeRange>,
    #[serde(rename = "Beard")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub beard: Option<Beard>,
    #[serde(rename = "BoundingBox")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bounding_box: Option<BoundingBox>,
    #[serde(rename = "Confidence")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confidence: Option<f32>,
    #[serde(rename = "Emotions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emotions: Option<Vec<Emotion>>,
    #[serde(rename = "EyeDirection")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eye_direction: Option<EyeDirection>,
    #[serde(rename = "Eyeglasses")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eyeglasses: Option<Eyeglasses>,
    #[serde(rename = "EyesOpen")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eyes_open: Option<EyeOpen>,
    #[serde(rename = "FaceOccluded")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub face_occluded: Option<FaceOccluded>,
    #[serde(rename = "Gender")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gender: Option<Gender>,
    #[serde(rename = "Landmarks")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub landmarks: Option<Vec<Landmark>>,
    #[serde(rename = "MouthOpen")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mouth_open: Option<MouthOpen>,
    #[serde(rename = "Mustache")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mustache: Option<Mustache>,
    #[serde(rename = "Pose")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pose: Option<Pose>,
    #[serde(rename = "Quality")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quality: Option<ImageQuality>,
    #[serde(rename = "Smile")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub smile: Option<Smile>,
    #[serde(rename = "Sunglasses")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sunglasses: Option<Sunglasses>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AgeRange {
    #[serde(rename = "High")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub high: Option<i32>,
    #[serde(rename = "Low")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub low: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Beard {
    #[serde(rename = "Confidence")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confidence: Option<f32>,
    #[serde(rename = "Value")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EyeDirection {
    #[serde(rename = "Confidence")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confidence: Option<f32>,
    #[serde(rename = "Pitch")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pitch: Option<f32>,
    #[serde(rename = "Yaw")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub yaw: Option<f32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Eyeglasses {
    #[serde(rename = "Confidence")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confidence: Option<f32>,
    #[serde(rename = "Value")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EyeOpen {
    #[serde(rename = "Confidence")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confidence: Option<f32>,
    #[serde(rename = "Value")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FaceOccluded {
    #[serde(rename = "Confidence")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confidence: Option<f32>,
    #[serde(rename = "Value")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Gender {
    #[serde(rename = "Confidence")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confidence: Option<f32>,
    #[serde(rename = "Value")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MouthOpen {
    #[serde(rename = "Confidence")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confidence: Option<f32>,
    #[serde(rename = "Value")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Mustache {
    #[serde(rename = "Confidence")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confidence: Option<f32>,
    #[serde(rename = "Value")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Sunglasses {
    #[serde(rename = "Confidence")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confidence: Option<f32>,
    #[serde(rename = "Value")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DetectLabelsRequest {
    #[serde(rename = "Features")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub features: Option<Vec<String>>,
    #[serde(rename = "Image")]
    #[serde(default)]
    pub image: Image,
    #[serde(rename = "MaxLabels")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_labels: Option<i32>,
    #[serde(rename = "MinConfidence")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_confidence: Option<f32>,
    #[serde(rename = "Settings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings: Option<DetectLabelsSettings>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DetectLabelsSettings {
    #[serde(rename = "GeneralLabels")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub general_labels: Option<GeneralLabelsSettings>,
    #[serde(rename = "ImageProperties")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_properties: Option<DetectLabelsImagePropertiesSettings>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GeneralLabelsSettings {
    #[serde(rename = "LabelCategoryExclusionFilters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label_category_exclusion_filters: Option<Vec<String>>,
    #[serde(rename = "LabelCategoryInclusionFilters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label_category_inclusion_filters: Option<Vec<String>>,
    #[serde(rename = "LabelExclusionFilters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label_exclusion_filters: Option<Vec<String>>,
    #[serde(rename = "LabelInclusionFilters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label_inclusion_filters: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DetectLabelsImagePropertiesSettings {
    #[serde(rename = "MaxDominantColors")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_dominant_colors: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DetectLabelsResponse {
    #[serde(rename = "ImageProperties")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_properties: Option<DetectLabelsImageProperties>,
    #[serde(rename = "LabelModelVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label_model_version: Option<String>,
    #[serde(rename = "Labels")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<Label>>,
    #[serde(rename = "OrientationCorrection")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub orientation_correction: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DetectLabelsImageProperties {
    #[serde(rename = "Background")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background: Option<DetectLabelsImageBackground>,
    #[serde(rename = "DominantColors")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dominant_colors: Option<Vec<DominantColor>>,
    #[serde(rename = "Foreground")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub foreground: Option<DetectLabelsImageForeground>,
    #[serde(rename = "Quality")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quality: Option<DetectLabelsImageQuality>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DetectLabelsImageBackground {
    #[serde(rename = "DominantColors")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dominant_colors: Option<Vec<DominantColor>>,
    #[serde(rename = "Quality")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quality: Option<DetectLabelsImageQuality>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DominantColor {
    #[serde(rename = "Blue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blue: Option<i32>,
    #[serde(rename = "CSSColor")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub c_s_s_color: Option<String>,
    #[serde(rename = "Green")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub green: Option<i32>,
    #[serde(rename = "HexCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hex_code: Option<String>,
    #[serde(rename = "PixelPercent")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pixel_percent: Option<f32>,
    #[serde(rename = "Red")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub red: Option<i32>,
    #[serde(rename = "SimplifiedColor")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub simplified_color: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DetectLabelsImageQuality {
    #[serde(rename = "Brightness")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub brightness: Option<f32>,
    #[serde(rename = "Contrast")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contrast: Option<f32>,
    #[serde(rename = "Sharpness")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sharpness: Option<f32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DetectLabelsImageForeground {
    #[serde(rename = "DominantColors")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dominant_colors: Option<Vec<DominantColor>>,
    #[serde(rename = "Quality")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quality: Option<DetectLabelsImageQuality>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Label {
    #[serde(rename = "Aliases")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aliases: Option<Vec<LabelAlias>>,
    #[serde(rename = "Categories")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub categories: Option<Vec<LabelCategory>>,
    #[serde(rename = "Confidence")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confidence: Option<f32>,
    #[serde(rename = "Instances")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instances: Option<Vec<Instance>>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Parents")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parents: Option<Vec<Parent>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LabelAlias {
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LabelCategory {
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Instance {
    #[serde(rename = "BoundingBox")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bounding_box: Option<BoundingBox>,
    #[serde(rename = "Confidence")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confidence: Option<f32>,
    #[serde(rename = "DominantColors")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dominant_colors: Option<Vec<DominantColor>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Parent {
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DetectModerationLabelsRequest {
    #[serde(rename = "HumanLoopConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub human_loop_config: Option<HumanLoopConfig>,
    #[serde(rename = "Image")]
    #[serde(default)]
    pub image: Image,
    #[serde(rename = "MinConfidence")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_confidence: Option<f32>,
    #[serde(rename = "ProjectVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_version: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct HumanLoopConfig {
    #[serde(rename = "DataAttributes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_attributes: Option<HumanLoopDataAttributes>,
    #[serde(rename = "FlowDefinitionArn")]
    #[serde(default)]
    pub flow_definition_arn: String,
    #[serde(rename = "HumanLoopName")]
    #[serde(default)]
    pub human_loop_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct HumanLoopDataAttributes {
    #[serde(rename = "ContentClassifiers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_classifiers: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DetectModerationLabelsResponse {
    #[serde(rename = "ContentTypes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_types: Option<Vec<ContentType>>,
    #[serde(rename = "HumanLoopActivationOutput")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub human_loop_activation_output: Option<HumanLoopActivationOutput>,
    #[serde(rename = "ModerationLabels")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub moderation_labels: Option<Vec<ModerationLabel>>,
    #[serde(rename = "ModerationModelVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub moderation_model_version: Option<String>,
    #[serde(rename = "ProjectVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_version: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ContentType {
    #[serde(rename = "Confidence")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confidence: Option<f32>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct HumanLoopActivationOutput {
    #[serde(rename = "HumanLoopActivationConditionsEvaluationResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub human_loop_activation_conditions_evaluation_results: Option<String>,
    #[serde(rename = "HumanLoopActivationReasons")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub human_loop_activation_reasons: Option<Vec<String>>,
    #[serde(rename = "HumanLoopArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub human_loop_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ModerationLabel {
    #[serde(rename = "Confidence")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confidence: Option<f32>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "ParentName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_name: Option<String>,
    #[serde(rename = "TaxonomyLevel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub taxonomy_level: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DetectProtectiveEquipmentRequest {
    #[serde(rename = "Image")]
    #[serde(default)]
    pub image: Image,
    #[serde(rename = "SummarizationAttributes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summarization_attributes: Option<ProtectiveEquipmentSummarizationAttributes>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ProtectiveEquipmentSummarizationAttributes {
    #[serde(rename = "MinConfidence")]
    #[serde(default)]
    pub min_confidence: f32,
    #[serde(rename = "RequiredEquipmentTypes")]
    #[serde(default)]
    pub required_equipment_types: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DetectProtectiveEquipmentResponse {
    #[serde(rename = "Persons")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub persons: Option<Vec<ProtectiveEquipmentPerson>>,
    #[serde(rename = "ProtectiveEquipmentModelVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protective_equipment_model_version: Option<String>,
    #[serde(rename = "Summary")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<ProtectiveEquipmentSummary>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ProtectiveEquipmentPerson {
    #[serde(rename = "BodyParts")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body_parts: Option<Vec<ProtectiveEquipmentBodyPart>>,
    #[serde(rename = "BoundingBox")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bounding_box: Option<BoundingBox>,
    #[serde(rename = "Confidence")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confidence: Option<f32>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ProtectiveEquipmentBodyPart {
    #[serde(rename = "Confidence")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confidence: Option<f32>,
    #[serde(rename = "EquipmentDetections")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub equipment_detections: Option<Vec<EquipmentDetection>>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EquipmentDetection {
    #[serde(rename = "BoundingBox")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bounding_box: Option<BoundingBox>,
    #[serde(rename = "Confidence")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confidence: Option<f32>,
    #[serde(rename = "CoversBodyPart")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub covers_body_part: Option<CoversBodyPart>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CoversBodyPart {
    #[serde(rename = "Confidence")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confidence: Option<f32>,
    #[serde(rename = "Value")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ProtectiveEquipmentSummary {
    #[serde(rename = "PersonsIndeterminate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub persons_indeterminate: Option<Vec<i32>>,
    #[serde(rename = "PersonsWithRequiredEquipment")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub persons_with_required_equipment: Option<Vec<i32>>,
    #[serde(rename = "PersonsWithoutRequiredEquipment")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub persons_without_required_equipment: Option<Vec<i32>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DetectTextRequest {
    #[serde(rename = "Filters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<DetectTextFilters>,
    #[serde(rename = "Image")]
    #[serde(default)]
    pub image: Image,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DetectTextFilters {
    #[serde(rename = "RegionsOfInterest")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub regions_of_interest: Option<Vec<RegionOfInterest>>,
    #[serde(rename = "WordFilter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub word_filter: Option<DetectionFilter>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DetectionFilter {
    #[serde(rename = "MinBoundingBoxHeight")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_bounding_box_height: Option<f32>,
    #[serde(rename = "MinBoundingBoxWidth")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_bounding_box_width: Option<f32>,
    #[serde(rename = "MinConfidence")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_confidence: Option<f32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DetectTextResponse {
    #[serde(rename = "TextDetections")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_detections: Option<Vec<TextDetection>>,
    #[serde(rename = "TextModelVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_model_version: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TextDetection {
    #[serde(rename = "Confidence")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confidence: Option<f32>,
    #[serde(rename = "DetectedText")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detected_text: Option<String>,
    #[serde(rename = "Geometry")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub geometry: Option<Geometry>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(rename = "ParentId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<i32>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DisassociateFacesRequest {
    #[serde(rename = "ClientRequestToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_token: Option<String>,
    #[serde(rename = "CollectionId")]
    #[serde(default)]
    pub collection_id: String,
    #[serde(rename = "FaceIds")]
    #[serde(default)]
    pub face_ids: Vec<String>,
    #[serde(rename = "UserId")]
    #[serde(default)]
    pub user_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DisassociateFacesResponse {
    #[serde(rename = "DisassociatedFaces")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disassociated_faces: Option<Vec<DisassociatedFace>>,
    #[serde(rename = "UnsuccessfulFaceDisassociations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unsuccessful_face_disassociations: Option<Vec<UnsuccessfulFaceDisassociation>>,
    #[serde(rename = "UserStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DisassociatedFace {
    #[serde(rename = "FaceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub face_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UnsuccessfulFaceDisassociation {
    #[serde(rename = "FaceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub face_id: Option<String>,
    #[serde(rename = "Reasons")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reasons: Option<Vec<String>>,
    #[serde(rename = "UserId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DistributeDatasetEntriesRequest {
    #[serde(rename = "Datasets")]
    #[serde(default)]
    pub datasets: Vec<DistributeDataset>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DistributeDataset {
    #[serde(rename = "Arn")]
    #[serde(default)]
    pub arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DistributeDatasetEntriesResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetCelebrityInfoRequest {
    #[serde(rename = "Id")]
    #[serde(default)]
    pub id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetCelebrityInfoResponse {
    #[serde(rename = "KnownGender")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub known_gender: Option<KnownGender>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Urls")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub urls: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct KnownGender {
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetCelebrityRecognitionRequest {
    #[serde(rename = "JobId")]
    #[serde(default)]
    pub job_id: String,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "SortBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetCelebrityRecognitionResponse {
    #[serde(rename = "Celebrities")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub celebrities: Option<Vec<CelebrityRecognition>>,
    #[serde(rename = "JobId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
    #[serde(rename = "JobStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_status: Option<String>,
    #[serde(rename = "JobTag")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_tag: Option<String>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "StatusMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_message: Option<String>,
    #[serde(rename = "Video")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video: Option<Video>,
    #[serde(rename = "VideoMetadata")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_metadata: Option<VideoMetadata>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CelebrityRecognition {
    #[serde(rename = "Celebrity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub celebrity: Option<CelebrityDetail>,
    #[serde(rename = "Timestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CelebrityDetail {
    #[serde(rename = "BoundingBox")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bounding_box: Option<BoundingBox>,
    #[serde(rename = "Confidence")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confidence: Option<f32>,
    #[serde(rename = "Face")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub face: Option<FaceDetail>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "KnownGender")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub known_gender: Option<KnownGender>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Urls")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub urls: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Video {
    #[serde(rename = "S3Object")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_object: Option<S3Object>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct VideoMetadata {
    #[serde(rename = "Codec")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub codec: Option<String>,
    #[serde(rename = "ColorRange")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color_range: Option<String>,
    #[serde(rename = "DurationMillis")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration_millis: Option<i64>,
    #[serde(rename = "Format")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    #[serde(rename = "FrameHeight")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frame_height: Option<i64>,
    #[serde(rename = "FrameRate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frame_rate: Option<f32>,
    #[serde(rename = "FrameWidth")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frame_width: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetContentModerationRequest {
    #[serde(rename = "AggregateBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aggregate_by: Option<String>,
    #[serde(rename = "JobId")]
    #[serde(default)]
    pub job_id: String,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "SortBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetContentModerationResponse {
    #[serde(rename = "GetRequestMetadata")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub get_request_metadata: Option<GetContentModerationRequestMetadata>,
    #[serde(rename = "JobId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
    #[serde(rename = "JobStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_status: Option<String>,
    #[serde(rename = "JobTag")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_tag: Option<String>,
    #[serde(rename = "ModerationLabels")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub moderation_labels: Option<Vec<ContentModerationDetection>>,
    #[serde(rename = "ModerationModelVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub moderation_model_version: Option<String>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "StatusMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_message: Option<String>,
    #[serde(rename = "Video")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video: Option<Video>,
    #[serde(rename = "VideoMetadata")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_metadata: Option<VideoMetadata>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetContentModerationRequestMetadata {
    #[serde(rename = "AggregateBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aggregate_by: Option<String>,
    #[serde(rename = "SortBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ContentModerationDetection {
    #[serde(rename = "ContentTypes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_types: Option<Vec<ContentType>>,
    #[serde(rename = "DurationMillis")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration_millis: Option<i64>,
    #[serde(rename = "EndTimestampMillis")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_timestamp_millis: Option<i64>,
    #[serde(rename = "ModerationLabel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub moderation_label: Option<ModerationLabel>,
    #[serde(rename = "StartTimestampMillis")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_timestamp_millis: Option<i64>,
    #[serde(rename = "Timestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetFaceDetectionRequest {
    #[serde(rename = "JobId")]
    #[serde(default)]
    pub job_id: String,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetFaceDetectionResponse {
    #[serde(rename = "Faces")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub faces: Option<Vec<FaceDetection>>,
    #[serde(rename = "JobId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
    #[serde(rename = "JobStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_status: Option<String>,
    #[serde(rename = "JobTag")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_tag: Option<String>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "StatusMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_message: Option<String>,
    #[serde(rename = "Video")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video: Option<Video>,
    #[serde(rename = "VideoMetadata")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_metadata: Option<VideoMetadata>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FaceDetection {
    #[serde(rename = "Face")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub face: Option<FaceDetail>,
    #[serde(rename = "Timestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetFaceLivenessSessionResultsRequest {
    #[serde(rename = "SessionId")]
    #[serde(default)]
    pub session_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetFaceLivenessSessionResultsResponse {
    #[serde(rename = "AuditImages")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audit_images: Option<Vec<AuditImage>>,
    #[serde(rename = "Challenge")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub challenge: Option<Challenge>,
    #[serde(rename = "Confidence")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confidence: Option<f32>,
    #[serde(rename = "ReferenceImage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference_image: Option<AuditImage>,
    #[serde(rename = "SessionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_id: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AuditImage {
    #[serde(rename = "BoundingBox")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bounding_box: Option<BoundingBox>,
    #[serde(rename = "Bytes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bytes: Option<String>,
    #[serde(rename = "S3Object")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_object: Option<S3Object>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Challenge {
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "Version")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetFaceSearchRequest {
    #[serde(rename = "JobId")]
    #[serde(default)]
    pub job_id: String,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "SortBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetFaceSearchResponse {
    #[serde(rename = "JobId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
    #[serde(rename = "JobStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_status: Option<String>,
    #[serde(rename = "JobTag")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_tag: Option<String>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "Persons")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub persons: Option<Vec<PersonMatch>>,
    #[serde(rename = "StatusMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_message: Option<String>,
    #[serde(rename = "Video")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video: Option<Video>,
    #[serde(rename = "VideoMetadata")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_metadata: Option<VideoMetadata>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PersonMatch {
    #[serde(rename = "FaceMatches")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub face_matches: Option<Vec<FaceMatch>>,
    #[serde(rename = "Person")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub person: Option<PersonDetail>,
    #[serde(rename = "Timestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FaceMatch {
    #[serde(rename = "Face")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub face: Option<Face>,
    #[serde(rename = "Similarity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub similarity: Option<f32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Face {
    #[serde(rename = "BoundingBox")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bounding_box: Option<BoundingBox>,
    #[serde(rename = "Confidence")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confidence: Option<f32>,
    #[serde(rename = "ExternalImageId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_image_id: Option<String>,
    #[serde(rename = "FaceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub face_id: Option<String>,
    #[serde(rename = "ImageId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_id: Option<String>,
    #[serde(rename = "IndexFacesModelVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_faces_model_version: Option<String>,
    #[serde(rename = "UserId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PersonDetail {
    #[serde(rename = "BoundingBox")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bounding_box: Option<BoundingBox>,
    #[serde(rename = "Face")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub face: Option<FaceDetail>,
    #[serde(rename = "Index")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetLabelDetectionRequest {
    #[serde(rename = "AggregateBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aggregate_by: Option<String>,
    #[serde(rename = "JobId")]
    #[serde(default)]
    pub job_id: String,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "SortBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetLabelDetectionResponse {
    #[serde(rename = "GetRequestMetadata")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub get_request_metadata: Option<GetLabelDetectionRequestMetadata>,
    #[serde(rename = "JobId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
    #[serde(rename = "JobStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_status: Option<String>,
    #[serde(rename = "JobTag")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_tag: Option<String>,
    #[serde(rename = "LabelModelVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label_model_version: Option<String>,
    #[serde(rename = "Labels")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<LabelDetection>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "StatusMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_message: Option<String>,
    #[serde(rename = "Video")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video: Option<Video>,
    #[serde(rename = "VideoMetadata")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_metadata: Option<VideoMetadata>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetLabelDetectionRequestMetadata {
    #[serde(rename = "AggregateBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aggregate_by: Option<String>,
    #[serde(rename = "SortBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LabelDetection {
    #[serde(rename = "DurationMillis")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration_millis: Option<i64>,
    #[serde(rename = "EndTimestampMillis")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_timestamp_millis: Option<i64>,
    #[serde(rename = "Label")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<Label>,
    #[serde(rename = "StartTimestampMillis")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_timestamp_millis: Option<i64>,
    #[serde(rename = "Timestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetMediaAnalysisJobRequest {
    #[serde(rename = "JobId")]
    #[serde(default)]
    pub job_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetMediaAnalysisJobResponse {
    #[serde(rename = "CompletionTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completion_timestamp: Option<f64>,
    #[serde(rename = "CreationTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_timestamp: Option<f64>,
    #[serde(rename = "FailureDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_details: Option<MediaAnalysisJobFailureDetails>,
    #[serde(rename = "Input")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input: Option<MediaAnalysisInput>,
    #[serde(rename = "JobId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
    #[serde(rename = "JobName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_name: Option<String>,
    #[serde(rename = "KmsKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<String>,
    #[serde(rename = "ManifestSummary")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manifest_summary: Option<MediaAnalysisManifestSummary>,
    #[serde(rename = "OperationsConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operations_config: Option<MediaAnalysisOperationsConfig>,
    #[serde(rename = "OutputConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_config: Option<MediaAnalysisOutputConfig>,
    #[serde(rename = "Results")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub results: Option<MediaAnalysisResults>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MediaAnalysisJobFailureDetails {
    #[serde(rename = "Code")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(rename = "Message")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MediaAnalysisInput {
    #[serde(rename = "S3Object")]
    #[serde(default)]
    pub s3_object: S3Object,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MediaAnalysisManifestSummary {
    #[serde(rename = "S3Object")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_object: Option<S3Object>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MediaAnalysisOperationsConfig {
    #[serde(rename = "DetectModerationLabels")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detect_moderation_labels: Option<MediaAnalysisDetectModerationLabelsConfig>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MediaAnalysisDetectModerationLabelsConfig {
    #[serde(rename = "MinConfidence")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_confidence: Option<f32>,
    #[serde(rename = "ProjectVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_version: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MediaAnalysisOutputConfig {
    #[serde(rename = "S3Bucket")]
    #[serde(default)]
    pub s3_bucket: String,
    #[serde(rename = "S3KeyPrefix")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_key_prefix: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MediaAnalysisResults {
    #[serde(rename = "ModelVersions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_versions: Option<MediaAnalysisModelVersions>,
    #[serde(rename = "S3Object")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_object: Option<S3Object>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MediaAnalysisModelVersions {
    #[serde(rename = "Moderation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub moderation: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetPersonTrackingRequest {
    #[serde(rename = "JobId")]
    #[serde(default)]
    pub job_id: String,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "SortBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetPersonTrackingResponse {
    #[serde(rename = "JobId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
    #[serde(rename = "JobStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_status: Option<String>,
    #[serde(rename = "JobTag")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_tag: Option<String>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "Persons")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub persons: Option<Vec<PersonDetection>>,
    #[serde(rename = "StatusMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_message: Option<String>,
    #[serde(rename = "Video")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video: Option<Video>,
    #[serde(rename = "VideoMetadata")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_metadata: Option<VideoMetadata>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PersonDetection {
    #[serde(rename = "Person")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub person: Option<PersonDetail>,
    #[serde(rename = "Timestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetSegmentDetectionRequest {
    #[serde(rename = "JobId")]
    #[serde(default)]
    pub job_id: String,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetSegmentDetectionResponse {
    #[serde(rename = "AudioMetadata")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_metadata: Option<Vec<AudioMetadata>>,
    #[serde(rename = "JobId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
    #[serde(rename = "JobStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_status: Option<String>,
    #[serde(rename = "JobTag")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_tag: Option<String>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "Segments")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segments: Option<Vec<SegmentDetection>>,
    #[serde(rename = "SelectedSegmentTypes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selected_segment_types: Option<Vec<SegmentTypeInfo>>,
    #[serde(rename = "StatusMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_message: Option<String>,
    #[serde(rename = "Video")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video: Option<Video>,
    #[serde(rename = "VideoMetadata")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_metadata: Option<Vec<VideoMetadata>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AudioMetadata {
    #[serde(rename = "Codec")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub codec: Option<String>,
    #[serde(rename = "DurationMillis")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration_millis: Option<i64>,
    #[serde(rename = "NumberOfChannels")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_channels: Option<i64>,
    #[serde(rename = "SampleRate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sample_rate: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SegmentDetection {
    #[serde(rename = "DurationFrames")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration_frames: Option<i64>,
    #[serde(rename = "DurationMillis")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration_millis: Option<i64>,
    #[serde(rename = "DurationSMPTE")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration_s_m_p_t_e: Option<String>,
    #[serde(rename = "EndFrameNumber")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_frame_number: Option<i64>,
    #[serde(rename = "EndTimecodeSMPTE")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_timecode_s_m_p_t_e: Option<String>,
    #[serde(rename = "EndTimestampMillis")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_timestamp_millis: Option<i64>,
    #[serde(rename = "ShotSegment")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shot_segment: Option<ShotSegment>,
    #[serde(rename = "StartFrameNumber")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_frame_number: Option<i64>,
    #[serde(rename = "StartTimecodeSMPTE")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_timecode_s_m_p_t_e: Option<String>,
    #[serde(rename = "StartTimestampMillis")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_timestamp_millis: Option<i64>,
    #[serde(rename = "TechnicalCueSegment")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub technical_cue_segment: Option<TechnicalCueSegment>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ShotSegment {
    #[serde(rename = "Confidence")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confidence: Option<f32>,
    #[serde(rename = "Index")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TechnicalCueSegment {
    #[serde(rename = "Confidence")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confidence: Option<f32>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SegmentTypeInfo {
    #[serde(rename = "ModelVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_version: Option<String>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetTextDetectionRequest {
    #[serde(rename = "JobId")]
    #[serde(default)]
    pub job_id: String,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetTextDetectionResponse {
    #[serde(rename = "JobId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
    #[serde(rename = "JobStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_status: Option<String>,
    #[serde(rename = "JobTag")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_tag: Option<String>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "StatusMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_message: Option<String>,
    #[serde(rename = "TextDetections")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_detections: Option<Vec<TextDetectionResult>>,
    #[serde(rename = "TextModelVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_model_version: Option<String>,
    #[serde(rename = "Video")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video: Option<Video>,
    #[serde(rename = "VideoMetadata")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_metadata: Option<VideoMetadata>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TextDetectionResult {
    #[serde(rename = "TextDetection")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_detection: Option<TextDetection>,
    #[serde(rename = "Timestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct IndexFacesRequest {
    #[serde(rename = "CollectionId")]
    #[serde(default)]
    pub collection_id: String,
    #[serde(rename = "DetectionAttributes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detection_attributes: Option<Vec<String>>,
    #[serde(rename = "ExternalImageId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_image_id: Option<String>,
    #[serde(rename = "Image")]
    #[serde(default)]
    pub image: Image,
    #[serde(rename = "MaxFaces")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_faces: Option<i32>,
    #[serde(rename = "QualityFilter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quality_filter: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct IndexFacesResponse {
    #[serde(rename = "FaceModelVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub face_model_version: Option<String>,
    #[serde(rename = "FaceRecords")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub face_records: Option<Vec<FaceRecord>>,
    #[serde(rename = "OrientationCorrection")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub orientation_correction: Option<String>,
    #[serde(rename = "UnindexedFaces")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unindexed_faces: Option<Vec<UnindexedFace>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FaceRecord {
    #[serde(rename = "Face")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub face: Option<Face>,
    #[serde(rename = "FaceDetail")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub face_detail: Option<FaceDetail>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UnindexedFace {
    #[serde(rename = "FaceDetail")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub face_detail: Option<FaceDetail>,
    #[serde(rename = "Reasons")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reasons: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListCollectionsRequest {
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListCollectionsResponse {
    #[serde(rename = "CollectionIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collection_ids: Option<Vec<String>>,
    #[serde(rename = "FaceModelVersions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub face_model_versions: Option<Vec<String>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListDatasetEntriesRequest {
    #[serde(rename = "ContainsLabels")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contains_labels: Option<Vec<String>>,
    #[serde(rename = "DatasetArn")]
    #[serde(default)]
    pub dataset_arn: String,
    #[serde(rename = "HasErrors")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_errors: Option<bool>,
    #[serde(rename = "Labeled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labeled: Option<bool>,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "SourceRefContains")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_ref_contains: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListDatasetEntriesResponse {
    #[serde(rename = "DatasetEntries")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dataset_entries: Option<Vec<String>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListDatasetLabelsRequest {
    #[serde(rename = "DatasetArn")]
    #[serde(default)]
    pub dataset_arn: String,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListDatasetLabelsResponse {
    #[serde(rename = "DatasetLabelDescriptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dataset_label_descriptions: Option<Vec<DatasetLabelDescription>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DatasetLabelDescription {
    #[serde(rename = "LabelName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label_name: Option<String>,
    #[serde(rename = "LabelStats")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label_stats: Option<DatasetLabelStats>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DatasetLabelStats {
    #[serde(rename = "BoundingBoxCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bounding_box_count: Option<i32>,
    #[serde(rename = "EntryCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entry_count: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListFacesRequest {
    #[serde(rename = "CollectionId")]
    #[serde(default)]
    pub collection_id: String,
    #[serde(rename = "FaceIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub face_ids: Option<Vec<String>>,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "UserId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListFacesResponse {
    #[serde(rename = "FaceModelVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub face_model_version: Option<String>,
    #[serde(rename = "Faces")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub faces: Option<Vec<Face>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListMediaAnalysisJobsRequest {
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListMediaAnalysisJobsResponse {
    #[serde(rename = "MediaAnalysisJobs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_analysis_jobs: Option<Vec<MediaAnalysisJobDescription>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MediaAnalysisJobDescription {
    #[serde(rename = "CompletionTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completion_timestamp: Option<f64>,
    #[serde(rename = "CreationTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_timestamp: Option<f64>,
    #[serde(rename = "FailureDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_details: Option<MediaAnalysisJobFailureDetails>,
    #[serde(rename = "Input")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input: Option<MediaAnalysisInput>,
    #[serde(rename = "JobId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
    #[serde(rename = "JobName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_name: Option<String>,
    #[serde(rename = "KmsKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<String>,
    #[serde(rename = "ManifestSummary")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manifest_summary: Option<MediaAnalysisManifestSummary>,
    #[serde(rename = "OperationsConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operations_config: Option<MediaAnalysisOperationsConfig>,
    #[serde(rename = "OutputConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_config: Option<MediaAnalysisOutputConfig>,
    #[serde(rename = "Results")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub results: Option<MediaAnalysisResults>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListProjectPoliciesRequest {
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "ProjectArn")]
    #[serde(default)]
    pub project_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListProjectPoliciesResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "ProjectPolicies")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_policies: Option<Vec<ProjectPolicy>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ProjectPolicy {
    #[serde(rename = "CreationTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_timestamp: Option<f64>,
    #[serde(rename = "LastUpdatedTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_timestamp: Option<f64>,
    #[serde(rename = "PolicyDocument")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_document: Option<String>,
    #[serde(rename = "PolicyName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_name: Option<String>,
    #[serde(rename = "PolicyRevisionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_revision_id: Option<String>,
    #[serde(rename = "ProjectArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListStreamProcessorsRequest {
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListStreamProcessorsResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "StreamProcessors")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_processors: Option<Vec<StreamProcessor>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StreamProcessor {
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTagsForResourceRequest {
    #[serde(rename = "ResourceArn")]
    #[serde(default)]
    pub resource_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTagsForResourceResponse {
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListUsersRequest {
    #[serde(rename = "CollectionId")]
    #[serde(default)]
    pub collection_id: String,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListUsersResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "Users")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub users: Option<Vec<User>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct User {
    #[serde(rename = "UserId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    #[serde(rename = "UserStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutProjectPolicyRequest {
    #[serde(rename = "PolicyDocument")]
    #[serde(default)]
    pub policy_document: String,
    #[serde(rename = "PolicyName")]
    #[serde(default)]
    pub policy_name: String,
    #[serde(rename = "PolicyRevisionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_revision_id: Option<String>,
    #[serde(rename = "ProjectArn")]
    #[serde(default)]
    pub project_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutProjectPolicyResponse {
    #[serde(rename = "PolicyRevisionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_revision_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RecognizeCelebritiesRequest {
    #[serde(rename = "Image")]
    #[serde(default)]
    pub image: Image,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RecognizeCelebritiesResponse {
    #[serde(rename = "CelebrityFaces")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub celebrity_faces: Option<Vec<Celebrity>>,
    #[serde(rename = "OrientationCorrection")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub orientation_correction: Option<String>,
    #[serde(rename = "UnrecognizedFaces")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unrecognized_faces: Option<Vec<ComparedFace>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Celebrity {
    #[serde(rename = "Face")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub face: Option<ComparedFace>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "KnownGender")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub known_gender: Option<KnownGender>,
    #[serde(rename = "MatchConfidence")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub match_confidence: Option<f32>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Urls")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub urls: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SearchFacesByImageRequest {
    #[serde(rename = "CollectionId")]
    #[serde(default)]
    pub collection_id: String,
    #[serde(rename = "FaceMatchThreshold")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub face_match_threshold: Option<f32>,
    #[serde(rename = "Image")]
    #[serde(default)]
    pub image: Image,
    #[serde(rename = "MaxFaces")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_faces: Option<i32>,
    #[serde(rename = "QualityFilter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quality_filter: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SearchFacesByImageResponse {
    #[serde(rename = "FaceMatches")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub face_matches: Option<Vec<FaceMatch>>,
    #[serde(rename = "FaceModelVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub face_model_version: Option<String>,
    #[serde(rename = "SearchedFaceBoundingBox")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub searched_face_bounding_box: Option<BoundingBox>,
    #[serde(rename = "SearchedFaceConfidence")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub searched_face_confidence: Option<f32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SearchFacesRequest {
    #[serde(rename = "CollectionId")]
    #[serde(default)]
    pub collection_id: String,
    #[serde(rename = "FaceId")]
    #[serde(default)]
    pub face_id: String,
    #[serde(rename = "FaceMatchThreshold")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub face_match_threshold: Option<f32>,
    #[serde(rename = "MaxFaces")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_faces: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SearchFacesResponse {
    #[serde(rename = "FaceMatches")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub face_matches: Option<Vec<FaceMatch>>,
    #[serde(rename = "FaceModelVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub face_model_version: Option<String>,
    #[serde(rename = "SearchedFaceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub searched_face_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SearchUsersByImageRequest {
    #[serde(rename = "CollectionId")]
    #[serde(default)]
    pub collection_id: String,
    #[serde(rename = "Image")]
    #[serde(default)]
    pub image: Image,
    #[serde(rename = "MaxUsers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_users: Option<i32>,
    #[serde(rename = "QualityFilter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quality_filter: Option<String>,
    #[serde(rename = "UserMatchThreshold")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_match_threshold: Option<f32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SearchUsersByImageResponse {
    #[serde(rename = "FaceModelVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub face_model_version: Option<String>,
    #[serde(rename = "SearchedFace")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub searched_face: Option<SearchedFaceDetails>,
    #[serde(rename = "UnsearchedFaces")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unsearched_faces: Option<Vec<UnsearchedFace>>,
    #[serde(rename = "UserMatches")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_matches: Option<Vec<UserMatch>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SearchedFaceDetails {
    #[serde(rename = "FaceDetail")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub face_detail: Option<FaceDetail>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UnsearchedFace {
    #[serde(rename = "FaceDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub face_details: Option<FaceDetail>,
    #[serde(rename = "Reasons")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reasons: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UserMatch {
    #[serde(rename = "Similarity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub similarity: Option<f32>,
    #[serde(rename = "User")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<MatchedUser>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MatchedUser {
    #[serde(rename = "UserId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    #[serde(rename = "UserStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SearchUsersRequest {
    #[serde(rename = "CollectionId")]
    #[serde(default)]
    pub collection_id: String,
    #[serde(rename = "FaceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub face_id: Option<String>,
    #[serde(rename = "MaxUsers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_users: Option<i32>,
    #[serde(rename = "UserId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    #[serde(rename = "UserMatchThreshold")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_match_threshold: Option<f32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SearchUsersResponse {
    #[serde(rename = "FaceModelVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub face_model_version: Option<String>,
    #[serde(rename = "SearchedFace")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub searched_face: Option<SearchedFace>,
    #[serde(rename = "SearchedUser")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub searched_user: Option<SearchedUser>,
    #[serde(rename = "UserMatches")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_matches: Option<Vec<UserMatch>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SearchedFace {
    #[serde(rename = "FaceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub face_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SearchedUser {
    #[serde(rename = "UserId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartCelebrityRecognitionRequest {
    #[serde(rename = "ClientRequestToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_token: Option<String>,
    #[serde(rename = "JobTag")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_tag: Option<String>,
    #[serde(rename = "NotificationChannel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification_channel: Option<NotificationChannel>,
    #[serde(rename = "Video")]
    #[serde(default)]
    pub video: Video,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct NotificationChannel {
    #[serde(rename = "RoleArn")]
    #[serde(default)]
    pub role_arn: String,
    #[serde(rename = "SNSTopicArn")]
    #[serde(default)]
    pub s_n_s_topic_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartCelebrityRecognitionResponse {
    #[serde(rename = "JobId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartContentModerationRequest {
    #[serde(rename = "ClientRequestToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_token: Option<String>,
    #[serde(rename = "JobTag")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_tag: Option<String>,
    #[serde(rename = "MinConfidence")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_confidence: Option<f32>,
    #[serde(rename = "NotificationChannel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification_channel: Option<NotificationChannel>,
    #[serde(rename = "Video")]
    #[serde(default)]
    pub video: Video,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartContentModerationResponse {
    #[serde(rename = "JobId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartFaceDetectionRequest {
    #[serde(rename = "ClientRequestToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_token: Option<String>,
    #[serde(rename = "FaceAttributes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub face_attributes: Option<String>,
    #[serde(rename = "JobTag")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_tag: Option<String>,
    #[serde(rename = "NotificationChannel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification_channel: Option<NotificationChannel>,
    #[serde(rename = "Video")]
    #[serde(default)]
    pub video: Video,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartFaceDetectionResponse {
    #[serde(rename = "JobId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartFaceSearchRequest {
    #[serde(rename = "ClientRequestToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_token: Option<String>,
    #[serde(rename = "CollectionId")]
    #[serde(default)]
    pub collection_id: String,
    #[serde(rename = "FaceMatchThreshold")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub face_match_threshold: Option<f32>,
    #[serde(rename = "JobTag")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_tag: Option<String>,
    #[serde(rename = "NotificationChannel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification_channel: Option<NotificationChannel>,
    #[serde(rename = "Video")]
    #[serde(default)]
    pub video: Video,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartFaceSearchResponse {
    #[serde(rename = "JobId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartLabelDetectionRequest {
    #[serde(rename = "ClientRequestToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_token: Option<String>,
    #[serde(rename = "Features")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub features: Option<Vec<String>>,
    #[serde(rename = "JobTag")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_tag: Option<String>,
    #[serde(rename = "MinConfidence")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_confidence: Option<f32>,
    #[serde(rename = "NotificationChannel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification_channel: Option<NotificationChannel>,
    #[serde(rename = "Settings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings: Option<LabelDetectionSettings>,
    #[serde(rename = "Video")]
    #[serde(default)]
    pub video: Video,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LabelDetectionSettings {
    #[serde(rename = "GeneralLabels")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub general_labels: Option<GeneralLabelsSettings>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartLabelDetectionResponse {
    #[serde(rename = "JobId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartMediaAnalysisJobRequest {
    #[serde(rename = "ClientRequestToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_token: Option<String>,
    #[serde(rename = "Input")]
    #[serde(default)]
    pub input: MediaAnalysisInput,
    #[serde(rename = "JobName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_name: Option<String>,
    #[serde(rename = "KmsKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<String>,
    #[serde(rename = "OperationsConfig")]
    #[serde(default)]
    pub operations_config: MediaAnalysisOperationsConfig,
    #[serde(rename = "OutputConfig")]
    #[serde(default)]
    pub output_config: MediaAnalysisOutputConfig,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartMediaAnalysisJobResponse {
    #[serde(rename = "JobId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartPersonTrackingRequest {
    #[serde(rename = "ClientRequestToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_token: Option<String>,
    #[serde(rename = "JobTag")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_tag: Option<String>,
    #[serde(rename = "NotificationChannel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification_channel: Option<NotificationChannel>,
    #[serde(rename = "Video")]
    #[serde(default)]
    pub video: Video,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartPersonTrackingResponse {
    #[serde(rename = "JobId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartProjectVersionRequest {
    #[serde(rename = "MaxInferenceUnits")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_inference_units: Option<i32>,
    #[serde(rename = "MinInferenceUnits")]
    #[serde(default)]
    pub min_inference_units: i32,
    #[serde(rename = "ProjectVersionArn")]
    #[serde(default)]
    pub project_version_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartProjectVersionResponse {
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartSegmentDetectionRequest {
    #[serde(rename = "ClientRequestToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_token: Option<String>,
    #[serde(rename = "Filters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<StartSegmentDetectionFilters>,
    #[serde(rename = "JobTag")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_tag: Option<String>,
    #[serde(rename = "NotificationChannel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification_channel: Option<NotificationChannel>,
    #[serde(rename = "SegmentTypes")]
    #[serde(default)]
    pub segment_types: Vec<String>,
    #[serde(rename = "Video")]
    #[serde(default)]
    pub video: Video,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartSegmentDetectionFilters {
    #[serde(rename = "ShotFilter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shot_filter: Option<StartShotDetectionFilter>,
    #[serde(rename = "TechnicalCueFilter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub technical_cue_filter: Option<StartTechnicalCueDetectionFilter>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartShotDetectionFilter {
    #[serde(rename = "MinSegmentConfidence")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_segment_confidence: Option<f32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartTechnicalCueDetectionFilter {
    #[serde(rename = "BlackFrame")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub black_frame: Option<BlackFrame>,
    #[serde(rename = "MinSegmentConfidence")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_segment_confidence: Option<f32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BlackFrame {
    #[serde(rename = "MaxPixelThreshold")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_pixel_threshold: Option<f32>,
    #[serde(rename = "MinCoveragePercentage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_coverage_percentage: Option<f32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartSegmentDetectionResponse {
    #[serde(rename = "JobId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartStreamProcessorRequest {
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "StartSelector")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_selector: Option<StreamProcessingStartSelector>,
    #[serde(rename = "StopSelector")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_selector: Option<StreamProcessingStopSelector>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StreamProcessingStartSelector {
    #[serde(rename = "KVSStreamStartSelector")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub k_v_s_stream_start_selector: Option<KinesisVideoStreamStartSelector>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct KinesisVideoStreamStartSelector {
    #[serde(rename = "FragmentNumber")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fragment_number: Option<String>,
    #[serde(rename = "ProducerTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub producer_timestamp: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StreamProcessingStopSelector {
    #[serde(rename = "MaxDurationInSeconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_duration_in_seconds: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartStreamProcessorResponse {
    #[serde(rename = "SessionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartTextDetectionRequest {
    #[serde(rename = "ClientRequestToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_token: Option<String>,
    #[serde(rename = "Filters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<StartTextDetectionFilters>,
    #[serde(rename = "JobTag")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_tag: Option<String>,
    #[serde(rename = "NotificationChannel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification_channel: Option<NotificationChannel>,
    #[serde(rename = "Video")]
    #[serde(default)]
    pub video: Video,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartTextDetectionFilters {
    #[serde(rename = "RegionsOfInterest")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub regions_of_interest: Option<Vec<RegionOfInterest>>,
    #[serde(rename = "WordFilter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub word_filter: Option<DetectionFilter>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartTextDetectionResponse {
    #[serde(rename = "JobId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StopProjectVersionRequest {
    #[serde(rename = "ProjectVersionArn")]
    #[serde(default)]
    pub project_version_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StopProjectVersionResponse {
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StopStreamProcessorRequest {
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StopStreamProcessorResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TagResourceRequest {
    #[serde(rename = "ResourceArn")]
    #[serde(default)]
    pub resource_arn: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    pub tags: std::collections::HashMap<String, String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TagResourceResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UntagResourceRequest {
    #[serde(rename = "ResourceArn")]
    #[serde(default)]
    pub resource_arn: String,
    #[serde(rename = "TagKeys")]
    #[serde(default)]
    pub tag_keys: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UntagResourceResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateDatasetEntriesRequest {
    #[serde(rename = "Changes")]
    #[serde(default)]
    pub changes: DatasetChanges,
    #[serde(rename = "DatasetArn")]
    #[serde(default)]
    pub dataset_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DatasetChanges {
    #[serde(rename = "GroundTruth")]
    #[serde(default)]
    pub ground_truth: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateDatasetEntriesResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateStreamProcessorRequest {
    #[serde(rename = "DataSharingPreferenceForUpdate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_sharing_preference_for_update: Option<StreamProcessorDataSharingPreference>,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "ParametersToDelete")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters_to_delete: Option<Vec<String>>,
    #[serde(rename = "RegionsOfInterestForUpdate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub regions_of_interest_for_update: Option<Vec<RegionOfInterest>>,
    #[serde(rename = "SettingsForUpdate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings_for_update: Option<StreamProcessorSettingsForUpdate>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StreamProcessorSettingsForUpdate {
    #[serde(rename = "ConnectedHomeForUpdate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connected_home_for_update: Option<ConnectedHomeSettingsForUpdate>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ConnectedHomeSettingsForUpdate {
    #[serde(rename = "Labels")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<String>>,
    #[serde(rename = "MinConfidence")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_confidence: Option<f32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateStreamProcessorResponse {}
