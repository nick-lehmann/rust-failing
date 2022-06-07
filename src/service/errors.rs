use crate::external::DatabaseError;
use anyhow::anyhow;

#[derive(Debug, thiserror::Error)]
pub enum ServiceError {
    #[error("Validation failed")]
    ValidationError(#[source] anyhow::Error),

    #[error("Forbidden")]
    Forbidden(),

    #[error("Unable to reach database")]
    Recoverable(#[source] anyhow::Error),

    #[error("Unexpected error")]
    Unexpected(#[source] anyhow::Error),
}

pub type ServiceResult<T> = Result<T, ServiceError>;

impl Into<ServiceError> for DatabaseError {
    fn into(self) -> ServiceError {
        match self {
            DatabaseError::NotFound() => ServiceError::Unexpected(self.into()),
            DatabaseError::DatabaseMissing(_) => {
                ServiceError::Unexpected(anyhow!(self).context("The database was missing"))
            }
            DatabaseError::Timeout() => {
                ServiceError::Recoverable(anyhow!(self).context("The database timed out"))
            }
        }
    }
}

impl From<retry::Error<ServiceError>> for ServiceError {
    fn from(error: retry::Error<ServiceError>) -> ServiceError {
        match error {
            retry::Error::Operation {
                error,
                total_delay,
                tries,
            } => error,
            retry::Error::Internal(msg) => ServiceError::Unexpected(anyhow!(msg)),
        }
    }
}
