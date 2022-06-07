use crate::service::errors::ServiceError;
use anyhow::anyhow;

// Notes:
// - We do not expose the retry error. If a retry error occurs, we retry and the operation either succeeds or becomes an unexpected error.
#[derive(thiserror::Error)]
pub enum ApiError {
    // User has entered invalid data.
    #[error("Validation failed")]
    ValidationError(#[source] anyhow::Error),

    // User has no access to the required data.
    #[error("{0}")]
    Forbidden(String),

    // Something that should not have happened.
    #[error("An unexpected error occurred")]
    Unexpected(#[source] anyhow::Error),
}

impl std::fmt::Debug for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}

pub type ApiResult<T> = Result<T, ApiError>;

impl From<ServiceError> for ApiError {
    fn from(e: ServiceError) -> Self {
        match e {
            ServiceError::ValidationError(msg) => ApiError::ValidationError(anyhow!(msg)),
            ServiceError::Forbidden() => ApiError::Forbidden("Forbidden".into()),
            ServiceError::Recoverable(e) => ApiError::Unexpected(e),
            ServiceError::Unexpected(e) => ApiError::Unexpected(e),
        }
    }
}

pub fn error_chain_fmt(
    e: &impl std::error::Error,
    f: &mut std::fmt::Formatter<'_>,
) -> std::fmt::Result {
    writeln!(f, "{}\n", e)?;
    let mut current = e.source();
    while let Some(cause) = current {
        writeln!(f, "Caused by:\n\t{}", cause)?;
        current = cause.source();
    }
    Ok(())
}
