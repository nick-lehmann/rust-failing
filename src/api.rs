#![allow(dead_code, unused_variables)]

// pub enum ApiError {
//     ValidationError(String),
//     Recoverable(String),
//     Unexpected(anyhow::Error),
// }

use crate::service::{
    errors::ServiceError, service::get_user, validation::validate_input, ApiInput,
};

pub enum HTTPStatus {
    Ok = 200,
    BadRequest = 400,
    NotFound = 404,
    InternalServerError = 500,
}

pub type ApiResult<T> = Result<T, ServiceError>;

fn get_user_handler(api_input: ApiInput) -> ApiResult<Option<i32>> {
    let input = validate_input(api_input)?;
    get_user(input.id)
}

#[cfg(test)]
mod api_tests {
    use super::*;
    use crate::state::{reset_state, DatabaseState, STATE};
    use std::collections::HashMap;

    #[test]
    fn test_happy_path() {
        reset_state();
        let valid_input: ApiInput = HashMap::from([("id".into(), "1".to_string())]);
        let response = get_user_handler(valid_input).unwrap();
        assert_eq!(response, Some(1));
    }

    #[test]
    fn test_invalid_user_id() {
        reset_state();
        let api_input = HashMap::from([("id".into(), "foo".to_string())]);
        let response = get_user_handler(api_input);

        assert!(response.is_err());
        assert_eq!(response.unwrap_err().to_string(), "Not a valid id");
    }

    #[test]
    fn test_user_not_found() {
        reset_state();
        let api_input = HashMap::from([("id".into(), "100".to_string())]);
        let response = get_user_handler(api_input).unwrap();

        assert_eq!(response, None);
    }

    #[test]
    fn test_database_not_reachable() {
        reset_state();
        let mut guard = STATE.lock().unwrap();
        guard.database_state = DatabaseState::Unreachable();
        drop(guard);

        println!("{:?}", STATE.lock().unwrap());

        let valid_input: ApiInput = HashMap::from([("id".into(), "1".to_string())]);
        let response = get_user_handler(valid_input);

        assert!(response.is_err());
        // assert_eq!(response.unwrap_err().to_string(), "Not a valid id");
    }
}
