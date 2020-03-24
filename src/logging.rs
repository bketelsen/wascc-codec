//! # Logging Data Types
//!
//! This module contains data types for the `wascc:logging` capability provider

pub const OP_LOG: &str = "WriteLog";

#[derive(Debug, PartialEq, Deserialize, Serialize)]
/// A representation of a request to write a log entry
pub struct WriteLogRequest {
    /// A string which represents the actor source of the log message
    pub actor: String,
    /// level corresponds to the log level 
    ///
    /// "OFF"=0 , "ERROR"=1, "WARN"=2, "INFO"=3, "DEBUG"=4, "TRACE"=5
    pub level: usize,
    /// A string that represents the body of the log message
    pub body: String,
}