use std::{collections::HashMap, num::ParseIntError};

use snafu::prelude::*;

type DynError = Box<dyn std::error::Error + Send + Sync>;

#[derive(Debug, Snafu)]
enum ValidationError {
    #[snafu(display("A required key is missing: {key}"))]
    MissingKey { key: &'static str },
    #[snafu(display("Not a valid id: {id}"))]
    InvalidID { id: String, source: ParseIntError },
}

#[derive(Debug, Snafu)]
enum ServiceError {
    #[snafu(display("Invalid data was passed"))]
    Invalid { source: ValidationError },
    #[snafu(display("An operation failed but might succeed later"))]
    Recoverable { source: DynError },
    #[snafu(display("An unexpected error occurred"))]
    Unexpected { source: DynError },
}

#[derive(Snafu, Debug)]
pub enum DatabaseError {
    NotFound,
    DatabaseMissing { name: String },
    Timeout,
}

impl From<DatabaseError> for ServiceError {
    fn from(error: DatabaseError) -> Self {
        match &error {
            DatabaseError::NotFound => {
                unreachable!("NotFound should always be converted into `None`")
            }
            DatabaseError::DatabaseMissing { .. } => ServiceError::Unexpected {
                source: Box::new(error),
            },
            DatabaseError::Timeout => ServiceError::Recoverable {
                source: Box::new(error),
            },
        }
    }
}

/// Checks if the given input contains an ID and if the ID is numeric.
fn validate_input(input: HashMap<String, String>) -> Result<i32, ValidationError> {
    let id = input.get("id").context(MissingKeySnafu { key: "id" })?;
    id.parse::<i32>().context(InvalidIDSnafu { id })
}

pub fn main() {
    let input = HashMap::from([("id".to_string(), "foo".to_string())]);
    let result = validate_input(input);
    println!("{:#?}", result);
}
