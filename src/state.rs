// This module is used to simulate a system state and all failures that can occur.
// Regardless of the state, the system should never panic and even if it can not handle a request properly,
// it should return a proper error message.
use lazy_static::lazy_static;
use std::sync::Mutex;

lazy_static! {
    pub static ref STATE: Mutex<SystemState> = Mutex::new(SystemState::default());
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DatabaseState {
    Fine(),
    DatabaseMissing(),
    Unreachable(),
}

impl Default for DatabaseState {
    fn default() -> Self {
        return DatabaseState::Fine();
    }
}

#[derive(Default, Debug)]
pub struct SystemState {
    pub database_state: DatabaseState,
}

pub fn reset_state() {
    let mut guard = STATE.lock().unwrap();
    guard.database_state = DatabaseState::default();
}
