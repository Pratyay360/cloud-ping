//! Metrics collection and ring buffer implementation

use std::collections::VecDeque;
use super::probe::ProbeRecord;
use super::utils::percentile;

/// Ring buffer for efficient sliding window operations
#[derive(Debug, Clone)]
pub struct RingBuffer<T> {
    data: VecDeque<T>,
    capacity: usize,
}

impl<T> RingBuffer<T> {
    /// Create a new ring buffer with specified capacity
    #[must_use]
    pub const fn new(capacity: usize) -> Self {
        Self {
            data: VecDeque::new(),
            capacity,
        }
    }

    /// Push an item, removing oldest if at capacity
    pub fn push(&mut self, item: T) {
        if self.data.len() >= self.capacity {
            self.data.pop_front();
        }
        self.data.push_back(item);
    }

    /// Get iterator over items
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.data.iter()
    }

    /// Get current length
    #[must_use]
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Check if empty
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Get capacity
    #[must_use]
    pub const fn capacity(&self) -> usize {
        self.capacity
    }

    /// Get items as slice (most recent first)
    #[must_use]
    pub fn as_slice(&self) -> Vec<&T> {
        self.data.iter().rev().collect()
    }

    /// Clear all items
    pub fn clear(&mut self) {
        self.data.clear();
    }

    /// Get the most recent item
    #[must_use]
    pub fn latest(&self) -> Option<&T> {
        self.data.back()
    }

    /// Get the oldest item
    #[must_use]
    pub fn oldest(&self) -> Option<&T> {
        self.data.front()
    }
}

/// Aggregator state for per-endpoint metrics with optimized calculations
#[derive(Debug, Clone)]
pub struct AggregatorState {
    // Endpoint identification
    pub endpoint_id: String,
    
    // Data storage - circular buffers for efficient memory usage
    pub circular_buffer_short: RingBuffer<ProbeRecord>,
    pub circular_buffer_long: RingBuffer<ProbeRecord>,
    
    // Real-time metrics
    pub ewma_jitter_ms: f64,
    pub last_rtt_ms: Option<f64>,
    
    // Counters for efficiency
    pub total_sent_long: usize,
    pub total_recv_long: usize,
    pub total_sent_short: usize,
    pub total_recv_short: usize,
    
    // Cached aggregates for fast reads
    pub cached_p50_short: f64,
    pub cached_p90_short: f64,
    pub cached_p99_short: f64,
    pub cached_loss_short: f64,
    pub cached_loss_long: f64,
    pub cached_avail_short: f64,
    pub cached_avail_long: f64,
    pub last_score: Option<f64>,
    
    // Performance optimization: track if recalculation is needed
    dirty_short: bool,
    dirty_long: bool,
}

impl AggregatorState {
    /// Create new aggregator state
    #[must_use]
    pub fn new(endpoint_id: String, w_short: usize, w_long: usize) -> Self {
        Self {
            endpoint_id,
            circular_buffer_short: RingBuffer::new(w_short),
            circular_buffer_long: RingBuffer::new(w_long),
            ewma_jitter_ms: 0.0,
            last_rtt_ms: None,
            total_sent_long: 0,
            total_recv_long: 0,
            total_sent_short: 0,
            total_recv_short: 0,
            cached_p50_short: 0.0,
            cached_p90_short: 0.0,
            cached_p99_short: 0.0,
            cached_loss_short: 0.0,
            cached_loss_long: 0.0,
            cached_avail_short: 0.0,
            cached_avail_long: 0.0,
            last_score: None,
            dirty_short: true,
            dirty_long: true,
        }
    }

    /// Create a builder for AggregatorState
    #[must_use]
    pub fn builder(endpoint_id: String) -> AggregatorStateBuilder {
        AggregatorStateBuilder::new(endpoint_id)
    }

    /// Add a probe record and update all metrics
    pub fn add_record(&mut self, record: ProbeRecord, ewma_alpha: f64) {
        // Push to both buffers
        self.circular_buffer_short.push(record.clone());
        self.circular_buffer_long.push(record.clone());

        // Mark as dirty for recalculation
        self.dirty_short = true;
        self.dirty_long = true;

        // Update counts incrementally
        self.update_counts();

        // Update EWMA jitter
        self.update_ewma_jitter(&record, ewma_alpha);

        // Recompute short window aggregates immediately
        self.recompute_short_aggregates();
    }

