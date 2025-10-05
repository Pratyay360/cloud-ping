//! UI utilities for progress bars and display formatting

use indicatif::{MultiProgress, ProgressBar, ProgressStyle};

/// Utility for creating and managing progress bars
pub struct ProgressBarFactory {
    multi_progress: MultiProgress,
}

impl ProgressBarFactory {
    /// Create a new progress bar factory
    pub fn new(multi_progress: MultiProgress) -> Self {
        Self { multi_progress }
    }

    /// Create a standard progress bar for network testing
    pub fn create_test_progress_bar(&self, count: usize, label: &str) -> ProgressBar {
        let pb = self.multi_progress.add(ProgressBar::new(count as u64));
        
        let style = ProgressStyle::default_bar()
            .template("{spinner:.green} {msg} [{bar:30.cyan/blue}] {pos}/{len} ({eta}) {per_sec}")
            .unwrap()
            .progress_chars("█▉▊▋▌▍▎▏  ");
        
        pb.set_style(style);
        pb.set_message(format!("Testing {}", Self::truncate_text(label, 30)));
        pb
    }

    /// Create multiple progress bars for batch operations
    pub fn create_multiple_progress_bars(&self, items: &[(&str, usize)]) -> Vec<ProgressBar> {
        items
            .iter()
            .map(|(label, count)| self.create_test_progress_bar(*count, label))
            .collect()
    }

    /// Truncate text for display with ellipsis
    pub fn truncate_text(text: &str, max_len: usize) -> String {
        if text.len() <= max_len {
            text.to_string()
        } else {
            format!("{}...", &text[..max_len.saturating_sub(3)])
        }
    }
}

/// Utility functions for consistent display formatting
pub struct DisplayUtils;

impl DisplayUtils {
    /// Format a region name for display with consistent truncation
    pub fn format_region_name(name: &str, max_len: usize) -> String {
        ProgressBarFactory::truncate_text(name, max_len)
    }

    /// Create a standard header separator
    pub fn create_separator(length: usize) -> String {
        "=".repeat(length)
    }

    /// Create a standard sub-header separator
    pub fn create_sub_separator(length: usize) -> String {
        "-".repeat(length)
    }

    /// Format a percentage with consistent precision
    pub fn format_percentage(value: f64) -> String {
        crate::format_utils::FormatUtils::format_percentage(value)
    }

    /// Format a latency value with consistent precision
    pub fn format_latency(value: f64) -> String {
        crate::format_utils::FormatUtils::format_latency_ms(value)
    }

    /// Format a score with consistent precision
    pub fn format_score(value: f64) -> String {
        format!("{:.1}", value)
    }
}