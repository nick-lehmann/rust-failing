use crate::service::errors::ServiceError;

// Notes:
// - We do not expose the retry error. If a retry error occurs, we retry and the operation either succeeds or becomes an unexpected error.
pub enum ApiError {
    // User has entered invalid data.
    ValidationError(String),
    // User has no access to the required data.
    Forbidden(String),
    // Something that should not have happened.
    Unexpected(anyhow::Error),
}

pub type ApiResult<T> = Result<T, ServiceError>;
