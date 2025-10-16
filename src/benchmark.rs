//! Benchmark orchestration with concurrent testing
//!
//! Coordinates multi-threaded network tests across cloud regions with
//! comprehensive scoring and result aggregation.


use dashmap::DashMap;
use futures::future::join_all;
use indicatif::{MultiProgress, ProgressBar};
use std::sync::Arc;
use tokio::sync::Semaphore;
use tracing::{debug, info, warn};

use crate::{
    config::AppConfig,
    data_loader::DataLoader,
    display::DisplayFormatter,
    error::{CloudPingError, Result},
    models::{CloudProvider, PingStats, Region, TestHistory, AlgorithmWeights, ScoringAdapter},
    network::NetworkTester,
    ui_utils::{ProgressBarFactory, DisplayUtils},
};

/// Orchestrates concurrent network testing across multiple regions
pub struct ConnectionBenchmark {
    config: AppConfig,
    weights: AlgorithmWeights,
    providers: Vec<CloudProvider>,
    test_history: Arc<DashMap<String, TestHistory>>,
    network_tester: NetworkTester,
    progress_factory: ProgressBarFactory,
}

impl ConnectionBenchmark {
    pub fn new(config: AppConfig) -> Result<Self> {
        let network_tester = NetworkTester::new(config.clone())?;
        let weights = AlgorithmWeights::default();
        let multi_progress = MultiProgress::new();
        let progress_factory = ProgressBarFactory::new(multi_progress);

        Ok(Self {
            config,
            weights,
            providers: Vec::new(),
            test_history: Arc::new(DashMap::new()),
            network_tester,
            progress_factory,
        })
    }

    /// Create benchmark with custom scoring algorithm weights
    pub fn with_weights(config: AppConfig, mut weights: AlgorithmWeights) -> Result<Self> {
        if !weights.is_valid() {
            weights.normalize();
        }
        
        let network_tester = NetworkTester::new(config.clone())?;

        let multi_progress = MultiProgress::new();
        let progress_factory = ProgressBarFactory::new(multi_progress);

        Ok(Self {
            config,
            weights,
            providers: Vec::new(),
            test_history: Arc::new(DashMap::new()),
            network_tester,
            progress_factory,
        })
    }

    /// Test single endpoint with progress tracking
    pub async fn perform_comprehensive_ping_test(&self, url: &str, count: usize) -> PingStats {
        info!("Starting comprehensive ping test to {} with {} pings", url, count);
        
        let pb = if self.config.show_progress {
            Some(self.progress_factory.create_test_progress_bar(count, url))
        } else {
            None
        };

        let stats = self.network_tester.perform_ping_test(url, count).await;

        if let Some(pb) = pb {
            pb.finish_with_message(format!(
                "Completed: {:.1}% success, {:.2}ms avg",
                stats.success_rate(),
                stats.avg
            ));
        }

        // Store in history
        if let Some(region_id) = &stats.region_id {
            self.update_test_history(region_id.clone(), url.to_string(), stats.clone());
        }

        stats
    }



    fn update_test_history(&self, region_id: String, url: String, stats: PingStats) {
        let region_name = self.providers
            .iter()
            .flat_map(|p| &p.regions)
            .find(|r| r.id == region_id)
            .map(|r| r.name.clone())
            .unwrap_or_else(|| "Unknown".to_string());

        self.test_history
            .entry(region_id.clone())
            .or_insert_with(|| TestHistory::new(region_id, region_name, url))
            .add_test_result(stats);
    }

    /// Execute concurrent tests across multiple regions
    /// 
    /// # PERF: Uses semaphore to limit concurrent connections
    pub async fn test_regions_concurrently(
        &self,
        regions: &[Region],
        ping_count: usize,
    ) -> Result<Vec<(String, PingStats)>> {
        if regions.is_empty() {
            return Ok(Vec::new());
        }

        let semaphore = Arc::new(Semaphore::new(self.config.max_threads));
        let mut tasks = Vec::new();

        info!(
            "Testing {} regions with {} concurrent threads, {} pings each",
            regions.len(),
            self.config.max_threads,
            ping_count
        );

        // Create progress bars for each region if enabled
        let progress_bars: Vec<Option<ProgressBar>> = if self.config.show_progress {
            regions
                .iter()
                .map(|region| Some(self.progress_factory.create_test_progress_bar(ping_count, &region.name)))
                .collect()
        } else {
            vec![None; regions.len()]
        };

        for (i, region) in regions.iter().enumerate() {
            let task = self.create_region_test_task(
                semaphore.clone(),
                region.clone(),
                ping_count,
                progress_bars[i].clone(),
            );
            tasks.push(task);
        }

        let results = join_all(tasks).await;
        
        // Collect successful results and log failures
        let mut successful_results = Vec::new();
        for result in results {
            match result {
                Ok(Ok((name, stats))) => {
                    successful_results.push((name, stats));
                }
                Ok(Err(e)) => {
                    warn!("Region test failed: {}", e);
                }
                Err(e) => {
                    warn!("Task execution failed: {}", e);
                }
            }
        }

        info!("Completed testing {} regions successfully", successful_results.len());
        Ok(successful_results)
    }

