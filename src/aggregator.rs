//! Real-time metrics aggregation with sliding windows
//!
//! Processes probe records in real-time, maintaining short and long-term
//! performance metrics with configurable scoring algorithms.

use std::collections::HashMap;

use tokio::sync::mpsc;
use tokio::time::{interval, Instant};
use crate::time_utils::TimeUtils;
use crate::collection_utils::CollectionUtils;
use tracing::{debug, info};

use crate::models::{
    AggregatorState, Alert, AlgorithmWeights, ComprehensiveScoreResult, ProbeRecord,
};
use crate::models::scoring;

/// Configuration for metrics aggregation and alerting
#[derive(Debug, Clone)]
pub struct AggregatorConfig {
    pub w_short: usize,
    pub w_long: usize,
    pub ewma_alpha: f64,
    pub weights: AlgorithmWeights,
    pub long_recompute_interval_ms: u64,
    pub alert_score_drop_threshold: f64,
    pub alert_sustained_loss_threshold: f64,
    pub alert_availability_threshold: f64,
}

impl Default for AggregatorConfig {
    fn default() -> Self {
        Self {
            w_short: 60,           // ~5 minutes at 5s intervals
            w_long: 720,           // ~1 hour at 5s intervals
            ewma_alpha: 1.0 / 16.0, // ~0.0625
            weights: AlgorithmWeights::default(),
            long_recompute_interval_ms: 30000, // 30 seconds
            alert_score_drop_threshold: 20.0,
            alert_sustained_loss_threshold: 3.0,
            alert_availability_threshold: 95.0,
        }
    }
}

/// Real-time aggregator for probe data with sliding window metrics
pub struct StreamingAggregator {
    config: AggregatorConfig,
    state_map: HashMap<String, AggregatorState>,
    #[allow(dead_code)]
    alert_sender: mpsc::UnboundedSender<Alert>,
    last_long_recompute: Instant,
}

impl StreamingAggregator {
    pub fn new(config: AggregatorConfig) -> (Self, mpsc::UnboundedReceiver<Alert>) {
        let (alert_sender, alert_receiver) = mpsc::unbounded_channel();

        let aggregator = Self {
            config,
            state_map: CollectionUtils::new_hashmap(),
            alert_sender,
            last_long_recompute: Instant::now(),
        };

        (aggregator, alert_receiver)
    }

    /// Main processing loop for probe records and periodic tasks
    pub async fn start(
        mut self,
        mut probe_receiver: mpsc::UnboundedReceiver<ProbeRecord>,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        info!("Starting streaming aggregator");

        // Set up periodic long window recomputation
        let mut recompute_timer = interval(TimeUtils::duration_from_millis(self.config.long_recompute_interval_ms));

        loop {
            tokio::select! {
                // Process incoming probe records
                Some(record) = probe_receiver.recv() => {
                    self.process_probe_record(record).await;
                }
                
                // Periodic long window recomputation
                _ = recompute_timer.tick() => {
                    self.recompute_long_windows().await;
                }
                
                // Handle shutdown gracefully
                else => {
                    info!("Aggregator shutting down");
                    break;
                }
            }
        }

        Ok(())
    }

    async fn process_probe_record(&mut self, record: ProbeRecord) {
        debug!("Processing probe record for endpoint: {}", record.endpoint_id);

        // Get or create aggregator state for this endpoint
        let endpoint_id = record.endpoint_id.clone();
        
        // Use entry API to avoid double lookup and borrowing issues
        let state = self.state_map
            .entry(endpoint_id)
            .or_insert_with(|| {
                AggregatorState::new(
                    record.endpoint_id.clone(),
                    self.config.w_short,
                    self.config.w_long,
                )
            });

        // Add record and update metrics
        state.add_record(record, self.config.ewma_alpha);

        // Compute current score
        let score_result = scoring::compute_score(state, &self.config.weights);
        
        // Update last score for future comparisons
        state.last_score = Some(score_result.score as f64);

        debug!(
            "Updated metrics for {}: score={}, grade={}, loss={:.1}%, avail={:.1}%",
            state.endpoint_id,
            score_result.score,
            score_result.grade,
            state.cached_loss_short,
            state.cached_avail_short
        );
    }



