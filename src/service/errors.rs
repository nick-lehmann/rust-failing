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
pub enum UserServiceError {
    #[snafu(display("Validation failed"))]
    ValidationError { source: ValidationError },

    #[snafu(display("Forbidden"))]
    Forbidden,

    #[snafu(display("A dependency is unavailable"))]
    Recoverable { source: anyhow::Error },

    #[snafu(display("Unexpected error"))]
    Unexpected { source: anyhow::Error },
}

pub type ServiceResult<T> = Result<T, UserServiceError>;

impl Into<UserServiceError> for DatabaseError {
    fn into(self) -> UserServiceError {
        match self {
            DatabaseError::NotFound() => {
                unreachable!("NotFound should always be converted into `None`")
            }
            DatabaseError::DatabaseMissing(_) => UserServiceError::Unexpected {
                source: anyhow!(self).context("The database was missing"),
            },
            DatabaseError::Timeout() => UserServiceError::Recoverable {
                source: anyhow!(self).context("The database timed out"),
            },
        }
    }
}

impl From<retry::Error<UserServiceError>> for UserServiceError {
    fn from(error: retry::Error<UserServiceError>) -> UserServiceError {
        match error {
            retry::Error::Operation {
                error,
                total_delay,
                tries,
            } => error,
            retry::Error::Internal(msg) => UserServiceError::Unexpected {
                source: anyhow!(msg),
            },
        }
    }
}
