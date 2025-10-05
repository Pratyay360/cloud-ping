//! Asynchronous endpoint probing with concurrent testing
//!
//! Provides TCP, HTTP, and ICMP probing capabilities with configurable
//! concurrency limits and jitter for distributed testing.

use std::net::{SocketAddr, ToSocketAddrs};
use std::sync::Arc;
use std::time::{Duration, Instant};
use crate::time_utils::TimeUtils;
use tokio::net::TcpStream;
use tokio::sync::{mpsc, Semaphore};
use tokio::time::{sleep, timeout};
use tracing::{debug, error, info, warn};
use rand::Rng;

use crate::error::{CloudPingError, Result};
use crate::models::{Endpoint, ProbeRecord, ProbeType};

/// Configuration for probe timing and concurrency
#[derive(Debug, Clone)]
pub struct ProbeConfig {
    pub probe_interval_ms: u64,
    pub concurrency_limit: usize,
    pub rtt_timeout_ms: u64,
    pub jitter_percent: u8,
}

impl Default for ProbeConfig {
    fn default() -> Self {
        Self {
            probe_interval_ms: 5000, // 5 seconds
            concurrency_limit: 500,
            rtt_timeout_ms: 2000,
            jitter_percent: 10,
        }
    }
}

/// Manages concurrent probing of multiple endpoints
pub struct ProbeRunner {
    config: ProbeConfig,
    semaphore: Arc<Semaphore>,
    probe_sender: mpsc::UnboundedSender<ProbeRecord>,
}

impl ProbeRunner {
    pub fn new(config: ProbeConfig) -> (Self, mpsc::UnboundedReceiver<ProbeRecord>) {
        let (probe_sender, probe_receiver) = mpsc::unbounded_channel();
        let semaphore = Arc::new(Semaphore::new(config.concurrency_limit));

        let runner = Self {
            config,
            semaphore,
            probe_sender,
        };

        (runner, probe_receiver)
    }

    /// Launch probe loops for all provided endpoints
    pub async fn start_probing(&self, endpoints: Vec<Endpoint>) -> Result<()> {
        info!("Starting probe runner with {} endpoints", endpoints.len());

        for endpoint in endpoints {
            let runner_clone = self.clone();
            tokio::spawn(async move {
                runner_clone.probe_loop(endpoint).await;
            });
        }

        Ok(())
    }

    async fn probe_loop(&self, endpoint: Endpoint) {
        info!("Starting probe loop for endpoint: {}", endpoint.id);

        loop {
            // Acquire semaphore permit
            let _permit = match self.semaphore.acquire().await {
                Ok(permit) => permit,
                Err(_) => {
                    error!("Failed to acquire semaphore permit for {}", endpoint.id);
                    break;
                }
            };

            let start = Instant::now();
            let result = self.probe_once(&endpoint).await;
            let elapsed = start.elapsed();

            let record = match result {
                Ok(success) if success => {
                    let rtt_ms = elapsed.as_millis() as f64;
                    ProbeRecord::new(endpoint.id.clone(), Some(rtt_ms), true)
                }
                Ok(_) => ProbeRecord::new(endpoint.id.clone(), None, false),
                Err(e) => ProbeRecord::with_error(endpoint.id.clone(), e.to_string()),
            };

            // Send record to aggregator
            if let Err(e) = self.probe_sender.send(record) {
                error!("Failed to send probe record for {}: {}", endpoint.id, e);
                break;
            }

            // Sleep with jitter before next probe
            let sleep_duration = self.calculate_sleep_duration();
            sleep(sleep_duration).await;
        }

        warn!("Probe loop ended for endpoint: {}", endpoint.id);
    }

    async fn probe_once(&self, endpoint: &Endpoint) -> Result<bool> {
        let timeout_duration = TimeUtils::duration_from_millis(self.config.rtt_timeout_ms);

        match endpoint.probe_type {
            ProbeType::TCP => self.probe_tcp(endpoint, timeout_duration).await,
            ProbeType::HTTP => self.probe_http(endpoint, timeout_duration).await,
            ProbeType::ICMP => self.probe_icmp(endpoint, timeout_duration).await,
        }
    }

    async fn probe_tcp(&self, endpoint: &Endpoint, timeout_duration: Duration) -> Result<bool> {
        let addr = format!("{}:{}", endpoint.host, endpoint.port);
        
        // Resolve address
        let socket_addr = match self.resolve_address(&addr).await {
            Ok(addr) => addr,
            Err(e) => {
                debug!("DNS resolution failed for {}: {}", addr, e);
                return Ok(false);
            }
        };

        // Attempt TCP connection
        let connect_future = TcpStream::connect(socket_addr);
        
        match timeout(timeout_duration, connect_future).await {
            Ok(Ok(stream)) => {
                debug!("TCP connection successful to {}", addr);
                drop(stream); // Close connection immediately
                Ok(true)
            }
            Ok(Err(e)) => {
                debug!("TCP connection failed to {}: {}", addr, e);
                Ok(false)
            }
            Err(_) => {
                debug!("TCP connection timed out to {}", addr);
                Ok(false)
            }
        }
    }