    /// # PERF: Periodic recomputation prevents drift in long-term metrics
    async fn recompute_long_windows(&mut self) {
        let now = Instant::now();
        if now.duration_since(self.last_long_recompute).as_millis() < self.config.long_recompute_interval_ms as u128 {
            return;
        }

        debug!("Recomputing long window metrics for {} endpoints", self.state_map.len());

        for state in self.state_map.values_mut() {
            state.recompute_long_aggregates();
        }

        self.last_long_recompute = now;
    }

    pub fn get_endpoint_state(&self, endpoint_id: &str) -> Option<&AggregatorState> {
        self.state_map.get(endpoint_id)
    }

    pub fn get_endpoint_score(&self, endpoint_id: &str) -> Option<ComprehensiveScoreResult> {
        self.state_map.get(endpoint_id)
            .map(|state| scoring::compute_score(state, &self.config.weights))
    }

    pub fn get_all_states(&self) -> &HashMap<String, AggregatorState> {
        &self.state_map
    }

    pub fn get_summary_stats(&self) -> AggregatorSummary {
        let total_endpoints = self.state_map.len();
        let mut healthy_endpoints = 0;
        let mut degraded_endpoints = 0;
        let mut failed_endpoints = 0;

        for state in self.state_map.values() {
            let score = scoring::compute_score(state, &self.config.weights);
            match score.grade {
                'A' | 'B' => healthy_endpoints += 1,
                'C' | 'D' => degraded_endpoints += 1,
                'F' => failed_endpoints += 1,
                _ => {}
            }
        }

        AggregatorSummary {
            total_endpoints,
            healthy_endpoints,
            degraded_endpoints,
            failed_endpoints,
        }
    }
}

/// High-level health summary across all monitored endpoints
#[derive(Debug, Clone)]
pub struct AggregatorSummary {
    pub total_endpoints: usize,
    pub healthy_endpoints: usize,
    pub degraded_endpoints: usize,
    pub failed_endpoints: usize,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::ProbeRecord;
    use crate::time_utils::TimeUtils;

    #[tokio::test]
    async fn test_aggregator_basic_functionality() {
        let config = AggregatorConfig::default();
        let (mut aggregator, _alert_receiver) = StreamingAggregator::new(config);

        // Create test probe records
        let record1 = ProbeRecord {
            endpoint_id: "test-endpoint".to_string(),
            timestamp: TimeUtils::now(),
            rtt_ms: Some(50.0),
            success: true,
            error_code: None,
        };

        let record2 = ProbeRecord {
            endpoint_id: "test-endpoint".to_string(),
            timestamp: TimeUtils::now(),
            rtt_ms: Some(75.0),
            success: true,
            error_code: None,
        };

        // Process records
        aggregator.process_probe_record(record1).await;
        aggregator.process_probe_record(record2).await;

        // Check state
        let state = aggregator.get_endpoint_state("test-endpoint").unwrap();
        assert_eq!(state.total_sent_short, 2);
        assert_eq!(state.total_recv_short, 2);
        assert!(state.cached_p90_short > 0.0);
    }

    #[tokio::test]
    async fn test_score_calculation() {
        let config = AggregatorConfig::default();
        let (mut aggregator, _alert_receiver) = StreamingAggregator::new(config);

        // Add some good records
        for i in 0..10 {
            let record = ProbeRecord {
                endpoint_id: "test-endpoint".to_string(),
                timestamp: TimeUtils::now(),
                rtt_ms: Some(20.0 + i as f64), // 20-29ms latency
                success: true,
                error_code: None,
            };
            aggregator.process_probe_record(record).await;
        }

        let score = aggregator.get_endpoint_score("test-endpoint").unwrap();
        assert!(score.score >= 80.0); // Should be a good score
        assert!(matches!(score.grade, 'A' | 'B'));
    }
}