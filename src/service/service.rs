use super::errors::ServiceResult;
use crate::external::{query, DatabaseError};

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
pub fn get_user(id: i32) -> ServiceResult<Option<i32>> {
    match query(id) {
        Ok(user) => Ok(Some(user)),
        Err(e) => match e {
            DatabaseError::NotFound() => Ok(None),
            _ => Err(e.into()),
        },
    }
}
