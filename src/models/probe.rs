//! Probe record and alert definitions

use chrono::{DateTime, Utc};
use crate::time_utils::TimeUtils;
use crate::format_utils::FormatUtils;
use serde::{Deserialize, Serialize};

/// Individual probe record from a single test
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ProbeRecord {
    pub endpoint_id: String,        // Unique identifier for the endpoint that was probed
    pub timestamp: DateTime<Utc>,   // Timestamp when the probe was performed
    pub rtt_ms: Option<f64>,        // Round-trip time in milliseconds (None if probe failed)
    pub success: bool,              // Whether the probe was successful
    pub error_code: Option<String>, // Error code if the probe failed
}

impl ProbeRecord {
    /// Create a new probe record
    pub fn new(endpoint_id: String, rtt_ms: Option<f64>, success: bool) -> Self {
        Self {
            endpoint_id,
            timestamp: TimeUtils::now(),
            rtt_ms,
            success,
            error_code: None,
        }
    }

    /// Create a probe record with error
    pub fn with_error(endpoint_id: String, error: String) -> Self {
        Self {
            endpoint_id,
            timestamp: TimeUtils::now(),
            rtt_ms: None,
            success: false,
            error_code: Some(error),
        }
    }

    /// Create a successful probe record
    pub fn success(endpoint_id: String, rtt_ms: f64) -> Self {
        Self {
            endpoint_id,
            timestamp: TimeUtils::now(),
            rtt_ms: Some(rtt_ms),
            success: true,
            error_code: None,
        }
    }

    /// Create a failed probe record
    pub fn failure(endpoint_id: String, error: Option<String>) -> Self {
        Self {
            endpoint_id,
            timestamp: TimeUtils::now(),
            rtt_ms: None,
            success: false,
            error_code: error,
        }
    }

    /// Create a timeout probe record
    pub fn timeout(endpoint_id: String) -> Self {
        Self::failure(endpoint_id, Some("timeout".to_string()))
    }

    /// Check if the probe was successful
    pub fn is_success(&self) -> bool {
        self.success && self.rtt_ms.is_some()
    }

    /// Get RTT or default value
    pub fn rtt_or_default(&self, default: f64) -> f64 {
        self.rtt_ms.unwrap_or(default)
    }
}

/// Alert types for incident detection
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AlertType {
    ScoreDrop { old_score: f64, new_score: f64 },        // Alert for significant score drop
    SustainedLoss { loss_percent: f64 },                 // Alert for sustained packet loss
    AvailabilityLow { availability: f64 },               // Alert for low availability
    HighLatency { latency_ms: f64 },                     // Alert for high latency
    HighJitter { jitter_ms: f64 },                      // Alert for high jitter
}

impl AlertType {
    /// Get alert severity level
    pub fn severity(&self) -> AlertSeverity {
        match self {
            AlertType::ScoreDrop { old_score, new_score } => {
                let drop = old_score - new_score;
                if drop >= 40.0 {
                    AlertSeverity::Critical
                } else if drop >= 20.0 {
                    AlertSeverity::Warning
                } else {
                    AlertSeverity::Info
                }
            }
            AlertType::SustainedLoss { loss_percent } => {
                if *loss_percent >= 10.0 {
                    AlertSeverity::Critical
                } else if *loss_percent >= 3.0 {
                    AlertSeverity::Warning
                } else {
                    AlertSeverity::Info
                }
            }
            AlertType::AvailabilityLow { availability } => {
                if *availability < 90.0 {
                    AlertSeverity::Critical
                } else if *availability < 95.0 {
                    AlertSeverity::Warning
                } else {
                    AlertSeverity::Info
                }
            }
            AlertType::HighLatency { latency_ms } => {
                if *latency_ms > 500.0 {
                    AlertSeverity::Critical
                } else if *latency_ms > 200.0 {
                    AlertSeverity::Warning
                } else {
                    AlertSeverity::Info
                }
            }
            AlertType::HighJitter { jitter_ms } => {
                if *jitter_ms > 100.0 {
                    AlertSeverity::Critical
                } else if *jitter_ms > 50.0 {
                    AlertSeverity::Warning
                } else {
                    AlertSeverity::Info
                }
            }
        }
    }

