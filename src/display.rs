//! Result formatting and display utilities
//!
//! Provides structured output formatting for test results with scoring
//! and ranking information.

use crate::models::{PingStats, AlgorithmWeights, ScoringAdapter};
use crate::ui_utils::DisplayUtils;
use tabled::{Table, Tabled, settings::{Style, Alignment, Modify, object::Columns}};

/// Table row for ranking display
#[derive(Tabled)]
struct RankingRow {
    #[tabled(rename = "Rank")]
    rank: usize,
    #[tabled(rename = "Region")]
    region: String,
    #[tabled(rename = "Score")]
    score: String,
    #[tabled(rename = "Grade")]
    grade: char,
    #[tabled(rename = "Latency")]
    latency: String,
    #[tabled(rename = "Loss %")]
    loss: String,
    #[tabled(rename = "Gaming")]
    gaming: String,
    #[tabled(rename = "Streaming")]
    streaming: String,
}

/// Table row for detailed metrics display
#[derive(Tabled)]
struct MetricsRow {
    #[tabled(rename = "Metric")]
    metric: String,
    #[tabled(rename = "Value")]
    value: String,
    #[tabled(rename = "Score")]
    score: String,
}

/// Formats test results for console and file output
pub struct DisplayFormatter;

impl DisplayFormatter {
    /// Get category description from grade
    fn get_category(grade: char) -> &'static str {
        match grade {
            'A' => "Excellent",
            'B' => "Good", 
            'C' => "Fair",
            'D' => "Poor",
            'F' => "Bad",
            _ => "Unknown",
        }
    }

    /// Display comprehensive results with scoring breakdown
    pub fn display_enhanced_results(name: &str, stats: &PingStats, weights: &AlgorithmWeights) {
        let score = ScoringAdapter::score_ping_stats(stats, weights, name);

        println!("\n=== {} ===", name);

        if stats.successful_pings == 0 {
            println!("Status: UNREACHABLE (100% packet loss)");
            println!("Connection Score: 0.0/100 F (Completely Unreliable)");
            return;
        }

        // Create metrics table
        let metrics_data = vec![
            MetricsRow {
                metric: "Latency (avg)".to_string(),
                value: format!("{:.2} ms", stats.avg),
                score: format!("{:.1}", score.components.latency_score),
            },
            MetricsRow {
                metric: "Latency Range".to_string(),
                value: format!("{:.2}-{:.2} ms", stats.min, stats.max),
                score: "-".to_string(),
            },
            MetricsRow {
                metric: "Jitter".to_string(),
                value: format!("{:.2} ms", stats.jitter),
                score: format!("{:.1}", score.components.jitter_score),
            },
            MetricsRow {
                metric: "Packet Loss".to_string(),
                value: format!("{:.1}%", stats.packet_loss),
                score: format!("{:.1}", score.components.packet_loss_score),
            },
            MetricsRow {
                metric: "Availability".to_string(),
                value: format!("{}/{} successful", stats.successful_pings, stats.total_pings),
                score: format!("{:.1}", score.components.availability_score),
            },
            MetricsRow {
                metric: "Overall Score".to_string(),
                value: format!("{:.1}/100 ({})", score.score, Self::get_category(score.grade)),
                score: score.grade.to_string(),
            },
        ];

        let mut table = Table::new(metrics_data);
        table
            .with(Style::rounded())
            .with(Modify::new(Columns::single(0)).with(Alignment::left()))
            .with(Modify::new(Columns::single(1)).with(Alignment::right()))
            .with(Modify::new(Columns::single(2)).with(Alignment::center()));

        println!("{}", table);

        // Display suitability scores
        Self::display_suitability_scores(&score);
    }



    fn display_suitability_scores(score: &crate::models::ComprehensiveScoreResult) {
        println!("\nApplication Suitability Scores:");
        
        let suitability_data = vec![
            MetricsRow {
                metric: "Gaming".to_string(),
                value: format!("{:.1}/100", score.suitability.gaming),
                score: Self::get_suitability_grade(score.suitability.gaming).to_string(),
            },
            MetricsRow {
                metric: "Streaming".to_string(),
                value: format!("{:.1}/100", score.suitability.streaming),
                score: Self::get_suitability_grade(score.suitability.streaming).to_string(),
            },
            MetricsRow {
                metric: "Web Browsing".to_string(),
                value: format!("{:.1}/100", score.suitability.web_browsing),
                score: Self::get_suitability_grade(score.suitability.web_browsing).to_string(),
            },
            MetricsRow {
                metric: "File Transfer".to_string(),
                value: format!("{:.1}/100", score.suitability.file_transfer),
                score: Self::get_suitability_grade(score.suitability.file_transfer).to_string(),
            },
            MetricsRow {
                metric: "VoIP".to_string(),
                value: format!("{:.1}/100", score.suitability.voip),
                score: Self::get_suitability_grade(score.suitability.voip).to_string(),
            },
        ];

        let mut table = Table::new(suitability_data);
        table
            .with(Style::rounded())
            .with(Modify::new(Columns::single(0)).with(Alignment::left()))
            .with(Modify::new(Columns::single(1)).with(Alignment::right()))
            .with(Modify::new(Columns::single(2)).with(Alignment::center()));

        println!("{}", table);
    }

    fn get_suitability_grade(score: f64) -> char {
        match score {
            s if s >= 80.0 => '★',
            s if s >= 60.0 => '◆',
            s if s >= 40.0 => '▲',
            _ => '○',
        }
    }

    /// Generate ranked performance report with recommendations
    pub fn generate_ranking_report(results: &[(String, PingStats)], weights: &AlgorithmWeights) {
        println!("\n{}", DisplayUtils::create_separator(100));
        println!("COMPREHENSIVE RANKING REPORT");
        println!("{}", DisplayUtils::create_separator(100));

        let ranked = ScoringAdapter::get_sorted_results(results, weights);
        Self::display_top_performers(&ranked);
        Self::display_recommendations(&ranked);
    }



    fn display_top_performers(ranked: &[(f64, String, PingStats, crate::models::ComprehensiveScoreResult)]) {
        println!("\nTOP PERFORMERS:");

        let display_count = ranked.len().min(10);
        let ranking_data: Vec<RankingRow> = ranked
            .iter()
            .take(display_count)
            .enumerate()
            .map(|(i, (_, name, stats, comp_score))| {
                RankingRow {
                    rank: i + 1,
                    region: DisplayUtils::format_region_name(name, 40),
                    score: format!("{:.1}", comp_score.score),
                    grade: comp_score.grade,
                    latency: DisplayUtils::format_latency(stats.avg),
                    loss: DisplayUtils::format_percentage(stats.packet_loss),
                    gaming: format!("{:.1}", comp_score.suitability.gaming),
                    streaming: format!("{:.1}", comp_score.suitability.streaming),
                }
            })
            .collect();

        let mut table = Table::new(ranking_data);
        table
            .with(Style::rounded())
            .with(Modify::new(Columns::single(0)).with(Alignment::center()))
            .with(Modify::new(Columns::single(1)).with(Alignment::left()))
            .with(Modify::new(Columns::single(2)).with(Alignment::right()))
            .with(Modify::new(Columns::single(3)).with(Alignment::center()))
            .with(Modify::new(Columns::single(4)).with(Alignment::right()))
            .with(Modify::new(Columns::single(5)).with(Alignment::right()))
            .with(Modify::new(Columns::single(6)).with(Alignment::right()))
            .with(Modify::new(Columns::single(7)).with(Alignment::right()));

        println!("{}", table);
    }

    fn display_recommendations(ranked: &[(f64, String, PingStats, crate::models::ComprehensiveScoreResult)]) {
        if ranked.is_empty() {
            return;
        }

        println!("\n{}", DisplayUtils::create_separator(100));
        println!("RECOMMENDATIONS:");
        println!("{}", DisplayUtils::create_separator(100));

        // Find best for different applications
        let best_latency = ranked
            .iter()
            .max_by(|a, b| {
                a.3.components
                    .latency_score
                    .partial_cmp(&b.3.components.latency_score)
                    .unwrap()
            })
            .unwrap();

        let best_reliability = ranked
            .iter()
            .max_by(|a, b| {
                a.3.components
                    .availability_score
                    .partial_cmp(&b.3.components.availability_score)
                    .unwrap()
            })
            .unwrap();

        println!(
            "⚡ Best Latency:       {} (Score: {})",
            best_latency.1, DisplayUtils::format_score(best_latency.3.components.latency_score)
        );
        println!(
            "🔒 Best Reliability:   {} (Score: {})",
            best_reliability.1, DisplayUtils::format_score(best_reliability.3.components.availability_score)
        );
        println!(
            "🌟 Overall Best:       {} (Overall: {})",
            ranked[0].1, ranked[0].3.score
        );
    }

    /// Show detailed URL test results with optional verbose output
    pub fn display_detailed_url_results(url: &str, stats: &PingStats, verbose: bool) {
        let weights = AlgorithmWeights::default();
        Self::display_enhanced_results(url, stats, &weights);

        if verbose {
            println!("\n=== DETAILED STATISTICS ===");
            println!("Test ID: {}", stats.id);
            println!("Test Duration: {}ms", stats.test_duration_ms);
            println!("Median Latency: {:.2}ms", stats.median_latency());
            println!("95th Percentile: {:.2}ms", stats.percentile_95());
            println!("Success Rate: {:.1}%", stats.success_rate());

            if !stats.status_codes.is_empty() {
                println!("HTTP Status Codes: {:?}", stats.status_codes);
            }

            if !stats.error_message.is_empty() {
                println!("Error: {}", stats.error_message);
            }
        }
    }

    /// Show basic URL test results summary
    pub fn display_simple_url_results(url: &str, stats: &PingStats) {
        println!("Results for {}:", url);
        println!("  Latency: {:.2}ms avg ({:.2}-{:.2}ms)", stats.avg, stats.min, stats.max);
        println!("  Jitter: {:.2}ms", stats.jitter);
        println!("  Packet Loss: {:.1}%", stats.packet_loss);
        println!("  Success Rate: {:.1}%", stats.success_rate());
    }

    /// Display simple network quality assessment
    pub fn display_simple_score(score: u8) {
        let (quality, message) = match score {
            90..=100 => ("Excellent", "Your network connection is performing exceptionally well."),
            75..=89 => ("Good", "Your network connection is solid."),
            60..=74 => ("Fair", "Your network connection is acceptable, but could be improved."),
            40..=59 => ("Poor", "You may experience some issues with your network connection."),
            _ => ("Very Poor", "Your network connection is performing poorly."),
        };

        println!("\n=== Network Quality Assessment ===");
        println!("Overall Score: {}/100", score);
        println!("Quality: {}", quality);
        println!("- {}", message);
    }
}