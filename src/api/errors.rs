use crate::service::errors::ServiceError;
use anyhow::anyhow;

// Notes:
// - We do not expose the retry error. If a retry error occurs, we retry and the operation either succeeds or becomes an unexpected error.
#[derive(Debug)]
pub enum ApiError {
    // User has entered invalid data.
    ValidationError(String),
    // User has no access to the required data.
    Forbidden(String),
    // Something that should not have happened.
    Unexpected(anyhow::Error),
}

pub type ApiResult<T> = Result<T, ApiError>;

impl From<ServiceError> for ApiError {
    fn from(e: ServiceError) -> Self {
        match e {
            ServiceError::ValidationError(_) => ApiError::ValidationError("ValidationError".into()),
            ServiceError::Forbidden() => ApiError::Forbidden("Forbidden".into()),
            ServiceError::Recoverable(e) => ApiError::Unexpected(anyhow!(e)),
            ServiceError::Unexpected(e) => ApiError::Unexpected(anyhow!(e)),
        }
    }
}
