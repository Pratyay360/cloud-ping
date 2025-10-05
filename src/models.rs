//! Core data models - now organized into submodules for better maintainability

// Re-export all public types from submodules
pub use self::endpoint::{Endpoint, ProbeType};
pub use self::metrics::{AggregatorState, AggregatorStateBuilder, HealthStatus, RingBuffer};
pub use self::probe::{Alert, AlertSeverity, AlertType, ProbeRecord};
pub use self::region::{CloudProvider, Coordinates, Region};
pub use self::scoring::{AlgorithmWeights, ComprehensiveScoreResult, ScoreComponents};
pub use self::scoring::utils::ScoringAdapter;
pub use self::stats::{PerformanceSummary, PingStats, TestHistory};

// Submodules
pub mod endpoint;
pub mod metrics;
pub mod probe;
pub mod region;
pub mod scoring;
pub mod stats;
pub mod utils;