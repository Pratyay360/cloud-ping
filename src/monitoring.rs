//! Main monitoring system that orchestrates probing, aggregation, and alerting

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{broadcast, RwLock};
use tokio::time::interval;
use crate::time_utils::TimeUtils;
use crate::collection_utils::CollectionUtils;
use tracing::{error, info, warn};

use crate::aggregator::{AggregatorConfig, StreamingAggregator};
use crate::error::Result;
use crate::models::{Alert, ComprehensiveScoreResult, Endpoint, ProbeType};
use crate::probe::{ProbeConfig, ProbeRunner};

/// Main monitoring system configuration
#[derive(Debug, Clone)]
pub struct MonitoringConfig {
    /// Configuration for the probe runner
    pub probe_config: ProbeConfig,
    /// Configuration for the aggregator
    pub aggregator_config: AggregatorConfig,
    /// Interval for exporting metrics in milliseconds
    pub metrics_export_interval_ms: u64,
}

impl Default for MonitoringConfig {
    fn default() -> Self {
        Self {
            probe_config: ProbeConfig::default(),
            aggregator_config: AggregatorConfig::default(),
            metrics_export_interval_ms: 60000, // 1 minute
        }
    }
}

/// Main monitoring system that coordinates all components
pub struct NetworkMonitoringSystem {
    config: MonitoringConfig,
    endpoints: Arc<RwLock<HashMap<String, Endpoint>>>,
    alert_broadcast: broadcast::Sender<Alert>,
    metrics_broadcast: broadcast::Sender<HashMap<String, ComprehensiveScoreResult>>,
}

impl NetworkMonitoringSystem {
    /// Create a new monitoring system
    pub fn new(config: MonitoringConfig) -> Self {
        let (alert_broadcast, _) = broadcast::channel(1000);
        let (metrics_broadcast, _) = broadcast::channel(100);

        Self {
            config,
            endpoints: Arc::new(RwLock::new(CollectionUtils::new_hashmap())),
            alert_broadcast,
            metrics_broadcast,
        }
    }

    /// Add an endpoint to monitor
    pub async fn add_endpoint(&self, endpoint: Endpoint) {
        let mut endpoints = self.endpoints.write().await;
        endpoints.insert(endpoint.id.clone(), endpoint);
        info!("Added endpoint for monitoring: {}", endpoints.len());
    }

    /// Remove an endpoint from monitoring
    pub async fn remove_endpoint(&self, endpoint_id: &str) -> bool {
        let mut endpoints = self.endpoints.write().await;
        endpoints.remove(endpoint_id).is_some()
    }

    /// Add multiple endpoints from regions
    pub async fn add_endpoints_from_regions(&self, regions: &[crate::models::Region]) {
        for region in regions {
            if !region.enabled {
                continue;
            }

            // Parse URL to extract host and port
            if let Ok(url) = url::Url::parse(&region.url) {
                let host = url.host_str().unwrap_or(&region.url).to_string();
                let port = url.port().unwrap_or(if url.scheme() == "https" { 443 } else { 80 });
                let probe_type = if url.scheme() == "http" || url.scheme() == "https" {
                    ProbeType::HTTP
                } else {
                    ProbeType::TCP
                };

                let endpoint = Endpoint {
                    id: region.id.clone(),
                    host,
                    port,
                    probe_type,
                    metadata: CollectionUtils::create_metadata(&[
                        ("name", &region.name),
                        ("url", &region.url),
                        ("provider", &region.provider),
                        ("country", &region.country),
                    ]),
                };

                self.add_endpoint(endpoint).await;
            } else {
                warn!("Failed to parse URL for region {}: {}", region.name, region.url);
            }
        }
    }

    /// Start the monitoring system
    pub async fn start(&self) -> Result<()> {
        info!("Starting network monitoring system");

        // Get current endpoints
        let endpoints: Vec<Endpoint> = {
            let endpoints_guard = self.endpoints.read().await;
            endpoints_guard.values().cloned().collect()
        };

        if endpoints.is_empty() {
            warn!("No endpoints configured for monitoring");
            return Ok(());
        }

        info!("Starting monitoring for {} endpoints", endpoints.len());

        // Create probe runner and aggregator
        let (probe_runner, probe_receiver) = ProbeRunner::new(self.config.probe_config.clone());
        let (aggregator, alert_receiver) = StreamingAggregator::new(self.config.aggregator_config.clone());

        // Start probe runner
        probe_runner.start_probing(endpoints).await?;

        // Start alert handler
        let alert_broadcast = self.alert_broadcast.clone();
        tokio::spawn(async move {
            Self::handle_alerts(alert_receiver, alert_broadcast).await;
        });

        // Start metrics exporter
        let metrics_broadcast = self.metrics_broadcast.clone();
        let export_interval = self.config.metrics_export_interval_ms;
        tokio::spawn(async move {
            Self::export_metrics_periodically(metrics_broadcast, export_interval).await;
        });

        // Start aggregator (this will run indefinitely)
        aggregator.start(probe_receiver).await.map_err(|e| {
            crate::error::CloudPingError::system(format!("Aggregator failed: {}", e))
        })?;

        Ok(())
    }

