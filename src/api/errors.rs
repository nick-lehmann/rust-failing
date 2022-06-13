use crate::{service::errors::UserServiceError, utils::error_chain_fmt};
use anyhow::anyhow;
use snafu::Snafu;

// Notes:
// - We do not expose the retry error. If a retry error occurs, we retry and the operation either succeeds or becomes an unexpected error.
// #[derive(thiserror::Error)]
// pub enum ApiError {
//     // User has entered invalid data.
//     #[error("Validation failed")]
//     ValidationError(#[source] anyhow::Error),

//     // User has no access to the required data.
//     #[error("{0}")]
//     Forbidden(String),

//     // Something that should not have happened.
//     #[error("An unexpected error occurred")]
//     Unexpected(#[source] anyhow::Error),
// }

#[derive(Snafu)]
pub enum ApiError {
    // User has entered invalid data.
    #[snafu(display("Validation failed"))]
    ValidationError { source: anyhow::Error },

    // User has no access to the required data.
    #[snafu(display("Forbidden"))]
    Forbidden,

    // Something that should not have happened.
    #[snafu(display("An unexpected error occurred"))]
    Unexpected { source: anyhow::Error },
}

impl std::fmt::Debug for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}

pub type ApiResult<T> = Result<T, ApiError>;

impl From<UserServiceError> for ApiError {
    fn from(e: UserServiceError) -> Self {
        match e {
            UserServiceError::ValidationError { .. } => {
                ApiError::ValidationError { source: anyhow!(e) }
            }
            UserServiceError::Forbidden => ApiError::Forbidden,
            UserServiceError::Recoverable { .. } => ApiError::Unexpected { source: anyhow!(e) },
            UserServiceError::Unexpected { .. } => ApiError::Unexpected { source: anyhow!(e) },
        }
    }
}
