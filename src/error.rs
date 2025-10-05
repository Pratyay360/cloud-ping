//! Error types and handling utilities
//!
//! Provides structured error types with context for better debugging.

use std::fmt;
use thiserror::Error;

/// Application error types with structured context
#[derive(Error, Debug)]
pub enum CloudPingError {
    #[error("Network error: {0}")]
    Network(#[from] reqwest::Error),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("JSON parsing error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("Configuration error: {0}")]
    ConfigError(#[from] config::ConfigError),

    #[error("Configuration error: {message}")]
    Config { message: String },

    #[error("Data loading error: {message}")]
    DataLoading { message: String },

    #[error("Invalid URL: {url}")]
    InvalidUrl { url: String },

    #[error("Test execution error: {message}")]
    TestExecution { message: String },

    #[error("Timeout error: operation timed out after {timeout_ms}ms")]
    Timeout { timeout_ms: u64 },

    #[error("Validation error: {field} - {message}")]
    Validation { field: String, message: String },

    #[error("Concurrent execution error: {message}")]
    Concurrency { message: String },
}

impl CloudPingError {
    #[must_use]
    pub fn config(message: impl Into<String>) -> Self {
        Self::Config {
            message: message.into(),
        }
    }

    #[must_use]
    pub fn data_loading(message: impl Into<String>) -> Self {
        Self::DataLoading {
            message: message.into(),
        }
    }

    #[must_use]
    pub fn invalid_url(url: impl Into<String>) -> Self {
        Self::InvalidUrl { url: url.into() }
    }

    #[must_use]
    pub fn test_execution(message: impl Into<String>) -> Self {
        Self::TestExecution {
            message: message.into(),
        }
    }

    #[must_use]
    pub const fn timeout(timeout_ms: u64) -> Self {
        Self::Timeout { timeout_ms }
    }

    #[must_use]
    pub fn validation(field: impl Into<String>, message: impl Into<String>) -> Self {
        Self::Validation {
            field: field.into(),
            message: message.into(),
        }
    }

    #[must_use]
    pub fn concurrency(message: impl Into<String>) -> Self {
        Self::Concurrency {
            message: message.into(),
        }
    }

    #[must_use]
    pub fn network(message: impl Into<String>) -> Self {
        Self::TestExecution {
            message: message.into(),
        }
    }

    #[must_use]
    pub fn system(message: impl Into<String>) -> Self {
        Self::TestExecution {
            message: message.into(),
        }
    }
}

/// Result type alias for convenience
pub type Result<T> = std::result::Result<T, CloudPingError>;

/// Extension trait for adding context to errors
pub trait ErrorContext<T> {
    fn with_context(self, context: &str) -> Result<T>;
}

impl<T, E> ErrorContext<T> for std::result::Result<T, E>
where
    E: fmt::Display,
{
    fn with_context(self, context: &str) -> Result<T> {
        self.map_err(|e| CloudPingError::test_execution(format!("{}: {}", context, e)))
    }
}

impl From<anyhow::Error> for CloudPingError {
    fn from(err: anyhow::Error) -> Self {
        CloudPingError::test_execution(format!("Operation failed: {}", err))
    }
}