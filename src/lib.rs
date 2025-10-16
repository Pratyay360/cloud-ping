//! Network performance testing library for cloud infrastructure
//!
//! Provides concurrent testing capabilities across multiple cloud providers with
//! comprehensive scoring algorithms and detailed reporting.
//!
//! # Core Features
//! - Multi-threaded concurrent testing
//! - Application-specific suitability scoring
//! - Historical trend analysis
//! - Multiple output formats (JSON, CSV, table)
//!
//! # Example
//! ```rust
//! use cloud_ping::{AppConfig, ConnectionBenchmark};
//!
//! # async fn example() -> cloud_ping::Result<()> {
//! let config = AppConfig::default();
//! let mut benchmark = ConnectionBenchmark::new(config)?;
//! benchmark.load_cloud_providers("data.json").await?;
//! let results = benchmark.run_filtered_benchmark(10, None, None).await?;
//! # Ok(())
//! # }
//! ```

pub mod config;
pub mod error;
pub mod models;
pub mod benchmark;
pub mod display;
pub mod data_loader;
pub mod network;
pub mod probe;
pub mod aggregator;
pub mod monitoring;
pub mod ui_utils;
pub mod time_utils;
pub mod collection_utils;
pub mod format_utils;

#[cfg(test)]
mod tests;

// Re-export commonly used types
pub use config::{AppConfig, OutputFormat};
pub use error::{CloudPingError, ErrorContext, Result};
pub use models::{
    CloudProvider, Coordinates, PingStats, Region, TestHistory, PerformanceSummary,
    Endpoint, ProbeType, AggregatorState, AggregatorStateBuilder, Alert, AlertType, ProbeRecord,
    AlgorithmWeights, ComprehensiveScoreResult, ScoreComponents, HealthStatus, ScoringAdapter
};
pub use ui_utils::{ProgressBarFactory, DisplayUtils};
pub use benchmark::ConnectionBenchmark;
pub use display::DisplayFormatter;
pub use data_loader::DataLoader;
pub use network::NetworkTester;
pub use monitoring::NetworkMonitoringSystem;
pub use probe::ProbeRunner;
pub use aggregator::StreamingAggregator;

/// Library version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Default user agent string
pub const USER_AGENT: &str = concat!("cloud-ping-rs/", env!("CARGO_PKG_VERSION"));

/// Maximum recommended concurrent connections
pub const MAX_CONCURRENT_CONNECTIONS: usize = 50;

/// Default timeout for HTTP requests in milliseconds
pub const DEFAULT_TIMEOUT_MS: u64 = 5000;