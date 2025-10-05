//! Utility functions for models

use uuid::Uuid;
use statistical::{mean, median, standard_deviation};

/// Generate a new UUID string
pub fn generate_uuid() -> String {
    Uuid::new_v4().to_string()
}

/// Calculate percentile from values (optimized version)
pub fn percentile(values: &[f64], p: f64) -> f64 {
    if values.is_empty() {
        return f64::INFINITY;
    }

    if values.len() == 1 {
        return values[0];
    }

    // Use unstable sort for better performance
    let mut sorted = values.to_vec();
    sorted.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));

    let index = (p / 100.0) * (sorted.len() - 1) as f64;
    let lower = index.floor() as usize;
    let upper = index.ceil() as usize;

    if lower == upper || upper >= sorted.len() {
        sorted[lower.min(sorted.len() - 1)]
    } else {
        let weight = index - lower as f64;
        sorted[lower] * (1.0 - weight) + sorted[upper] * weight
    }
}

/// Calculate multiple percentiles efficiently
pub fn percentiles(values: &[f64], percentiles: &[f64]) -> Vec<f64> {
    if values.is_empty() {
        return vec![f64::INFINITY; percentiles.len()];
    }

    // Sort once and reuse for all percentiles
    let mut sorted = values.to_vec();
    sorted.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));

    percentiles.iter().map(|&p| {
        let index = (p / 100.0) * (sorted.len() - 1) as f64;
        let lower = index.floor() as usize;
        let upper = index.ceil() as usize;

        if lower == upper || upper >= sorted.len() {
            sorted[lower.min(sorted.len() - 1)]
        } else {
            let weight = index - lower as f64;
            sorted[lower] * (1.0 - weight) + sorted[upper] * weight
        }
    }).collect()
}

/// Calculate basic statistics from values
#[derive(Debug, Clone, PartialEq)]
pub struct BasicStats {
    /// Number of values in the dataset
    pub count: usize,
    /// Minimum value
    pub min: f64,
    /// Maximum value
    pub max: f64,
    /// Arithmetic mean
    pub mean: f64,
    /// Median value
    pub median: f64,
    /// Standard deviation
    pub std_dev: f64,
}

impl BasicStats {
    /// Calculate basic statistics from values using statistical crate
    pub fn from_values(values: &[f64]) -> Self {
        if values.is_empty() {
            return Self {
                count: 0,
                min: f64::NAN,
                max: f64::NAN,
                mean: f64::NAN,
                median: f64::NAN,
                std_dev: f64::NAN,
            };
        }

        let count = values.len();
        let min_val = values.iter().fold(f64::INFINITY, |a, &b| a.min(b));
        let max_val = values.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));
        let mean_val = mean(values);
        let median_val = median(values);
        let std_dev_val = standard_deviation(values, Some(mean_val));

        Self {
            count,
            min: min_val,
            max: max_val,
            mean: mean_val,
            median: median_val,
            std_dev: std_dev_val,
        }
    }
}

/// Exponential weighted moving average calculator
#[derive(Debug, Clone)]
pub struct EWMA {
    alpha: f64,
    value: Option<f64>,
}

impl EWMA {
    /// Create new EWMA with smoothing factor
    pub fn new(alpha: f64) -> Self {
        Self {
            alpha: alpha.clamp(0.0, 1.0),
            value: None,
        }
    }

    /// Update with new value
    pub fn update(&mut self, new_value: f64) {
        match self.value {
            None => self.value = Some(new_value),
            Some(current) => {
                self.value = Some(current + self.alpha * (new_value - current));
            }
        }
    }

    /// Get current value
    pub fn value(&self) -> Option<f64> {
        self.value
    }

    /// Reset to initial state
    pub fn reset(&mut self) {
        self.value = None;
    }
}

/// Time-based rate limiter
#[derive(Debug)]
pub struct RateLimiter {
    last_time: std::time::Instant,
    min_interval: std::time::Duration,
}

impl RateLimiter {
    /// Create new rate limiter
    pub fn new(min_interval: std::time::Duration) -> Self {
        Self {
            last_time: std::time::Instant::now() - min_interval,
            min_interval,
        }
    }

    /// Check if action is allowed
    pub fn is_allowed(&mut self) -> bool {
        let now = std::time::Instant::now();
        if now.duration_since(self.last_time) >= self.min_interval {
            self.last_time = now;
            true
        } else {
            false
        }
    }

    /// Get time until next allowed action
    pub fn time_until_allowed(&self) -> std::time::Duration {
        let elapsed = std::time::Instant::now().duration_since(self.last_time);
        if elapsed >= self.min_interval {
            std::time::Duration::ZERO
        } else {
            self.min_interval - elapsed
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_percentile_calculation() {
        let values = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        
        assert_eq!(percentile(&values, 0.0), 1.0);
        assert_eq!(percentile(&values, 50.0), 3.0);
        assert_eq!(percentile(&values, 100.0), 5.0);
        
        // Test empty slice
        assert!(percentile(&[], 50.0).is_infinite());
    }

    #[test]
    fn test_multiple_percentiles() {
        let values = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0];
        let percs = vec![25.0, 50.0, 75.0, 90.0, 95.0];
        
        let results = percentiles(&values, &percs);
        assert_eq!(results.len(), 5);
        assert_eq!(results[1], 5.5); // 50th percentile
    }

    #[test]
    fn test_basic_stats() {
        let values = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let stats = BasicStats::from_values(&values);
        
        assert_eq!(stats.count, 5);
        assert_eq!(stats.min, 1.0);
        assert_eq!(stats.max, 5.0);
        assert_eq!(stats.mean, 3.0);
        assert_eq!(stats.median, 3.0);
    }

    #[test]
    fn test_ewma() {
        let mut ewma = EWMA::new(0.1);
        
        ewma.update(10.0);
        assert_eq!(ewma.value(), Some(10.0));
        
        ewma.update(20.0);
        assert_eq!(ewma.value(), Some(11.0)); // 10 + 0.1 * (20 - 10)
    }

    #[test]
    fn test_rate_limiter() {
        let mut limiter = RateLimiter::new(crate::time_utils::TimeUtils::duration_from_millis(100));
        
        assert!(limiter.is_allowed());
        assert!(!limiter.is_allowed()); // Too soon
        
        std::thread::sleep(crate::time_utils::TimeUtils::duration_from_millis(101));
        assert!(limiter.is_allowed());
    }
}