use super::{
    errors::{ServiceError, ServiceResult},
    ApiInput, InputData,
};

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
pub fn validate_input(input: ApiInput) -> ServiceResult<InputData> {
    let id_string = input
        .get("id")
        .ok_or(ServiceError::ValidationError("No value 'id' found".into()))?;

    let id: i32 = id_string
        .parse()
        .map_err(|e| ServiceError::ValidationError("Not a valid id".into()))?;

    Ok(InputData { id: id })
}
