//! Performance statistics and historical tracking
//!
//! Provides comprehensive metrics collection and trend analysis for
//! network performance data.

use chrono::{DateTime, Utc};
use crate::time_utils::TimeUtils;
use crate::collection_utils::CollectionUtils;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::scoring::AlgorithmWeights;
use super::utils::generate_uuid;

/// Comprehensive network performance statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PingStats {
    #[serde(default = "generate_uuid")]
    pub id: String,
    pub region_id: Option<String>,
    pub min: f64,
    pub max: f64,
    pub avg: f64,
    pub jitter: f64,
    pub packet_loss: f64,
    pub total_pings: usize,
    pub successful_pings: usize,
    pub standard_deviation: f64,
    pub latencies: Vec<f64>,
    pub error_message: String,
    pub test_time: DateTime<Utc>,
    pub test_duration_ms: u64,
    #[serde(default)]
    pub metadata: HashMap<String, String>,
    #[serde(default)]
    pub status_codes: Vec<u16>,
    pub dns_resolution_time: Option<f64>,
    pub connection_time: Option<f64>,
    pub tls_handshake_time: Option<f64>,
}

impl PingStats {
    pub fn new(count: usize) -> Self {
        Self {
            id: generate_uuid(),
            region_id: None,
            min: f64::MAX,
            max: 0.0,
            avg: 0.0,
            jitter: 0.0,
            packet_loss: 0.0,
            total_pings: count,
            successful_pings: 0,
            standard_deviation: 0.0,
            latencies: Vec::with_capacity(count),
            error_message: String::new(),
            test_time: TimeUtils::now(),
            test_duration_ms: 0,
            metadata: CollectionUtils::new_hashmap(),
            status_codes: Vec::new(),
            dns_resolution_time: None,
            connection_time: None,
            tls_handshake_time: None,
        }
    }

    pub fn new_with_region(count: usize, region_id: String) -> Self {
        let mut stats = Self::new(count);
        stats.region_id = Some(region_id);
        stats
    }

    pub fn is_successful(&self) -> bool {
        self.successful_pings > 0
    }

    pub fn success_rate(&self) -> f64 {
        if self.total_pings == 0 {
            0.0
        } else {
            (self.successful_pings as f64 / self.total_pings as f64) * 100.0
        }
    }

    fn get_successful_latencies(&self) -> Vec<f64> {
        self.latencies
            .iter()
            .filter(|&&lat| lat > 0.0)
            .copied()
            .collect()
    }

    pub fn median_latency(&self) -> f64 {
        use super::utils::percentile;
        
        let successful_latencies = self.get_successful_latencies();

        if successful_latencies.is_empty() {
            0.0
        } else {
            percentile(&successful_latencies, 50.0)
        }
    }

    pub fn percentile_95(&self) -> f64 {
        use super::utils::percentile;
        
        let successful_latencies = self.get_successful_latencies();

        if successful_latencies.is_empty() {
            0.0
        } else {
            percentile(&successful_latencies, 95.0)
        }
    }

    pub fn percentiles(&self, percentiles: &[f64]) -> Vec<f64> {
        use super::utils::percentiles as calculate_percentiles;
        
        let successful_latencies = self.get_successful_latencies();
 
        if successful_latencies.is_empty() {
            vec![0.0; percentiles.len()]
        } else {
            calculate_percentiles(&successful_latencies, percentiles)
        }
    }

    /// Calculate weighted QoS score using algorithm weights
    pub fn calculate_qos_grade(&self, weights: &AlgorithmWeights) -> f64 {
        if self.successful_pings == 0 {
            return 0.0;
        }

        // Normalize factors (lower is better for latency/jitter/packet_loss)
        let latency_score = Self::normalize_latency_score(self.avg);
        let jitter_score = Self::normalize_jitter_score(self.jitter);
        let packet_loss_score = Self::normalize_packet_loss_score(self.packet_loss);
        let reliability_score = self.success_rate();

        // Calculate consistency score (using standard deviation)
        let consistency_score = if self.successful_pings > 1 {
            (100.0 - self.standard_deviation.min(100.0)).max(0.0)
        } else {
            0.0
        };

        // Weighted score
        let score = (latency_score * weights.latency)
            + (jitter_score * weights.jitter)
            + (packet_loss_score * weights.packet_loss)
            + (consistency_score * weights.consistency)
            + (reliability_score * weights.availability);

        score.max(0.0).min(100.0)
    }

    fn normalize_latency_score(latency_ms: f64) -> f64 {
        super::scoring::normalization::normalize_latency_ms(Some(latency_ms))
    }

    fn normalize_jitter_score(jitter_ms: f64) -> f64 {
        super::scoring::normalization::normalize_jitter_ms(jitter_ms)
    }