    /// Subscribe to alerts
    pub fn subscribe_to_alerts(&self) -> broadcast::Receiver<Alert> {
        self.alert_broadcast.subscribe()
    }

    /// Subscribe to metrics updates
    pub fn subscribe_to_metrics(&self) -> broadcast::Receiver<HashMap<String, ComprehensiveScoreResult>> {
        self.metrics_broadcast.subscribe()
    }

    /// Handle incoming alerts
    async fn handle_alerts(
        mut alert_receiver: tokio::sync::mpsc::UnboundedReceiver<Alert>,
        alert_broadcast: broadcast::Sender<Alert>,
    ) {
        while let Some(alert) = alert_receiver.recv().await {
            info!("Alert received: {:?}", alert);

            // Broadcast alert to subscribers
            if let Err(e) = alert_broadcast.send(alert) {
                error!("Failed to broadcast alert: {}", e);
            }
        }
    }

    /// Export metrics periodically
    async fn export_metrics_periodically(
        metrics_broadcast: broadcast::Sender<HashMap<String, ComprehensiveScoreResult>>,
        interval_ms: u64,
    ) {
        let mut timer = interval(TimeUtils::duration_from_millis(interval_ms));

        loop {
            timer.tick().await;

            // In a real implementation, you would collect metrics from the aggregator
            // For now, we'll send an empty metrics update
            let metrics = CollectionUtils::new_hashmap();

            if let Err(e) = metrics_broadcast.send(metrics) {
                error!("Failed to broadcast metrics: {}", e);
            }
        }
    }

    /// Get current endpoint count
    pub async fn endpoint_count(&self) -> usize {
        self.endpoints.read().await.len()
    }

    /// Get list of endpoint IDs
    pub async fn get_endpoint_ids(&self) -> Vec<String> {
        self.endpoints.read().await.keys().cloned().collect()
    }
}

/// Convenience function to create a monitoring system with default config
pub fn create_default_monitoring_system() -> NetworkMonitoringSystem {
    NetworkMonitoringSystem::new(MonitoringConfig::default())
}

/// Convenience function to create endpoints from common services
pub fn create_common_endpoints() -> Vec<Endpoint> {
    vec![
        Endpoint::new("google-dns".to_string(), "8.8.8.8".to_string(), 53, ProbeType::TCP),
        Endpoint::new("cloudflare-dns".to_string(), "1.1.1.1".to_string(), 53, ProbeType::TCP),
        Endpoint::new("google-http".to_string(), "www.google.com".to_string(), 443, ProbeType::HTTP),
        Endpoint::new("cloudflare-http".to_string(), "www.cloudflare.com".to_string(), 443, ProbeType::HTTP),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_monitoring_system_creation() {
        let config = MonitoringConfig::default();
        let system = NetworkMonitoringSystem::new(config);
        
        assert_eq!(system.endpoint_count().await, 0);
    }

    #[tokio::test]
    async fn test_add_remove_endpoints() {
        let system = create_default_monitoring_system();
        
        let endpoint = Endpoint::new(
            "test".to_string(),
            "example.com".to_string(),
            80,
            ProbeType::HTTP,
        );

        system.add_endpoint(endpoint).await;
        assert_eq!(system.endpoint_count().await, 1);

        let removed = system.remove_endpoint("test").await;
        assert!(removed);
        assert_eq!(system.endpoint_count().await, 0);
    }

    #[tokio::test]
    async fn test_common_endpoints() {
        let endpoints = create_common_endpoints();
        assert!(!endpoints.is_empty());
        
        let system = create_default_monitoring_system();
        for endpoint in endpoints {
            system.add_endpoint(endpoint).await;
        }
        
        assert!(system.endpoint_count().await > 0);
    }
}