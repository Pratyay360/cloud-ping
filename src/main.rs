use clap::{Parser, Subcommand};
use console::style;
use tracing::{info, Level};
use tracing_subscriber::{EnvFilter, FmtSubscriber};

use cloud_ping::{
    AppConfig, ConnectionBenchmark, DisplayFormatter, Result, VERSION,
};

/// Cloud Ping - Network Performance Testing Tool
#[derive(Parser)]
#[command(name = "cloud-ping")]
#[command(author, version, about = "A comprehensive network performance testing utility", long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    /// Enable verbose logging
    #[arg(short, long)]
    verbose: bool,

    /// Path to custom data file
    #[arg(short, long)]
    data_file: Option<String>,

    #[command(subcommand)]
    command: Option<Commands>,
}

/// Available commands
#[derive(Subcommand)]
enum Commands {
    /// Run a comprehensive benchmark
    Benchmark {
        /// Number of pings per region
        #[arg(short, long, default_value = "10")]
        count: usize,

        /// Filter by provider name
        #[arg(short, long)]
        provider: Option<String>,

        /// Filter by region name
        #[arg(short, long)]
        region: Option<String>,
    },
    /// Run a quick test with fewer pings
    Quick {
        /// Number of pings per region (default: 3)
        #[arg(short, long, default_value = "3")]
        count: usize,
    },
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    if let Err(e) = run(cli).await {
        eprintln!("{}: {}", style("Error").red().bold(), e);
        std::process::exit(1);
    }
}

async fn run(cli: Cli) -> Result<()> {
    // Initialize logging
    init_logging(cli.verbose);
    
    info!("Starting Cloud Ping RS v{}", VERSION);
    
    // Load configuration
    let config = AppConfig::load().unwrap_or_else(|e| {
        eprintln!("Warning: Failed to load config, using defaults: {}", e);
        AppConfig::default()
    });
    
    // Use custom data file if specified
    let data_file = cli.data_file.unwrap_or_else(|| config.data_file.clone());
    
    // Run the benchmark
    let mut benchmark = ConnectionBenchmark::new(config)?;
    benchmark.load_cloud_providers(&data_file).await?;
    
    // Check if regions were loaded
    let all_regions = benchmark.collect_all_regions();
    if all_regions.is_empty() {
        eprintln!("No regions were loaded from {}. Please check the file format.", data_file);
        std::process::exit(1);
    }
    
    // Execute the appropriate command
    match cli.command {
        Some(Commands::Benchmark { count, provider, region }) => {
            info!("Running benchmark with {} pings per region", count);
            let results = benchmark.run_filtered_benchmark(count, provider, region).await?;
            display_results(&results, &benchmark);
        }
        Some(Commands::Quick { count }) => {
            info!("Running quick test with {} pings per region", count);
            let results = benchmark.run_filtered_benchmark(count, None, None).await?;
            display_results(&results, &benchmark);
        }
        None => {
            // Default: run benchmark with 10 pings
            info!("Running default benchmark with 10 pings per region");
            let results = benchmark.run_filtered_benchmark(10, None, None).await?;
            display_results(&results, &benchmark);
        }
    }
    
    Ok(())
}

/// Display benchmark results
fn display_results(results: &[(String, cloud_ping::PingStats)], benchmark: &ConnectionBenchmark) {
    if results.is_empty() {
        eprintln!("No results to display");
        return;
    }
    
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
    
    // Display the score
    DisplayFormatter::display_simple_score(average_score);
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