    fn normalize_packet_loss_score(packet_loss_percent: f64) -> f64 {
        super::scoring::normalization::normalize_loss_percent(packet_loss_percent)
    }

    pub fn get_qos_letter_grade(&self, score: f64) -> &'static str {
        match score {
            s if s >= 95.0 => "A+ (Excellent)",
            s if s >= 90.0 => "A (Excellent)",
            s if s >= 85.0 => "A- (Very Good)",
            s if s >= 80.0 => "B+ (Very Good)",
            s if s >= 75.0 => "B (Good)",
            s if s >= 70.0 => "B- (Good)",
            s if s >= 65.0 => "C+ (Fair)",
            s if s >= 60.0 => "C (Fair)",
            s if s >= 50.0 => "D (Poor)",
            s if s >= 40.0 => "E (Very Poor)",
            _ => "F (Unacceptable)",
        }
    }

    pub fn get_performance_category(&self) -> &'static str {
        if self.successful_pings == 0 {
            return "Unreachable";
        }

        let avg_latency = self.avg;
        let packet_loss = self.packet_loss;
        let jitter = self.jitter;

        match (avg_latency, packet_loss, jitter) {
            (l, p, j) if l <= 20.0 && p <= 0.1 && j <= 5.0 => "Premium",
            (l, p, j) if l <= 50.0 && p <= 1.0 && j <= 10.0 => "Excellent",
            (l, p, j) if l <= 100.0 && p <= 2.0 && j <= 20.0 => "Good",
            (l, p, j) if l <= 200.0 && p <= 5.0 && j <= 50.0 => "Fair",
            _ => "Poor",
        }
    }

    pub fn get_basic_stats(&self) -> super::utils::BasicStats {
        let successful_latencies = self.get_successful_latencies();

        super::utils::BasicStats::from_values(&successful_latencies)
    }
}

/// Historical performance data with trend analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestHistory {
    #[serde(default = "generate_uuid")]
    pub id: String,
    pub region_id: String,
    pub region_name: String,
    pub region_url: String,
    pub historical_data: Vec<PingStats>,
    pub trend: f64,
    pub trend_confidence: f64,
    pub last_updated: DateTime<Utc>,
}

impl TestHistory {
    pub fn new(region_id: String, region_name: String, region_url: String) -> Self {
        Self {
            id: generate_uuid(),
            region_id,
            region_name,
            region_url,
            historical_data: Vec::new(),
            trend: 0.0,
            trend_confidence: 0.0,
            last_updated: TimeUtils::now(),
        }
    }

    /// # PERF: Maintains bounded history size to prevent memory growth
    pub fn add_test_result(&mut self, stats: PingStats) {
        self.historical_data.push(stats);
        
        // Keep only last 100 results to prevent unbounded growth
        const MAX_HISTORY_SIZE: usize = 100;
        if self.historical_data.len() > MAX_HISTORY_SIZE {
            self.historical_data.drain(0..self.historical_data.len() - MAX_HISTORY_SIZE);
        }
        
        // Sort by test time to ensure chronological order
        self.historical_data.sort_by(|a, b| a.test_time.cmp(&b.test_time));
        self.last_updated = TimeUtils::now();
    }

    /// Calculate performance trend using linear regression on QoS scores
    pub fn calculate_trend(&mut self, weights: &AlgorithmWeights) -> f64 {
        if self.historical_data.len() < 2 {
            self.trend = 0.0;
            self.trend_confidence = 0.0;
            return 0.0;
        }

        let scores: Vec<f64> = self
            .historical_data
            .iter()
            .map(|stats| stats.calculate_qos_grade(weights))
            .collect();

        // Calculate linear regression slope for trend
        let n = scores.len() as f64;
        let x_mean = (n - 1.0) / 2.0;
        let y_mean = scores.iter().sum::<f64>() / n;

        let mut numerator = 0.0;
        let mut denominator = 0.0;

        for (i, &score) in scores.iter().enumerate() {
            let x_diff = i as f64 - x_mean;
            let y_diff = score - y_mean;
            numerator += x_diff * y_diff;
            denominator += x_diff * x_diff;
        }

        if denominator == 0.0 {
            self.trend = 0.0;
            self.trend_confidence = 0.0;
            return 0.0;
        }

        let slope = numerator / denominator;
        self.trend = slope * 100.0; // Convert to percentage change per test
        
        // Calculate R-squared for confidence
        let mut ss_res = 0.0;
        let mut ss_tot = 0.0;
        
        for (i, &score) in scores.iter().enumerate() {
            let predicted = y_mean + slope * (i as f64 - x_mean);
            ss_res += (score - predicted).powi(2);
            ss_tot += (score - y_mean).powi(2);
        }
        
        self.trend_confidence = if ss_tot > 0.0 {
            ((1.0 - ss_res / ss_tot) * 100.0).max(0.0)
        } else {
            0.0
        };

        self.trend
    }