    fn create_region_test_task(
        &self,
        semaphore: Arc<Semaphore>,
        region: Region,
        ping_count: usize,
        progress_bar: Option<ProgressBar>,
    ) -> tokio::task::JoinHandle<Result<(String, PingStats)>> {
        let network_tester = self.network_tester.clone();
        let region_id = region.id.clone();
        
        tokio::spawn(async move {
            let _permit = semaphore.acquire().await
                .map_err(|e| CloudPingError::concurrency(format!("Failed to acquire semaphore: {}", e)))?;
            
            debug!("Starting test for region: {}", region.name);
            
            let mut stats = network_tester.perform_ping_test(&region.url, ping_count).await;
            stats.region_id = Some(region_id);
            
            if let Some(pb) = progress_bar {
                pb.finish_with_message(format!(
                    "{}: {:.1}% success, {:.2}ms avg",
                    region.name,
                    stats.success_rate(),
                    stats.avg
                ));
            }
            
            debug!("Completed test for region: {} - Success: {:.1}%", region.name, stats.success_rate());
            
            Ok((region.name, stats))
        })
    }

    pub fn display_enhanced_results(&self, name: &str, stats: &PingStats) {
        DisplayFormatter::display_enhanced_results(name, stats, &self.weights);
    }

    pub fn display_top_results(&self, results: &[(String, PingStats)], count: usize) {
        let scored_results = ScoringAdapter::get_sorted_results(results, &self.weights);

        println!("\n{}", console::style("=== TOP PERFORMERS ===").cyan().bold());
        for (i, (score, name, stats, _)) in scored_results.iter().take(count).enumerate() {
            println!(
                "{}. {} - Score: {}, Latency: {}, Loss: {}",
                i + 1,
                console::style(name).green(),
                DisplayUtils::format_score(*score),
                DisplayUtils::format_latency(stats.avg),
                DisplayUtils::format_percentage(stats.packet_loss)
            );
        }
    }









    pub async fn load_cloud_providers(&mut self, filename: &str) -> Result<()> {
        info!("Loading cloud providers from: {}", filename);
        self.providers = DataLoader::load_cloud_providers(filename).await?;
        info!("Loaded {} providers with {} total regions", 
              self.providers.len(),
              self.providers.iter().map(|p| p.regions.len()).sum::<usize>());
        Ok(())
    }

    /// Execute benchmark with optional provider/region filtering
    pub async fn run_filtered_benchmark(
        &mut self,
        ping_count: usize,
        provider_filter: Option<String>,
        region_filter: Option<String>,
    ) -> Result<Vec<(String, PingStats)>> {
        if self.providers.is_empty() {
            self.load_cloud_providers(&self.config.data_file.clone()).await?;
        }

        let filtered_regions = self.collect_filtered_regions(provider_filter, region_filter);
        
        if filtered_regions.is_empty() {
            return Err(CloudPingError::test_execution("No regions match the specified filters"));
        }

        info!("Testing {} regions with {} pings each", filtered_regions.len(), ping_count);
        
        let results = self.test_regions_concurrently(&filtered_regions, ping_count).await?;

        Ok(results)
    }

    #[must_use]
    fn collect_filtered_regions(
        &self,
        provider_filter: Option<String>,
        region_filter: Option<String>,
    ) -> Vec<Region> {
        self.providers
            .iter()
            .filter(|provider| {
                provider_filter.as_ref().map_or(true, |filter| {
                    provider.name.to_lowercase().contains(&filter.to_lowercase())
                })
            })
            .flat_map(|provider| &provider.regions)
            .filter(|region| {
                region.enabled && region_filter.as_ref().map_or(true, |filter| {
                    region.name.to_lowercase().contains(&filter.to_lowercase())
                })
            })
            .cloned()
            .collect()
    }

    #[must_use]
    pub fn collect_all_regions(&self) -> Vec<Region> {
        self.providers
            .iter()
            .flat_map(|provider| provider.enabled_regions())
            .cloned()
            .collect()
    }

    pub fn generate_ranking_report(&self, results: &[(String, PingStats)]) {
        DisplayFormatter::generate_ranking_report(results, &self.weights);
    }

    #[must_use]
    pub fn get_test_history(&self, region_id: &str) -> Option<TestHistory> {
        self.test_history.get(region_id).map(|entry| entry.clone())
    }

    #[must_use]
    pub fn get_all_test_histories(&self) -> Vec<TestHistory> {
        self.test_history.iter().map(|entry| entry.value().clone()).collect()
    }

    pub fn clear_test_history(&self) {
        self.test_history.clear();
    }

    #[must_use]
    pub const fn config(&self) -> &AppConfig {
        &self.config
    }

    #[must_use]
    pub const fn weights(&self) -> &AlgorithmWeights {
        &self.weights
    }

    pub fn set_weights(&mut self, mut weights: AlgorithmWeights) -> Result<()> {
        if !weights.is_valid() {
            weights.normalize();
        }
        self.weights = weights;
        Ok(())
    }

    #[must_use]
    pub fn builder(config: AppConfig) -> ConnectionBenchmarkBuilder {
        ConnectionBenchmarkBuilder::new(config)
    }
}

/// Builder pattern for ConnectionBenchmark configuration
#[derive(Debug)]
pub struct ConnectionBenchmarkBuilder {
    config: AppConfig,
    weights: Option<AlgorithmWeights>,
}

impl ConnectionBenchmarkBuilder {
    #[must_use]
    pub const fn new(config: AppConfig) -> Self {
        Self {
            config,
            weights: None,
        }
    }

    #[must_use]
    pub fn weights(mut self, weights: AlgorithmWeights) -> Self {
        self.weights = Some(weights);
        self
    }

    /// # Errors
    /// Returns error if network tester creation fails or weights are invalid
    pub fn build(self) -> Result<ConnectionBenchmark> {
        if let Some(weights) = self.weights {
            ConnectionBenchmark::with_weights(self.config, weights)
        } else {
            ConnectionBenchmark::new(self.config)
        }
    }
}

