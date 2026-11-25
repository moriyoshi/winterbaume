pub mod handlers;
pub mod model;
pub mod relaxations;
pub mod state;
pub mod types;
pub mod views;
pub mod wire;

pub use handlers::StsService;
pub use relaxations::{ASSUME_ROLE_WITHOUT_ROLE_ARN, RELAXATIONS};
pub use state::StsState;
pub use views::StsStateView;
