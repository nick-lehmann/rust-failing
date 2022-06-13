use snafu::{OptionExt, Snafu};

use super::{ApiInput, InputData};

type FieldName = &'static str;

// #[derive(Debug, thiserror::Error)]
// pub enum ValidationError {
//     #[error("Missing field: {0}")]
//     Missing(FieldName),
//     #[error("Invalid value for {0}: {1}")]
//     Invalid(FieldName, String),
// }

#[derive(Debug, Snafu)]
pub enum ValidationError {
    #[snafu(display("Missing field: {field}"))]
    Missing { field: FieldName },
    #[snafu(display("Invalid value for {field}: {value}"))]
    Invalid { field: FieldName, value: String },
}

// impl From<ValidationError> for ServiceError {
//     fn from(error: ValidationError) -> Self {
//         return ServiceError::ValidationError(error.into());
//     }
// }

/// Validate input received by the user.
///
/// # Examples
/// ```
/// # use std::collections::HashMap;
/// # use failing::service::{InputData, validation::validate_input};
/// let input = HashMap::from([("id".to_string(), "1".to_string())]);
/// let data = validate_input(input).unwrap();
/// assert_eq!(data, InputData { id: 1 });
/// ```
pub fn validate_input(input: ApiInput) -> Result<InputData, ValidationError> {
    let id_string = input.get("id").context(MissingSnafu { field: "id" })?;

    let id: i32 = id_string.parse().map_err(|_| ValidationError::Invalid {
        field: "id",
        value: id_string.to_owned(),
    })?;

    Ok(InputData { id: id })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::reset_state;
    use std::collections::HashMap;

    #[test]
    fn test_valid() {
        reset_state();

        let api_input = HashMap::from([("id".into(), "1".to_string())]);
        let response = validate_input(api_input).unwrap();

        assert_eq!(response, InputData { id: 1 });
    }

    #[test]
    fn test_missing_key() {
        reset_state();

        // TODO: Check for exact error.
        // let expected = ServiceError::ValidationError("Invalid user id".to_string());

        let api_input = HashMap::from([("bar".into(), "foo".to_string())]);
        let response = validate_input(api_input);
        let error = response.unwrap_err();

        // match error {
        //     ServiceError::ValidationError(_) => "",
        //     _ => panic!("Expected ValidationError, instead returned: {:?}", error),
        // };
    }

    #[test]
    fn test_invalid_user_id() {
        reset_state();

        // TODO: Check for exact error.
        // let expected = ServiceError::ValidationError("Invalid user id".to_string());

        let api_input = HashMap::from([("id".into(), "foo".to_string())]);
        let response = validate_input(api_input);
        let error = response.unwrap_err();

        // match error {
        //     ServiceError::ValidationError(_) => "",
        //     _ => panic!("Expected ValidationError, instead returned: {:?}", error),
        // };
    }
}