    async fn probe_http(&self, endpoint: &Endpoint, timeout_duration: Duration) -> Result<bool> {
        let url = if endpoint.port == 443 || endpoint.port == 8443 {
            format!("https://{}:{}", endpoint.host, endpoint.port)
        } else {
            format!("http://{}:{}", endpoint.host, endpoint.port)
        };

        // Add cache buster to prevent cached responses
        let cache_buster = format!("cache_buster={}", 
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_millis()
        );
        let url_with_cache_buster = format!("{}?{}", url, cache_buster);

        let client = reqwest::Client::builder()
            .timeout(timeout_duration)
            .build()
            .map_err(|e| CloudPingError::network(format!("Failed to build HTTP client: {}", e)))?;

        match client.head(&url_with_cache_buster).send().await {
            Ok(response) => {
                let success = response.status().is_success() || response.status().is_redirection();
                debug!("HTTP probe to {} returned status: {}", url, response.status());
                Ok(success)
            }
            Err(e) => {
                debug!("HTTP probe failed to {}: {}", url, e);
                Ok(false)
            }
        }
    }

    /// # OPS: ICMP requires raw socket privileges - falls back to TCP
    async fn probe_icmp(&self, endpoint: &Endpoint, _timeout_duration: Duration) -> Result<bool> {
        warn!("ICMP probing not implemented, falling back to TCP for {}", endpoint.id);
        self.probe_tcp(endpoint, _timeout_duration).await
    }

    async fn resolve_address(&self, addr: &str) -> Result<SocketAddr> {
        let addrs: Vec<SocketAddr> = tokio::task::spawn_blocking({
            let addr = addr.to_string();
            move || addr.to_socket_addrs()
        })
        .await
        .map_err(|e| CloudPingError::network(format!("DNS resolution task failed: {}", e)))?
        .map_err(|e| CloudPingError::network(format!("DNS resolution failed: {}", e)))?
        .collect();

        addrs.into_iter().next()
            .ok_or_else(|| CloudPingError::network("No addresses resolved".to_string()))
    }

    /// # WHY: Jitter prevents thundering herd effects in distributed probing
    fn calculate_sleep_duration(&self) -> Duration {
        let base_ms = self.config.probe_interval_ms;
        let jitter_range = (base_ms * self.config.jitter_percent as u64) / 100;
        
        let mut rng = rand::thread_rng();
        let jitter = rng.gen_range(0..=jitter_range * 2) as i64 - jitter_range as i64;
        
        let final_ms = (base_ms as i64 + jitter).max(100) as u64; // Minimum 100ms
        TimeUtils::min_duration(TimeUtils::duration_from_millis(final_ms))
    }
}

impl Clone for ProbeRunner {
    fn clone(&self) -> Self {
        Self {
            config: self.config.clone(),
            semaphore: Arc::clone(&self.semaphore),
            probe_sender: self.probe_sender.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cache_buster_format() {
        let url = "https://example.com/ping";
        let cache_buster = format!("cache_buster={}", 
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_millis()
        );
        let url_with_cache_buster = format!("{}?{}", url, cache_buster);
        
        assert!(url_with_cache_buster.starts_with("https://example.com/ping?cache_buster="));
        assert!(url_with_cache_buster.len() > url.len() + 15); // Should have timestamp
    }

    #[tokio::test]
    async fn test_tcp_probe_success() {
        let config = ProbeConfig::default();
        let (runner, _receiver) = ProbeRunner::new(config);
        
        // Test against a known good endpoint (Google DNS)
        let endpoint = Endpoint::new(
            "test".to_string(),
            "8.8.8.8".to_string(),
            53,
            ProbeType::TCP,
        );

        let result = runner.probe_tcp(&endpoint, TimeUtils::duration_from_secs(5)).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_tcp_probe_failure() {
        let config = ProbeConfig::default();
        let (runner, _receiver) = ProbeRunner::new(config);
        
        // Test against a non-existent endpoint
        let endpoint = Endpoint::new(
            "test".to_string(),
            "192.0.2.1".to_string(), // RFC5737 test address
            12345,
            ProbeType::TCP,
        );

        let result = runner.probe_tcp(&endpoint, TimeUtils::duration_from_millis(100)).await;
        assert!(result.is_ok());
        assert!(!result.unwrap()); // Should fail
    }

    #[test]
    fn test_sleep_duration_jitter() {
        let config = ProbeConfig {
            probe_interval_ms: 1000,
            jitter_percent: 10,
            ..Default::default()
        };
        let (runner, _receiver) = ProbeRunner::new(config);

        for _ in 0..100 {
            let duration = runner.calculate_sleep_duration();
            let ms = duration.as_millis() as u64;
            assert!(ms >= 900 && ms <= 1100, "Duration {} outside expected range", ms);
        }
    }
}