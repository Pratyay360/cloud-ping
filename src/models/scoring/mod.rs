//! Scoring algorithms and utilities for network performance evaluation

use serde::{Deserialize, Serialize};
use std::fmt;

use super::AggregatorState;

pub mod normalization;
pub mod utils;

pub use utils::ScoringAdapter;

/// Weights for different scoring algorithm components
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlgorithmWeights {
    pub latency: f64,
    pub jitter: f64,
    pub packet_loss: f64,
    pub consistency: f64,
    pub availability: f64,
}

impl Default for AlgorithmWeights {
    fn default() -> Self {
        Self {
            latency: 0.3,
            jitter: 0.2,
            packet_loss: 0.25,
            consistency: 0.15,
            availability: 0.1,
        }
    }
}

impl AlgorithmWeights {
    /// Check if weights are valid (sum to 1.0 and all positive)
    pub fn is_valid(&self) -> bool {
        let sum = self.latency + self.jitter + self.packet_loss + self.consistency + self.availability;
        let tolerance = 1e-6;
        
        (sum - 1.0).abs() < tolerance
            && self.latency >= 0.0
            && self.jitter >= 0.0
            && self.packet_loss >= 0.0
            && self.consistency >= 0.0
            && self.availability >= 0.0
    }

    /// Normalize weights to sum to 1.0
    pub fn normalize(&mut self) {
        let sum = self.latency + self.jitter + self.packet_loss + self.consistency + self.availability;
        if sum > 0.0 {
            self.latency /= sum;
            self.jitter /= sum;
            self.packet_loss /= sum;
            self.consistency /= sum;
            self.availability /= sum;
        }
    }
}

/// Individual score components
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScoreComponents {
    pub latency_score: f64,
    pub jitter_score: f64,
    pub packet_loss_score: f64,
    pub consistency_score: f64,
    pub availability_score: f64,
}

impl Default for ScoreComponents {
    fn default() -> Self {
        Self {
            latency_score: 0.0,
            jitter_score: 0.0,
            packet_loss_score: 0.0,
            consistency_score: 0.0,
            availability_score: 0.0,
        }
    }
}

/// Comprehensive scoring result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComprehensiveScoreResult {
    pub score: f64,
    pub grade: char,
    pub components: ScoreComponents,
    pub suitability: SuitabilityScores,
}

/// Suitability scores for different use cases
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SuitabilityScores {
    pub gaming: f64,
    pub streaming: f64,
    pub web_browsing: f64,
    pub file_transfer: f64,
    pub voip: f64,
}

impl Default for SuitabilityScores {
    fn default() -> Self {
        Self {
            gaming: 0.0,
            streaming: 0.0,
            web_browsing: 0.0,
            file_transfer: 0.0,
            voip: 0.0,
        }
    }
}

impl fmt::Display for ComprehensiveScoreResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Score: {:.1} ({})", self.score, self.grade)
    }
}

/// Compute comprehensive score for an aggregator state
pub fn compute_score(state: &AggregatorState, weights: &AlgorithmWeights) -> ComprehensiveScoreResult {
    // Convert AggregatorState to PingStats-like metrics for scoring
    let avg_latency = state.cached_p50_short;
    let jitter = state.ewma_jitter_ms;
    let packet_loss_percent = state.cached_loss_short * 100.0;
    let availability_percent = state.cached_avail_short; // This is already a percentage (0-100)
    
    // Calculate individual component scores
    let components = ScoreComponents {
        latency_score: normalization::normalize_latency_ms(Some(avg_latency)),
        jitter_score: normalization::normalize_jitter_ms(jitter),
        packet_loss_score: normalization::normalize_loss_percent(packet_loss_percent),
        consistency_score: calculate_consistency_score_from_state(state),
        availability_score: availability_percent,
    };

    // Calculate weighted overall score
    let score = weights.latency * components.latency_score
        + weights.jitter * components.jitter_score
        + weights.packet_loss * components.packet_loss_score
        + weights.consistency * components.consistency_score
        + weights.availability * components.availability_score;

    // Calculate grade
    let grade = score_to_grade(score);

    // Calculate suitability scores
    let suitability = calculate_suitability_scores(&components);

    ComprehensiveScoreResult {
        score,
        grade,
        components,
        suitability,
    }
}

fn calculate_consistency_score_from_state(state: &AggregatorState) -> f64 {
    // Use the difference between p50 and p90 as a consistency metric
    let consistency_metric = state.cached_p90_short - state.cached_p50_short;
    // Lower difference = higher consistency score
    (100.0 - consistency_metric.min(100.0)).max(0.0)
}

fn score_to_grade(score: f64) -> char {
    match score {
        s if s >= 90.0 => 'A',
        s if s >= 80.0 => 'B',
        s if s >= 70.0 => 'C',
        s if s >= 60.0 => 'D',
        _ => 'F',
    }
}

fn calculate_suitability_scores(components: &ScoreComponents) -> SuitabilityScores {
    SuitabilityScores {
        // Gaming prioritizes low latency and jitter
        gaming: (components.latency_score * 0.5 + components.jitter_score * 0.3 + components.packet_loss_score * 0.2),
        
        // Streaming prioritizes consistency and availability
        streaming: (components.consistency_score * 0.4 + components.availability_score * 0.3 + components.packet_loss_score * 0.3),
        
        // Web browsing is balanced
        web_browsing: (components.latency_score * 0.3 + components.availability_score * 0.3 + components.consistency_score * 0.4),
        
        // File transfer prioritizes availability and packet loss
        file_transfer: (components.availability_score * 0.5 + components.packet_loss_score * 0.3 + components.consistency_score * 0.2),
        
        // VoIP prioritizes low latency, jitter, and packet loss
        voip: (components.latency_score * 0.4 + components.jitter_score * 0.3 + components.packet_loss_score * 0.3),
    }
}