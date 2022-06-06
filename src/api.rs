#![allow(dead_code, unused_variables)]

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

use crate::service::{
    errors::ServiceError, service::get_user, validation::validate_input, ApiInput,
};

pub type ApiResult<T> = Result<T, ServiceError>;

fn get_user_handler(api_input: ApiInput) -> ApiResult<Option<i32>> {
    let input = validate_input(api_input)?;
    get_user(input.id)
}

#[cfg(test)]
mod tests {}
