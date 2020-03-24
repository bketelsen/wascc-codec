//! # Logging Data Types
//!
//! This module contains data types for the `wascc:logging` capability provider
//! The message set is intentionally a mirror of the log crate's types.

pub const OP_LOG: &str = "Log";

#[derive(Debug, PartialEq, Deserialize, Serialize)]
/// A representation of a log message
pub struct LogRequest {
    /// A string which represents the module or source of the log message
    pub target: String,
    /// level corresponds to the log crate's Level enum where Error = 1 and Trace = 5
    pub level: usize,
    /// A string that represents the body of the log message
    pub body: String,
}