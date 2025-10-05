use console::style;
use tracing::{info, Level};
use tracing_subscriber::{EnvFilter, FmtSubscriber};

use cloud_ping::{
    AppConfig, ConnectionBenchmark, DisplayFormatter, Result, VERSION,
};

#[tokio::main]
async fn main() {
    if let Err(e) = run().await {
        eprintln!("{}: {}", style("Error").red().bold(), e);
        std::process::exit(1);
    }
}

async fn run() -> Result<()> {
    // Initialize logging
    init_logging(false);
    
    info!("Starting Cloud Ping RS v{}", VERSION);
    
    // Load configuration
    let config = AppConfig::load().unwrap_or_else(|e| {
        eprintln!("Warning: Failed to load config, using defaults: {}", e);
        AppConfig::default()
    });
    
    // Run the benchmark
    let mut benchmark = ConnectionBenchmark::new(config.clone())?;
    benchmark.load_cloud_providers(&config.data_file).await?;
    
    // Check if regions were loaded
    let all_regions = benchmark.collect_all_regions();
    if all_regions.is_empty() {
        eprintln!("No regions were loaded from data.json. Please check the file format.");
        std::process::exit(1);
    }
    
    let results = benchmark.run_filtered_benchmark(10, None, None).await?;
    
    // Calculate the average score
    let total_score: f64 = results
        .iter()
        .map(|(_, stats)| {
            let score = cloud_ping::models::ScoringAdapter::score_ping_stats(stats, benchmark.weights(), "");
            score.score as f64
        })
        .sum();
    
    let average_score = if !results.is_empty() {
        (total_score / results.len() as f64) as u8
    } else {
        0
    };
    
    // Display the simple score
    DisplayFormatter::display_simple_score(average_score);
    
    Ok(())
}

/// Initialize structured logging with appropriate level
fn init_logging(verbose: bool) {
    let level = if verbose { Level::DEBUG } else { Level::INFO };
    
    let subscriber = FmtSubscriber::builder()
        .with_max_level(level)
        .with_env_filter(EnvFilter::from_default_env())
        .with_target(false)
        .with_thread_ids(false)
        .with_thread_names(false)
        .compact()
        .finish();
    
    tracing::subscriber::set_global_default(subscriber)
        .expect("Failed to set tracing subscriber");
}