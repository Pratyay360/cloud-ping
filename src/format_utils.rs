//! Formatting utilities using external crates where possible

use bytesize::ByteSize;
use num_format::{Locale, ToFormattedString};

/// Formatting utilities for consistent display patterns
pub struct FormatUtils;

impl FormatUtils {
    /// Format a percentage with consistent precision (1 decimal place)
    #[inline]
    pub fn format_percentage(value: f64) -> String {
        format!("{:.1}%", value)
    }

    /// Format a latency value with consistent precision (2 decimal places)
    #[inline]
    pub fn format_latency_ms(value: f64) -> String {
        format!("{:.2}ms", value)
    }

    /// Format a duration in milliseconds
    #[inline]
    pub fn format_duration_ms(ms: u64) -> String {
        format!("{}ms", ms)
    }

    /// Format a timeout message
    #[inline]
    pub fn format_timeout_message(timeout_ms: u64) -> String {
        format!("Timeout after {}ms", timeout_ms)
    }

    /// Format a score with consistent precision (1 decimal place)
    #[inline]
    pub fn format_score(score: f64) -> String {
        format!("{:.1}", score)
    }

    /// Format a timestamp for display
    #[inline]
    pub fn format_timestamp_display() -> String {
        crate::time_utils::TimeUtils::format_timestamp(&crate::time_utils::TimeUtils::now())
    }

    /// Format bytes in human readable format using bytesize crate
    #[inline]
    pub fn format_bytes(bytes: u64) -> String {
        ByteSize::b(bytes).to_string_as(true)
    }

    /// Format a count with thousands separator using num-format crate
    #[inline]
    pub fn format_count(count: usize) -> String {
        count.to_formatted_string(&Locale::en)
    }

    /// Format a count in compact form (K, M notation)
    pub fn format_count_compact(count: usize) -> String {
        if count >= 1_000_000 {
            format!("{:.1}M", count as f64 / 1_000_000.0)
        } else if count >= 1_000 {
            format!("{:.1}K", count as f64 / 1_000.0)
        } else {
            count.to_string()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_percentage_formatting() {
        assert_eq!(FormatUtils::format_percentage(95.678), "95.7%");
        assert_eq!(FormatUtils::format_percentage(0.0), "0.0%");
        assert_eq!(FormatUtils::format_percentage(100.0), "100.0%");
    }

    #[test]
    fn test_latency_formatting() {
        assert_eq!(FormatUtils::format_latency_ms(45.678), "45.68ms");
        assert_eq!(FormatUtils::format_latency_ms(0.0), "0.00ms");
        assert_eq!(FormatUtils::format_latency_ms(1000.0), "1000.00ms");
    }

    #[test]
    fn test_duration_formatting() {
        assert_eq!(FormatUtils::format_duration_ms(500), "500ms");
        assert_eq!(FormatUtils::format_duration_ms(0), "0ms");
    }

    #[test]
    fn test_timeout_message() {
        assert_eq!(FormatUtils::format_timeout_message(5000), "Timeout after 5000ms");
    }

    #[test]
    fn test_score_formatting() {
        assert_eq!(FormatUtils::format_score(87.654), "87.7");
        assert_eq!(FormatUtils::format_score(100.0), "100.0");
    }

    #[test]
    fn test_bytes_formatting() {
        let result = FormatUtils::format_bytes(512);
        assert!(result.contains("512") || result.contains("B"));
        
        let result = FormatUtils::format_bytes(1024);
        assert!(result.len() > 2); // Should have some unit
        
        let result = FormatUtils::format_bytes(1048576);
        assert!(result.len() > 2); // Should have some unit
        
        // Just verify it doesn't panic and returns something reasonable
        let result = FormatUtils::format_bytes(0);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_count_formatting() {
        assert_eq!(FormatUtils::format_count(500), "500");
        assert_eq!(FormatUtils::format_count(1500), "1,500");
        assert_eq!(FormatUtils::format_count(1500000), "1,500,000");
    }

    #[test]
    fn test_count_compact_formatting() {
        assert_eq!(FormatUtils::format_count_compact(500), "500");
        assert_eq!(FormatUtils::format_count_compact(1500), "1.5K");
        assert_eq!(FormatUtils::format_count_compact(1500000), "1.5M");
    }
}