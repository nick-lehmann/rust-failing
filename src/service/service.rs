use super::errors::UserServiceResult;
use crate::{
    external::{query, DatabaseError},
    permission::{Action, PermissionError, Resource},
};
use retry::{delay::Fixed, retry};

// Special ID to test for forbidden access. No user should have access to the user data associated with this ID.
pub static FORBIDDEN_ID: i32 = 1;
pub static VALID_ID: i32 = 10;
pub static INVALID_ID: i32 = 100;

/// Query an external database.
///
/// This operation might fail for multiple reasons which we all have to encode in the return value.
///
/// # Examples
///
/// Happy path. When a low id is passed, we return the id (which has the same value).
/// ```
/// use failing::service::service::*;
/// let result = get_user(2).unwrap();
/// assert_eq!(result, Some(2));
/// ```
///
/// When a high id is passed, we don't find anything. We regard this as expected behavior and return None.
/// ```
/// use failing::service::service::*;
/// let result = get_user(100).unwrap();
/// assert_eq!(result, None);
/// ```
pub fn get_user(id: i32) -> UserServiceResult<Option<i32>> {
    if id == FORBIDDEN_ID {
        Err(PermissionError::new(Resource::User, Action::Read, None))?;
    }

    Ok(retry(Fixed::from_millis(100).take(3), || {
        match query(id) {
            Ok(user) => Ok(Some(user)),
            Err(e) => match e {
                DatabaseError::NotFound() => Ok(None),
                error => Err(error.into()),
            },
        }
    })?)
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::state::{reset_state, DatabaseState, STATE};

    #[test]
    fn test_happy_path() {
        reset_state();

        let response = get_user(VALID_ID).unwrap();
        assert_eq!(response, Some(10));
    }

    #[test]
    fn test_user_not_found() {
        reset_state();

        let response = get_user(INVALID_ID).unwrap();
        assert_eq!(response, None);
    }

    #[test]
    fn test_database_not_reachable() {
        reset_state();
        STATE.lock().unwrap().database_state = DatabaseState::Unreachable();

        let response = get_user(FORBIDDEN_ID);

        assert!(response.is_err());
        // assert_eq!(response.unwrap_err().to_string(), "Not a valid id");
    }
}
