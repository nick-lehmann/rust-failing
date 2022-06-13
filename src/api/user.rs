use crate::service::{
    errors::ServiceError, service::get_user, validation::validate_input, ApiInput,
};

use super::errors::ApiResult;

pub fn get_user_handler(api_input: ApiInput) -> ApiResult<Option<i32>> {
    let input =
        validate_input(api_input).map_err(|e| ServiceError::ValidationError { source: e })?;
    let user = get_user(input.id)?;
    Ok(user)
}

#[cfg(test)]
mod tests {
    use crate::{
        api::errors::ApiError,
        service::service::{FORBIDDEN_ID, VALID_ID},
        state::{reset_state, DatabaseState, STATE},
        utils::{assert_operator_report, assert_user_report, operator_report, user_report},
    };

    use super::*;
    use indoc::indoc;
    use std::collections::HashMap;

    #[test]
    fn test_valid() {
        reset_state();

        let api_input = HashMap::from([("id".into(), VALID_ID.to_string())]);
        let response = get_user_handler(api_input).unwrap();

        assert_eq!(response, Some(10));
    }

    #[test]
    fn test_invalid_input() {
        reset_state();

        let api_input = HashMap::from([("id".into(), "foo".to_string())]);
        let error = get_user_handler(api_input).unwrap_err();
        assert!(matches!(error, ApiError::ValidationError { .. }));

        assert_operator_report!(
            "
            Validation failed

            Caused by:
                Validation failed
            Caused by:
                Invalid value for id: foo
            ",
            error
        );
        assert_user_report!("Validation failed", error);
    }

    #[test]
    fn test_forbidden() {
        reset_state();

        let api_input = HashMap::from([("id".into(), FORBIDDEN_ID.to_string())]);
        let response = get_user_handler(api_input);
        let error = response.unwrap_err();
        assert!(matches!(error, ApiError::Forbidden));
        assert_operator_report!(
            "
            Forbidden

            ",
            error
        );
        assert_user_report!("Forbidden", error);
    }

    #[test]
    fn test_unreachable() {
        reset_state();
        STATE.lock().unwrap().database_state = DatabaseState::Unreachable();

        let api_input = HashMap::from([("id".into(), "10".to_string())]);
        let error = get_user_handler(api_input).unwrap_err();

        assert!(matches!(error, ApiError::Unexpected { .. }));
        assert_operator_report!(
            "
            An unexpected error occurred
            
            Caused by:
                A dependency is unavailable
            Caused by:
                The database timed out
            Caused by:
                Connection to the database timed out
            ",
            error
        );
        assert_user_report!("An unexpected error occurred", error);
    }

    #[test]
    fn test_unexpected() {
        reset_state();
        STATE.lock().unwrap().database_state = DatabaseState::DatabaseMissing();

        let api_input = HashMap::from([("id".into(), VALID_ID.to_string())]);
        let error = get_user_handler(api_input).unwrap_err();

        assert!(matches!(error, ApiError::Unexpected { .. }));
        assert_operator_report!(
            "
            An unexpected error occurred
            
            Caused by:
                Unexpected error
            Caused by:
                The database was missing
            Caused by:
                Database missing
            ",
            error
        );
        assert_user_report!("An unexpected error occurred", error);
    }
}
