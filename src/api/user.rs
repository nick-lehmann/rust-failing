use crate::service::{service::get_user, validation::validate_input, ApiInput};

use super::errors::ApiResult;

fn get_user_handler(api_input: ApiInput) -> ApiResult<Option<i32>> {
    let input = validate_input(api_input)?;
    let user = get_user(input.id)?;
    Ok(user)
}

#[cfg(test)]
mod tests {
    use crate::{
        api::errors::ApiError,
        state::{reset_state, DatabaseState, STATE},
    };

    use super::*;
    use indoc::indoc;
    use std::collections::HashMap;

    fn error_to_debug_string<E>(e: &E) -> String
    where
        E: std::fmt::Debug,
    {
        format!("{:?}", e).replace("\t", "    ")
    }

    fn error_to_display_string<E>(e: &E) -> String
    where
        E: std::fmt::Display,
    {
        format!("{}", e).replace("\t", "    ")
    }

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
        let error = get_user_handler(api_input).unwrap_err();

        let msg = indoc! {"
            Validation failed

            Caused by:
                Invalid value for id: foo
        "};

        match error {
            ApiError::ValidationError(_) => {
                assert_eq!(msg, error_to_debug_string(&error))
            }
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
        let error = get_user_handler(api_input).unwrap_err();

        let operator_message = indoc! {"
        An unexpected error occurred

        Caused by:
            The database timed out
        Caused by:
            Timeout
        "};

        let user_message = "An unexpected error occurred";

        match error {
            ApiError::Unexpected(_) => "",
            _ => panic!("Expected Unexpected, instead returned: {:?}", error),
        };

        assert_eq!(operator_message, error_to_debug_string(&error));
        assert_eq!(user_message, error_to_display_string(&error));
    }

    #[test]
    fn test_unexpected() {
        reset_state();
        STATE.lock().unwrap().database_state = DatabaseState::DatabaseMissing();

        let api_input = HashMap::from([("id".into(), "10".to_string())]);
        let error = get_user_handler(api_input).unwrap_err();

        let operator_message = indoc! {"
            An unexpected error occurred

            Caused by:
                The database was missing
            Caused by:
                Database missing
        "};

        assert_eq!(error_to_debug_string(&error), operator_message);

        match error {
            ApiError::Unexpected(_) => {
                assert_eq!(operator_message, error_to_debug_string(&error))
            }
            _ => panic!("Expected Unexpected, instead returned: {:?}", error),
        };
    }
}
