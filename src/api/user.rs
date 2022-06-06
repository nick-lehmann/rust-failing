#![allow(dead_code, unused_variables)]
use crate::service::{service::get_user, validation::validate_input, ApiInput};

use super::errors::ApiResult;

fn get_user_handler(api_input: ApiInput) -> ApiResult<Option<i32>> {
    let input = validate_input(api_input)?;
    Ok(get_user(input.id)?)
}

#[cfg(test)]
mod tests {
    use crate::{
        api::errors::ApiError,
        state::{reset_state, DatabaseState, STATE},
    };

    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_valid() {
        reset_state();

        let api_input = HashMap::from([("id".into(), "10".to_string())]);
        let response = get_user_handler(api_input).unwrap();

        assert_eq!(response, Some(10));
    }

    #[test]
    fn test_invalid_input() {
        reset_state();

        let api_input = HashMap::from([("id".into(), "foo".to_string())]);
        let response = get_user_handler(api_input);
        let error = response.unwrap_err();

        match error {
            ApiError::ValidationError(_) => "",
            _ => panic!("Expected ValidationError, instead returned: {:?}", error),
        };
    }

    #[test]
    fn test_forbidden() {
        reset_state();

        let api_input = HashMap::from([("id".into(), "1".to_string())]);
        let response = get_user_handler(api_input);
        let error = response.unwrap_err();

        match error {
            ApiError::Forbidden(_) => "",
            _ => panic!("Expected Forbidden, instead returned: {:?}", error),
        };
    }

    // TODO: Might need modification after retry is implemented.
    #[test]
    fn test_unreachable() {
        reset_state();
        STATE.lock().unwrap().database_state = DatabaseState::Unreachable();

        let api_input = HashMap::from([("id".into(), "10".to_string())]);
        let response = get_user_handler(api_input);

        match response {
            Err(ApiError::Unexpected(_)) => "",
            _ => panic!("Expected Unexpected, instead returned: {:?}", response),
        };
    }

    #[test]
    fn test_unexpected() {
        reset_state();
        STATE.lock().unwrap().database_state = DatabaseState::DatabaseMissing();

        let api_input = HashMap::from([("id".into(), "10".to_string())]);
        let response = get_user_handler(api_input);

        match response {
            Err(ApiError::Unexpected(_)) => "",
            _ => panic!("Expected Unexpected, instead returned: {:?}", response),
        };
    }
}
