//! Utility functions and adapters for scoring operations

use super::{AlgorithmWeights, ComprehensiveScoreResult, ScoreComponents, SuitabilityScores};
use crate::models::PingStats;

/// Adapter for scoring operations on different data types
pub struct ScoringAdapter;

impl ScoringAdapter {
    /// Score PingStats directly with algorithm weights
    pub fn score_ping_stats(
        stats: &PingStats,
        weights: &AlgorithmWeights,
        _name: &str,
    ) -> ComprehensiveScoreResult {
        let components = ScoreComponents {
            latency_score: Self::calculate_latency_score_from_stats(stats),
            jitter_score: Self::calculate_jitter_score_from_stats(stats),
            packet_loss_score: Self::calculate_packet_loss_score_from_stats(stats),
            consistency_score: Self::calculate_consistency_score_from_stats(stats),
            availability_score: Self::calculate_availability_score_from_stats(stats),
        };

        let score = weights.latency * components.latency_score
            + weights.jitter * components.jitter_score
            + weights.packet_loss * components.packet_loss_score
            + weights.consistency * components.consistency_score
            + weights.availability * components.availability_score;

        let grade = Self::score_to_grade(score);
        let suitability = Self::calculate_suitability_scores(&components);

        ComprehensiveScoreResult {
            score,
            grade,
            components,
            suitability,
        }
    }

    /// Get sorted results by score (highest first)
    pub fn get_sorted_results(
        results: &[(String, PingStats)],
        weights: &AlgorithmWeights,
    ) -> Vec<(f64, String, PingStats, ComprehensiveScoreResult)> {
        let mut scored_results: Vec<_> = results
            .iter()
            .map(|(name, stats)| {
                let score_result = Self::score_ping_stats(stats, weights, name);
                let score = score_result.score;
                (score, name.clone(), stats.clone(), score_result)
            })
            .collect();

        // Sort by score descending
        scored_results.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap_or(std::cmp::Ordering::Equal));
        scored_results
    }

    fn calculate_latency_score_from_stats(stats: &PingStats) -> f64 {
        super::normalization::normalize_latency_ms(Some(stats.avg))
    }

    fn calculate_jitter_score_from_stats(stats: &PingStats) -> f64 {
        let jitter = if stats.max > stats.min {
            stats.max - stats.min
        } else {
            0.0
        };
        super::normalization::normalize_jitter_ms(jitter)
    }

    fn calculate_packet_loss_score_from_stats(stats: &PingStats) -> f64 {
        let loss_percent = if stats.total_pings > 0 {
            ((stats.total_pings - stats.successful_pings) as f64 / stats.total_pings as f64) * 100.0
        } else {
            0.0
        };
        super::normalization::normalize_loss_percent(loss_percent)
    }

    fn calculate_consistency_score_from_stats(stats: &PingStats) -> f64 {
        if stats.successful_pings < 2 {
            return 0.0;
        }
        
        // Use standard deviation as consistency metric
        let std_dev = stats.standard_deviation;
        // Lower std dev = higher consistency score
        (100.0 - std_dev.min(100.0)).max(0.0)
    }

    fn calculate_availability_score_from_stats(stats: &PingStats) -> f64 {
        if stats.total_pings == 0 {
            return 0.0;
        }
        
        let availability_percent = (stats.successful_pings as f64 / stats.total_pings as f64) * 100.0;
        availability_percent
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scoring_adapter() {
        let mut stats = PingStats::new(10);
        stats.avg = 25.0;
        stats.min = 20.0;
        stats.max = 30.0;
        stats.standard_deviation = 5.0;
        stats.successful_pings = 10;

        let weights = AlgorithmWeights::default();
        let result = ScoringAdapter::score_ping_stats(&stats, &weights, "test");

        assert!(result.score > 0.0);
        assert!(result.score <= 100.0);
        assert!(matches!(result.grade, 'A' | 'B' | 'C' | 'D' | 'F'));
    }

    #[test]
    fn test_get_sorted_results() {
        let mut good_stats = PingStats::new(10);
        good_stats.avg = 20.0;
        good_stats.min = 18.0;
        good_stats.max = 22.0;
        good_stats.standard_deviation = 2.0;
        good_stats.successful_pings = 10;

        let mut bad_stats = PingStats::new(10);
        bad_stats.avg = 200.0;
        bad_stats.min = 180.0;
        bad_stats.max = 220.0;
        bad_stats.standard_deviation = 20.0;
        bad_stats.successful_pings = 8;

        let results = vec![
            ("good".to_string(), good_stats),
            ("bad".to_string(), bad_stats),
        ];

        let weights = AlgorithmWeights::default();
        let sorted = ScoringAdapter::get_sorted_results(&results, &weights);

        assert_eq!(sorted.len(), 2);
        assert_eq!(sorted[0].1, "good");
        assert_eq!(sorted[1].1, "bad");
        assert!(sorted[0].0 > sorted[1].0); // First should have higher score
    }
}