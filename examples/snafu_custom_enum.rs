use std::{collections::HashMap, num::ParseIntError};

use snafu::prelude::*;

#[derive(Debug, Snafu)]
enum ValidationError {
    #[snafu(display("A required key is missing: {key}"))]
    MissingKey { key: &'static str },
    #[snafu(display("Not a valid id: {id}"))]
    InvalidID { id: String, source: ParseIntError },
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
