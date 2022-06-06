use crate::external::DatabaseError;

#[derive(Debug, thiserror::Error)]
pub enum ServiceError {
    #[error("{0}")]
    ValidationError(String),

    #[error("Forbidden")]
    Forbidden(),

    #[error("{0}")]
    Recoverable(String),

    #[error(transparent)]
    Unexpected(#[from] anyhow::Error),
}

pub type ServiceResult<T> = Result<T, ServiceError>;

impl Into<ServiceError> for DatabaseError {
    fn into(self) -> ServiceError {
        match self {
            DatabaseError::NotFound() => ServiceError::Unexpected(self.into()),
            DatabaseError::DatabaseMissing() => ServiceError::Unexpected(self.into()),
            DatabaseError::Timeout() => ServiceError::Recoverable("Database timed out".to_string()),
        }
    }
}