    /// Get human-readable description
    pub fn description(&self) -> String {
        match self {
            AlertType::ScoreDrop { old_score, new_score } => {
                format!("Score dropped from {:.1} to {:.1}", old_score, new_score)
            }
            AlertType::SustainedLoss { loss_percent } => {
                format!("Sustained packet loss: {}", FormatUtils::format_percentage(*loss_percent))
            }
            AlertType::AvailabilityLow { availability } => {
                format!("Low availability: {}", FormatUtils::format_percentage(*availability))
            }
            AlertType::HighLatency { latency_ms } => {
                format!("High latency: {}", FormatUtils::format_latency_ms(*latency_ms))
            }
            AlertType::HighJitter { jitter_ms } => {
                format!("High jitter: {}", FormatUtils::format_latency_ms(*jitter_ms))
            }
        }
    }
}

/// Alert severity levels
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum AlertSeverity {
    Info,     // Informational alert
    Warning,  // Warning alert requiring attention
    Critical, // Critical alert requiring immediate action
}

impl AlertSeverity {
    /// Get color code for display
    pub fn color_code(&self) -> &'static str {
        match self {
            AlertSeverity::Info => "blue",
            AlertSeverity::Warning => "yellow",
            AlertSeverity::Critical => "red",
        }
    }

    /// Get emoji representation
    pub fn emoji(&self) -> &'static str {
        match self {
            AlertSeverity::Info => "ℹ️",
            AlertSeverity::Warning => "⚠️",
            AlertSeverity::Critical => "🚨",
        }
    }
}

/// Alert with metadata
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Alert {
    pub endpoint_id: String,        // Unique identifier for the endpoint that triggered the alert
    pub alert_type: AlertType,      // Type of alert that was triggered
    pub timestamp: DateTime<Utc>,   // Timestamp when the alert was created
    pub acknowledged: bool,         // Whether the alert has been acknowledged
}

impl Alert {
    /// Create a new alert
    pub fn new(endpoint_id: String, alert_type: AlertType) -> Self {
        Self {
            endpoint_id,
            alert_type,
            timestamp: TimeUtils::now(),
            acknowledged: false,
        }
    }

    /// Get alert severity
    pub fn severity(&self) -> AlertSeverity {
        self.alert_type.severity()
    }

    /// Get alert description
    pub fn description(&self) -> String {
        self.alert_type.description()
    }

    /// Acknowledge the alert
    pub fn acknowledge(&mut self) {
        self.acknowledged = true;
    }

    /// Check if alert is recent (within last hour)
    pub fn is_recent(&self) -> bool {
        TimeUtils::is_recent(&self.timestamp)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_probe_record_creation() {
        let success = ProbeRecord::success("test".to_string(), 50.0);
        assert!(success.is_success());
        assert_eq!(success.rtt_or_default(0.0), 50.0);

        let failure = ProbeRecord::failure("test".to_string(), Some("error".to_string()));
        assert!(!failure.is_success());
        assert_eq!(failure.rtt_or_default(999.0), 999.0);

        let timeout = ProbeRecord::timeout("test".to_string());
        assert!(!timeout.is_success());
        assert_eq!(timeout.error_code, Some("timeout".to_string()));
    }

    #[test]
    fn test_alert_severity() {
        let critical_drop = AlertType::ScoreDrop { old_score: 90.0, new_score: 40.0 };
        assert_eq!(critical_drop.severity(), AlertSeverity::Critical);

        let warning_loss = AlertType::SustainedLoss { loss_percent: 5.0 };
        assert_eq!(warning_loss.severity(), AlertSeverity::Warning);

        let info_jitter = AlertType::HighJitter { jitter_ms: 30.0 };
        assert_eq!(info_jitter.severity(), AlertSeverity::Info);
    }

    #[test]
    fn test_alert_creation() {
        let alert_type = AlertType::HighLatency { latency_ms: 300.0 };
        let alert = Alert::new("test".to_string(), alert_type);
        
        assert_eq!(alert.endpoint_id, "test");
        assert!(!alert.acknowledged);
        assert_eq!(alert.severity(), AlertSeverity::Warning);
        assert!(alert.is_recent());
    }
}