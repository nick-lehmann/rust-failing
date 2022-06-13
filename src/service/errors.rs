use crate::external::DatabaseError;
use anyhow::anyhow;
use snafu::Snafu;

use super::validation::ValidationError;

// #[derive(Debug, thiserror::Error)]
// pub enum ServiceError {
//     #[error("Validation failed")]
//     ValidationError(#[source] anyhow::Error),

//     #[error("Forbidden")]
//     Forbidden(),

//     #[error("Unable to reach database")]
//     Recoverable(#[source] anyhow::Error),

//     #[error("Unexpected error")]
//     Unexpected(#[source] anyhow::Error),
// }

#[derive(Debug, Snafu)]
pub enum ServiceError {
    #[snafu(display("Validation failed"))]
    ValidationError { source: ValidationError },

    #[snafu(display("Forbidden"))]
    Forbidden,

    #[snafu(display("A dependency is unavailable"))]
    Recoverable { source: anyhow::Error },

    #[snafu(display("Unexpected error"))]
    Unexpected { source: anyhow::Error },
}

pub type ServiceResult<T> = Result<T, ServiceError>;

impl Into<ServiceError> for DatabaseError {
    fn into(self) -> ServiceError {
        match self {
            DatabaseError::NotFound() => {
                unreachable!("NotFound should always be converted into `None`")
            }
            DatabaseError::DatabaseMissing(_) => ServiceError::Unexpected {
                source: anyhow!(self).context("The database was missing"),
            },
            DatabaseError::Timeout() => ServiceError::Recoverable {
                source: anyhow!(self).context("The database timed out"),
            },
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
            retry::Error::Internal(msg) => ServiceError::Unexpected {
                source: anyhow!(msg),
            },
        }
    }
}
