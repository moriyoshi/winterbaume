pub mod auth;
pub mod blob_store;
pub mod mock_aws;
pub mod mock_client;
pub mod protocol;
pub mod service;
pub mod state;
pub mod vfs;
pub mod views;

#[cfg(feature = "smithy-mocks")]
pub use aws_smithy_mocks;
pub use blob_store::{BlobStore, BlobStoreMap};
pub use mock_aws::MockAws;
pub use mock_client::MockAwsClient;
pub use protocol::{
    aws_query_response, body_has_top_level_field, extract_path, extract_query_string,
    json_error_response, parse_query_string, percent_decode, rest_json_error, urldecode,
    xml_escape,
};
pub use service::{MockRequest, MockResponse, MockService, StubService};
pub use state::{BackendState, DEFAULT_ACCOUNT_ID};
pub use vfs::{FsVfs, MemVfs, Vfs, VfsEntry, VfsError, VfsStat};
pub use views::{
    BlobBackedService, BlobExportEntry, BlobSource, BlobVisitor, DEFAULT_BLOB_BATCH_SIZE,
    StateChangeListener, StateChangeNotifier, StateViewError, StatefulService,
};
