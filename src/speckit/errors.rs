//! Spec-Kit Error Types

use thiserror::Error;

/// Errors that can occur when interacting with spec-kit CLI
#[derive(Error, Debug)]
pub enum SpecKitError {
    #[error("Spec-kit CLI not found. Please install with: uv tool install specify-cli")]
    CliNotFound,

    #[error("Python 3.11+ required but not found")]
    PythonVersionTooOld,

    #[error("Spec-kit command failed: {command}\nStderr: {stderr}\nExit code: {exit_code}")]
    CommandFailed {
        command: String,
        stderr: String,
        exit_code: i32,
    },

    #[error("Failed to parse spec-kit output: {0}")]
    ParseError(String),

    #[error("Timeout waiting for spec-kit command")]
    Timeout,

    #[error("Invalid path: {0}")]
    InvalidPath(String),

    #[error("File operation failed: {0}")]
    FileError(String),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}

impl SpecKitError {
    /// Create a command failed error
    pub fn command_failed(command: impl Into<String>, stderr: impl Into<String>, exit_code: i32) -> Self {
        Self::CommandFailed {
            command: command.into(),
            stderr: stderr.into(),
            exit_code,
        }
    }
}
