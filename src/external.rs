/// This module contains stubs for imports from external libraries.
///
use crate::state::{DatabaseState, STATE};

#[derive(Debug, thiserror::Error)]
pub enum DatabaseError {
    /// Not treated as an error in our case but converted to Option::None.
    #[error("Not found")]
    NotFound(),

    /// Unrecoverable errors.
    #[error("Database missing")]
    DatabaseMissing(String),

    /// Recoverable errors.
    #[error("Connection to the database timed out")]
    Timeout(),
}

pub fn query(id: i32) -> Result<i32, DatabaseError> {
    match STATE.lock().unwrap().database_state {
        DatabaseState::Fine() => {
            if id > 50 {
                return Err(DatabaseError::NotFound());
            }

            Ok(id)
        }
        DatabaseState::DatabaseMissing() => Err(DatabaseError::DatabaseMissing("user".to_string())),
        DatabaseState::Unreachable() => Err(DatabaseError::Timeout()),
    }
}