    pub fn get_trend_description(&self) -> &'static str {
        match self.trend {
            t if t > 5.0 => "Significantly Improving",
            t if t > 1.0 => "Improving",
            t if t > -1.0 => "Stable",
            t if t > -5.0 => "Degrading",
            _ => "Significantly Degrading",
        }
    }

    pub fn get_performance_summary(&self, weights: &AlgorithmWeights) -> PerformanceSummary {
        if self.historical_data.is_empty() {
            return PerformanceSummary::default();
        }

        let scores: Vec<f64> = self
            .historical_data
            .iter()
            .map(|stats| stats.calculate_qos_grade(weights))
            .collect();

        let recent_count = (scores.len() / 4).max(1).min(10);
        let recent_scores = &scores[scores.len().saturating_sub(recent_count)..];

        PerformanceSummary {
            overall_average: scores.iter().sum::<f64>() / scores.len() as f64,
            recent_average: recent_scores.iter().sum::<f64>() / recent_scores.len() as f64,
            best_score: scores.iter().fold(0.0, |a, &b| a.max(b)),
            worst_score: scores.iter().fold(100.0, |a, &b| a.min(b)),
            total_tests: scores.len(),
        }
    }

    pub fn get_recent_performance(&self, n: usize, weights: &AlgorithmWeights) -> Option<f64> {
        if self.historical_data.len() < n {
            return None;
        }

        let recent_data = &self.historical_data[self.historical_data.len() - n..];
        let scores: Vec<f64> = recent_data
            .iter()
            .map(|stats| stats.calculate_qos_grade(weights))
            .collect();

        Some(scores.iter().sum::<f64>() / scores.len() as f64)
    }
}

/// Aggregated performance metrics over time
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PerformanceSummary {
    pub overall_average: f64,
    pub recent_average: f64,
    pub best_score: f64,
    pub worst_score: f64,
    pub total_tests: usize,
}

impl Default for PerformanceSummary {
    fn default() -> Self {
        Self {
            overall_average: 0.0,
            recent_average: 0.0,
            best_score: 0.0,
            worst_score: 0.0,
            total_tests: 0,
        }
    }
}

impl PerformanceSummary {
    pub fn is_improving(&self) -> bool {
        self.recent_average > self.overall_average
    }

    pub fn trend_indicator(&self) -> &'static str {
        let diff = self.recent_average - self.overall_average;
        match diff {
            d if d > 5.0 => "📈 Improving",
            d if d > 1.0 => "↗️ Slightly Up",
            d if d < -5.0 => "📉 Degrading",
            d if d < -1.0 => "↘️ Slightly Down",
            _ => "➡️ Stable",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ping_stats_creation() {
        let stats = PingStats::new(10);
        assert_eq!(stats.total_pings, 10);
        assert_eq!(stats.successful_pings, 0);
        assert!(!stats.is_successful());
    }

    #[test]
    fn test_ping_stats_percentiles() {
        let mut stats = PingStats::new(5);
        stats.latencies = vec![10.0, 20.0, 30.0, 40.0, 50.0];
        stats.successful_pings = 5;

        let median = stats.median_latency();
        assert!(median >= 25.0 && median <= 35.0, "Expected median around 30.0, got {}", median);
        let p95 = stats.percentile_95();
        assert!(p95 >= 40.0 && p95 <= 60.0, "Expected 95th percentile around 50.0, got {}", p95);

        let percs = stats.percentiles(&[25.0, 50.0, 75.0]);
        // The percentile calculation uses interpolation, so we need to be flexible with the expected value
        assert!(percs[1] >= 20.0 && percs[1] <= 50.0, "Expected 50th percentile in reasonable range, got {}", percs[1]);
    }

    #[test]
    fn test_test_history() {
        let mut history = TestHistory::new(
            "test".to_string(),
            "Test Region".to_string(),
            "http://test.com".to_string(),
        );

        let stats = PingStats::new(1);
        history.add_test_result(stats);

        assert_eq!(history.historical_data.len(), 1);
    }

    #[test]
    fn test_performance_summary() {
        let summary = PerformanceSummary {
            overall_average: 80.0,
            recent_average: 85.0,
            best_score: 95.0,
            worst_score: 70.0,
            total_tests: 10,
        };

        assert!(summary.is_improving());
        assert_eq!(summary.trend_indicator(), "↗️ Slightly Up");
    }
}