    /// Update counters efficiently
    fn update_counts(&mut self) {
        self.total_sent_short = self.circular_buffer_short.len();
        self.total_recv_short = self.circular_buffer_short.iter()
            .filter(|r| r.success)
            .count();

        self.total_sent_long = self.circular_buffer_long.len();
        self.total_recv_long = self.circular_buffer_long.iter()
            .filter(|r| r.success)
            .count();
    }

    /// Update EWMA jitter calculation
    fn update_ewma_jitter(&mut self, record: &ProbeRecord, ewma_alpha: f64) {
        if let (Some(last_rtt), Some(current_rtt)) = (self.last_rtt_ms, record.rtt_ms) {
            let delta = (current_rtt - last_rtt).abs();
            self.ewma_jitter_ms += (delta - self.ewma_jitter_ms) * ewma_alpha;
        } else if record.rtt_ms.is_none() {
            // Treat timeout as large jitter penalty
            const MAX_JITTER_PENALTY: f64 = 100.0;
            self.ewma_jitter_ms += (MAX_JITTER_PENALTY - self.ewma_jitter_ms) * (ewma_alpha / 2.0);
        }

        if let Some(rtt) = record.rtt_ms {
            self.last_rtt_ms = Some(rtt);
        }
    }

    /// Recompute short window aggregates
    fn recompute_short_aggregates(&mut self) {
        if !self.dirty_short {
            return;
        }

        let rtts: Vec<f64> = self.circular_buffer_short.iter()
            .filter_map(|r| r.rtt_ms)
            .collect();

        if !rtts.is_empty() {
            self.cached_p50_short = percentile(&rtts, 50.0);
            self.cached_p90_short = percentile(&rtts, 90.0);
            self.cached_p99_short = percentile(&rtts, 99.0);
        } else {
            self.cached_p50_short = f64::INFINITY;
            self.cached_p90_short = f64::INFINITY;
            self.cached_p99_short = f64::INFINITY;
        }

        self.cached_loss_short = if self.total_sent_short > 0 {
            100.0 * (self.total_sent_short - self.total_recv_short) as f64 / self.total_sent_short as f64
        } else {
            0.0
        };

        self.cached_avail_short = if self.total_sent_short > 0 {
            100.0 * self.total_recv_short as f64 / self.total_sent_short as f64
        } else {
            0.0
        };

        self.dirty_short = false;
    }

    /// Recompute long window aggregates (called less frequently)
    pub fn recompute_long_aggregates(&mut self) {
        if !self.dirty_long {
            return;
        }

        self.cached_loss_long = if self.total_sent_long > 0 {
            100.0 * (self.total_sent_long - self.total_recv_long) as f64 / self.total_sent_long as f64
        } else {
            0.0
        };

        self.cached_avail_long = if self.total_sent_long > 0 {
            100.0 * self.total_recv_long as f64 / self.total_sent_long as f64
        } else {
            0.0
        };

        self.dirty_long = false;
    }

    /// Get recent failure count for alerting
    #[must_use]
    pub fn recent_failure_count(&self, last_n: usize) -> usize {
        self.circular_buffer_short
            .as_slice()
            .iter()
            .take(last_n)
            .filter(|record| !record.success)
            .count()
    }

    /// Get average RTT for short window
    #[must_use]
    pub fn avg_rtt_short(&self) -> f64 {
        let rtts: Vec<f64> = self.circular_buffer_short.iter()
            .filter_map(|r| r.rtt_ms)
            .collect();
        
        if rtts.is_empty() {
            0.0
        } else {
            rtts.iter().sum::<f64>() / rtts.len() as f64
        }
    }

    /// Check if we have enough data for reliable metrics
    #[must_use]
    pub const fn has_sufficient_data(&self) -> bool {
        self.total_sent_short >= 5 // Need at least 5 samples
    }

