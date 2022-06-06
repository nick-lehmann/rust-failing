#![allow(dead_code, unused_variables)]
use crate::service::{service::get_user, validation::validate_input, ApiInput};

use super::errors::ApiResult;

fn get_user_handler(api_input: ApiInput) -> ApiResult<Option<i32>> {
    let input = validate_input(api_input)?;
    get_user(input.id)
}

#[cfg(test)]
mod tests {}
