#![allow(dead_code, unused_variables)]
#![feature(once_cell)]
/// All operations in this file are part of the library.
/// Therefore, their errors can be used for control flow. We therefore have to find return concrete errors with a wise granularity.
///
pub mod api;
pub mod external;
pub mod http;
pub mod service;
pub mod state;