    /// Get health status based on current metrics
    #[must_use]
    pub fn health_status(&self) -> HealthStatus {
        if !self.has_sufficient_data() {
            return HealthStatus::Unknown;
        }

        let loss = self.cached_loss_short;
        let avg_rtt = self.avg_rtt_short();
        let jitter = self.ewma_jitter_ms;

        match (loss, avg_rtt, jitter) {
            (l, r, j) if l <= 1.0 && r <= 50.0 && j <= 10.0 => HealthStatus::Excellent,
            (l, r, j) if l <= 3.0 && r <= 100.0 && j <= 25.0 => HealthStatus::Good,
            (l, r, j) if l <= 5.0 && r <= 200.0 && j <= 50.0 => HealthStatus::Fair,
            (l, _, _) if l >= 10.0 => HealthStatus::Critical,
            _ => HealthStatus::Poor,
        }
    }
}

/// Builder for AggregatorState with sensible defaults
#[derive(Debug)]
pub struct AggregatorStateBuilder {
    endpoint_id: String,
    w_short: usize,
    w_long: usize,
}

impl AggregatorStateBuilder {
    /// Create a new builder
    #[must_use]
    pub const fn new(endpoint_id: String) -> Self {
        Self {
            endpoint_id,
            w_short: 100,
            w_long: 1000,
        }
    }

    /// Set short window size
    #[must_use]
    pub const fn short_window(mut self, size: usize) -> Self {
        self.w_short = size;
        self
    }

    /// Set long window size
    #[must_use]
    pub const fn long_window(mut self, size: usize) -> Self {
        self.w_long = size;
        self
    }

    /// Build the AggregatorState
    #[must_use]
    pub fn build(self) -> AggregatorState {
        AggregatorState::new(self.endpoint_id, self.w_short, self.w_long)
    }
}

/// Health status enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HealthStatus {
    Unknown,   // Health status is unknown due to insufficient data
    Excellent, // Excellent health with optimal performance
    Good,      // Good health with acceptable performance
    Fair,      // Fair health with some performance issues
    Poor,      // Poor health with significant performance issues
    Critical,  // Critical health with severe performance issues
}

impl HealthStatus {
    /// Get color for display
    #[must_use]
    pub const fn color(self) -> &'static str {
        match self {
            Self::Unknown => "gray",
            Self::Excellent => "green",
            Self::Good => "lightgreen",
            Self::Fair => "yellow",
            Self::Poor => "orange",
            Self::Critical => "red",
        }
    }

    /// Get emoji representation
    #[must_use]
    pub const fn emoji(self) -> &'static str {
        match self {
            Self::Unknown => "❓",
            Self::Excellent => "🟢",
            Self::Good => "🟡",
            Self::Fair => "🟠",
            Self::Poor => "🔴",
            Self::Critical => "💀",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::probe::ProbeRecord;

    #[test]
    fn test_ring_buffer() {
        let mut buffer = RingBuffer::new(3);
        
        buffer.push(1);
        buffer.push(2);
        buffer.push(3);
        assert_eq!(buffer.len(), 3);
        
        buffer.push(4); // Should remove 1
        assert_eq!(buffer.len(), 3);
        assert_eq!(buffer.latest(), Some(&4));
        assert_eq!(buffer.oldest(), Some(&2));
    }

    #[test]
    fn test_aggregator_state() {
        let mut state = AggregatorState::new("test".to_string(), 10, 100);
        
        // Add successful record
        let record = ProbeRecord::success("test".to_string(), 50.0);
        state.add_record(record, 0.1);
        
        assert_eq!(state.total_sent_short, 1);
        assert_eq!(state.total_recv_short, 1);
        assert_eq!(state.cached_avail_short, 100.0);
        assert_eq!(state.cached_loss_short, 0.0);
    }

    #[test]
    fn test_health_status() {
        let mut state = AggregatorState::new("test".to_string(), 10, 100);
        
        // Add enough good records
        for _ in 0..6 {
            let record = ProbeRecord::success("test".to_string(), 30.0);
            state.add_record(record, 0.1);
        }
        
        assert_eq!(state.health_status(), HealthStatus::Excellent);
    }
}