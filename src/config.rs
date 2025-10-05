//! Application configuration management
//!
//! Handles loading configuration from multiple sources with validation.

use config::{Config, ConfigError, Environment, File};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::time::Duration;

use crate::error::{CloudPingError, Result};

/// Application configuration with defaults and validation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    /// Default number of pings for comprehensive tests
    pub default_ping_count: usize,
    /// Number of pings for quick tests
    pub quick_ping_count: usize,
    /// Request timeout in milliseconds
    pub timeout_ms: u64,
    /// Request timeout as human-readable duration (e.g., "5s", "500ms")
    #[serde(with = "humantime_serde", default = "default_timeout")]
    pub timeout: Duration,
    /// Maximum number of concurrent threads
    pub max_threads: usize,
    /// Enable colored terminal output
    pub enable_color_output: bool,
    /// Save results to file automatically
    pub save_results_to_file: bool,
    /// Filename for results output
    pub results_filename: String,
    /// Data file path
    pub data_file: String,
    /// Enable progress bars
    pub show_progress: bool,
    /// Retry attempts for failed requests
    pub retry_attempts: usize,
    /// Delay between retries in milliseconds
    pub retry_delay_ms: u64,
    /// Delay between retries as human-readable duration (e.g., "100ms", "1s")
    #[serde(with = "humantime_serde", default = "default_retry_delay")]
    pub retry_delay: Duration,
    /// Enable detailed logging
    pub verbose: bool,
    /// Output format (json, table, csv)
    pub output_format: OutputFormat,
    /// HTTP user agent string
    pub user_agent: String,
    /// Enable TLS certificate validation
    pub validate_certificates: bool,
}

fn default_timeout() -> Duration {
    Duration::from_millis(5000)
}

fn default_retry_delay() -> Duration {
    Duration::from_millis(100)
}

/// Supported output formats for test results
#[derive(Debug, Clone, Serialize, Deserialize, clap::ValueEnum)]
#[serde(rename_all = "lowercase")]
pub enum OutputFormat {
    Json,
    Table,
    Csv,
}

impl Default for OutputFormat {
    fn default() -> Self {
        Self::Table
    }
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            default_ping_count: 10,
            quick_ping_count: 3,
            timeout_ms: 5000,
            timeout: default_timeout(),
            max_threads: 10,
            enable_color_output: true,
            save_results_to_file: true,
            results_filename: "connection_benchmark_results.json".to_string(),
            data_file: "data.json".to_string(),
            show_progress: true,
            retry_attempts: 2,
            retry_delay_ms: 100,
            retry_delay: default_retry_delay(),
            verbose: false,
            output_format: OutputFormat::default(),
            user_agent: format!("cloud-ping-rs/{}", env!("CARGO_PKG_VERSION")),
            validate_certificates: false,
        }
    }
}

impl AppConfig {
    /// Load configuration from multiple sources with precedence
    /// 
    /// Sources (highest to lowest precedence):
    /// 1. Environment variables (CLOUD_PING_*)
    /// 2. Config file (~/.config/cloud-ping-rs/config.toml)
    /// 3. Built-in defaults
    pub fn load() -> Result<Self> {
        let mut config = Config::builder()
            .add_source(Config::try_from(&AppConfig::default())?)
            .add_source(Environment::with_prefix("CLOUD_PING").separator("_"));

        // Try to load from config file
        if let Some(config_path) = Self::get_config_path() {
            if config_path.exists() {
                config = config.add_source(File::from(config_path));
            }
        }

        config
            .build()
            .and_then(|c| c.try_deserialize())
            .map_err(|e| CloudPingError::config(format!("Failed to load configuration: {}", e)))
    }

    fn get_config_path() -> Option<PathBuf> {
        dirs::config_dir().map(|mut path| {
            path.push("cloud-ping-rs");
            path.push("config.toml");
            path
        })
    }

    /// Persist configuration to default location
    pub fn save(&self) -> Result<()> {
        let config_path = Self::get_config_path()
            .ok_or_else(|| CloudPingError::config("Cannot determine config directory"))?;

        if let Some(parent) = config_path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        let toml_string = toml::to_string_pretty(self)
            .map_err(|e| CloudPingError::config(format!("Failed to serialize config: {}", e)))?;

        std::fs::write(&config_path, toml_string)?;
        Ok(())
    }

    /// Validate configuration constraints
    pub fn validate(&self) -> Result<()> {
        if self.default_ping_count == 0 {
            return Err(CloudPingError::validation(
                "default_ping_count",
                "must be greater than 0",
            ));
        }

        if self.quick_ping_count == 0 {
            return Err(CloudPingError::validation(
                "quick_ping_count",
                "must be greater than 0",
            ));
        }

        if self.timeout_ms == 0 {
            return Err(CloudPingError::validation(
                "timeout_ms",
                "must be greater than 0",
            ));
        }

        if self.max_threads == 0 {
            return Err(CloudPingError::validation(
                "max_threads",
                "must be greater than 0",
            ));
        }

        if self.max_threads > 100 {
            return Err(CloudPingError::validation(
                "max_threads",
                "should not exceed 100 for stability",
            ));
        }

        Ok(())
    }

    /// Get timeout as Duration (preferred over timeout_ms)
    pub fn get_timeout(&self) -> Duration {
        self.timeout
    }

    /// Get retry delay as Duration (preferred over retry_delay_ms)
    pub fn get_retry_delay(&self) -> Duration {
        self.retry_delay
    }

    /// Set timeout from human-readable string (e.g., "5s", "500ms")
    pub fn set_timeout_from_str(&mut self, timeout_str: &str) -> Result<()> {
        self.timeout = humantime::parse_duration(timeout_str)
            .map_err(|e| CloudPingError::config(format!("Invalid timeout format: {}", e)))?;
        self.timeout_ms = self.timeout.as_millis() as u64;
        Ok(())
    }

    /// Set retry delay from human-readable string (e.g., "100ms", "1s")
    pub fn set_retry_delay_from_str(&mut self, delay_str: &str) -> Result<()> {
        self.retry_delay = humantime::parse_duration(delay_str)
            .map_err(|e| CloudPingError::config(format!("Invalid retry delay format: {}", e)))?;
        self.retry_delay_ms = self.retry_delay.as_millis() as u64;
        Ok(())
    }
}



impl TryFrom<&AppConfig> for Config {
    type Error = ConfigError;

    fn try_from(app_config: &AppConfig) -> std::result::Result<Self, Self::Error> {
        Config::builder()
            .set_default("default_ping_count", app_config.default_ping_count as i64)?
            .set_default("quick_ping_count", app_config.quick_ping_count as i64)?
            .set_default("timeout_ms", app_config.timeout_ms as i64)?
            .set_default("max_threads", app_config.max_threads as i64)?
            .set_default("enable_color_output", app_config.enable_color_output)?
            .set_default("save_results_to_file", app_config.save_results_to_file)?
            .set_default("results_filename", app_config.results_filename.as_str())?
            .set_default("data_file", app_config.data_file.as_str())?
            .set_default("show_progress", app_config.show_progress)?
            .set_default("retry_attempts", app_config.retry_attempts as i64)?
            .set_default("retry_delay_ms", app_config.retry_delay_ms as i64)?
            .set_default("verbose", app_config.verbose)?
            .set_default("user_agent", app_config.user_agent.as_str())?
            .set_default("validate_certificates", app_config.validate_certificates)?
            .build()
    }
}