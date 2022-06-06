use std::collections::HashMap;

pub mod errors;
pub mod service;
pub mod validation;

pub type ApiInput = HashMap<String, String>;

#[derive(Debug, PartialEq, Eq)]
pub struct InputData {
    pub id: i32,
}
