//! Time and duration utilities - thin wrappers for consistency

use chrono::{DateTime, Utc};
use std::time::Duration;

/// Time utilities - mostly re-exports for consistency
pub struct TimeUtils;

impl TimeUtils {
    /// Get current UTC timestamp
    #[inline]
    pub fn now() -> DateTime<Utc> {
        Utc::now()
    }

    /// Create duration from milliseconds
    #[inline]
    pub const fn duration_from_millis(ms: u64) -> Duration {
        Duration::from_millis(ms)
    }

    /// Create duration from seconds
    #[inline]
    pub const fn duration_from_secs(secs: u64) -> Duration {
        Duration::from_secs(secs)
    }

    /// Format timestamp for display
    #[inline]
    pub fn format_timestamp(timestamp: &DateTime<Utc>) -> String {
        timestamp.format("%Y-%m-%d %H:%M:%S UTC").to_string()
    }

    /// Check if a timestamp is recent (within the last hour)
    #[inline]
    pub fn is_recent(timestamp: &DateTime<Utc>) -> bool {
        Utc::now().signed_duration_since(*timestamp).num_hours() < 1
    }

    /// Get minimum duration (100ms floor)
    #[inline]
    pub const fn min_duration(duration: Duration) -> Duration {
        if duration.as_millis() < 100 {
            Duration::from_millis(100)
        } else {
            duration
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_duration_creation() {
        let ms_duration = TimeUtils::duration_from_millis(500);
        assert_eq!(ms_duration.as_millis(), 500);

        let sec_duration = TimeUtils::duration_from_secs(5);
        assert_eq!(sec_duration.as_secs(), 5);
    }

    #[test]
    fn test_timestamp_formatting() {
        let now = TimeUtils::now();
        let formatted = TimeUtils::format_timestamp(&now);
        assert!(formatted.contains("UTC"));
        assert!(formatted.len() > 10);
    }

    #[test]
    fn test_recent_check() {
        let now = TimeUtils::now();
        assert!(TimeUtils::is_recent(&now));
        
        // Test with old timestamp (more than 1 hour ago)
        let old_timestamp = now - chrono::Duration::hours(2);
        assert!(!TimeUtils::is_recent(&old_timestamp));
    }

    #[test]
    fn test_min_duration() {
        let short_duration = Duration::from_millis(50);
        let min_duration = TimeUtils::min_duration(short_duration);
        assert_eq!(min_duration.as_millis(), 100);

        let long_duration = Duration::from_millis(500);
        let unchanged = TimeUtils::min_duration(long_duration);
        assert_eq!(unchanged.as_millis(), 500);
    }
}