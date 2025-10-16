//! HTTP-based network testing with detailed timing metrics
//!
//! Provides latency measurement, retry logic, and comprehensive statistics
//! collection for network performance analysis.

use ipnet::IpNet;
use reqwest::{Client, ClientBuilder};
use std::sync::Arc;
use std::time::{Duration, Instant};
use crate::time_utils::TimeUtils;
use crate::format_utils::FormatUtils;
use tokio::time::timeout;
use tracing::{debug, error, info, warn};
use url::Url;

use crate::config::AppConfig;
use crate::error::{CloudPingError, Result};
use crate::models::PingStats;

/// HTTP client wrapper for network performance testing
#[derive(Debug, Clone)]
pub struct NetworkTester {
    client: Arc<Client>,
    config: AppConfig,
}

/// Timing breakdown for individual HTTP requests
#[derive(Debug, Clone)]
pub struct RequestTiming {
    pub total_time: Duration,
    pub dns_lookup: Option<Duration>,
    pub tcp_connect: Option<Duration>,
    pub tls_handshake: Option<Duration>,
    pub request_send: Option<Duration>,
    pub response_receive: Option<Duration>,
    pub status_code: Option<u16>,
    pub success: bool,
    pub error_message: Option<String>,
}

impl NetworkTester {
    pub fn new(config: AppConfig) -> Result<Self> {
        let client = Self::build_http_client(&config)?;
        Ok(Self {
            client: Arc::new(client),
            config,
        })
    }

    #[must_use]
    pub const fn builder() -> NetworkTesterBuilder {
        NetworkTesterBuilder::new()
    }

    /// # PERF: Configures connection pooling and TLS for optimal performance
    fn build_http_client(config: &AppConfig) -> Result<Client> {
        let mut builder = ClientBuilder::new()
            .timeout(TimeUtils::duration_from_millis(config.timeout_ms))
            .user_agent(&config.user_agent)
            .pool_max_idle_per_host(10)
            .pool_idle_timeout(TimeUtils::duration_from_secs(30))
            .tcp_keepalive(TimeUtils::duration_from_secs(60));

        if !config.validate_certificates {
            builder = builder.danger_accept_invalid_certs(true);
        }

        // Use rustls for better performance and security
        builder = builder.use_rustls_tls();

        builder
            .build()
            .map_err(|e| CloudPingError::config(format!("Failed to build HTTP client: {}", e)))
    }

    /// Add cache buster parameter to URL to prevent caching
    pub fn add_cache_buster(url: &str) -> Result<String> {
        let cache_buster = format!("cache_buster={}", 
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_millis()
        );

        let parsed_url = Url::parse(url)
            .map_err(|e| CloudPingError::invalid_url(format!("Invalid URL '{}': {}", url, e)))?;

        let separator = if parsed_url.query().is_some() { "&" } else { "?" };
        Ok(format!("{}{}{}", url, separator, cache_buster))
    }

    /// Add protocol prefix if missing, validate URL format
    pub fn validate_and_normalize_url(url: &str) -> Result<String> {
        let url = url.trim();
        if url.is_empty() {
            return Err(CloudPingError::invalid_url("URL is empty"));
        }

        // Try to parse as-is first
        if let Ok(parsed) = Url::parse(url) {
            if parsed.scheme() == "http" || parsed.scheme() == "https" {
                return Ok(url.to_string());
            }
        }

        // Try to detect if it's an IP address
        let is_ip = url.parse::<IpNet>().is_ok();

        let normalized = if is_ip {
            format!("http://{}", url)
        } else {
            format!("https://{}", url)
        };

        // Validate the normalized URL
        Url::parse(&normalized)
            .map_err(|e| CloudPingError::invalid_url(format!("Invalid URL '{}': {}", url, e)))?;

        Ok(normalized)
    }

    /// Execute HTTP request with exponential backoff retry logic
    pub async fn ping_url_with_retry(&self, url: &str, max_retries: usize) -> RequestTiming {
        for attempt in 0..=max_retries {
            debug!("Attempting request to {} (attempt {}/{})", url, attempt + 1, max_retries + 1);
            
            let timing = self.perform_single_request(url).await;
            
            if timing.success {
                debug!("Request to {} succeeded in {:?}", url, timing.total_time);
                return timing;
            }

            if attempt < max_retries {
                let delay = TimeUtils::duration_from_millis(self.config.retry_delay_ms);
                debug!("Request failed, retrying in {:?}", delay);
                tokio::time::sleep(delay).await;
            }
        }

        warn!("All {} attempts to {} failed", max_retries + 1, url);
        RequestTiming {
            total_time: TimeUtils::duration_from_millis(0),
            dns_lookup: None,
            tcp_connect: None,
            tls_handshake: None,
            request_send: None,
            response_receive: None,
            status_code: None,
            success: false,
            error_message: Some("All retry attempts failed".to_string()),
        }
    }

    async fn perform_single_request(&self, url: &str) -> RequestTiming {
        let start = Instant::now();
        
        // Add cache buster to prevent cached responses
        let url_with_cache_buster = match Self::add_cache_buster(url) {
            Ok(url) => url,
            Err(e) => {
                error!("Failed to add cache buster to URL {}: {}", url, e);
                url.to_string() // Fall back to original URL
            }
        };
        
        let request_future = self.client.get(&url_with_cache_buster).send();
        let timeout_duration = TimeUtils::duration_from_millis(self.config.timeout_ms);
        
        match timeout(timeout_duration, request_future).await {
            Ok(Ok(response)) => {
                let total_time = start.elapsed();
                let status_code = response.status().as_u16();
                let success = response.status().is_success() || 
                             response.status().is_redirection() ||
                             status_code == 0; // Some endpoints return 0 for successful pings

                debug!("Request completed with status {} in {:?}", status_code, total_time);

                RequestTiming {
                    total_time,
                    dns_lookup: None, // TODO: Extract from reqwest if available
                    tcp_connect: None,
                    tls_handshake: None,
                    request_send: None,
                    response_receive: None,
                    status_code: Some(status_code),
                    success,
                    error_message: if success { None } else { Some(format!("HTTP {}", status_code)) },
                }
            }
            Ok(Err(e)) => {
                let total_time = start.elapsed();
                error!("Request to {} failed: {}", url, e);
                
                RequestTiming {
                    total_time,
                    dns_lookup: None,
                    tcp_connect: None,
                    tls_handshake: None,
                    request_send: None,
                    response_receive: None,
                    status_code: None,
                    success: false,
                    error_message: Some(e.to_string()),
                }
            }
            Err(_) => {
                let total_time = TimeUtils::duration_from_millis(self.config.timeout_ms);
                warn!("Request to {} timed out after {:?}", url, total_time);
                
                RequestTiming {
                    total_time,
                    dns_lookup: None,
                    tcp_connect: None,
                    tls_handshake: None,
                    request_send: None,
                    response_receive: None,
                    status_code: Some(408), // Request Timeout status code
                    success: false,
                    error_message: Some(FormatUtils::format_timeout_message(self.config.timeout_ms)),
                }
            }
        }
    }

    /// Execute multiple requests and aggregate performance statistics
    pub async fn perform_ping_test(&self, url: &str, count: usize) -> PingStats {
        info!("Starting ping test to {} with {} requests", url, count);
        let test_start = Instant::now();
        
        let mut stats = PingStats::new(count);
        let mut successful_latencies = Vec::new();
        let mut status_codes = Vec::new();

        for i in 0..count {
            debug!("Ping {}/{} to {}", i + 1, count, url);
            
            let timing = self.ping_url_with_retry(url, self.config.retry_attempts).await;
            let latency_ms = timing.total_time.as_millis() as f64;

            if timing.success && latency_ms > 0.0 {
                stats.successful_pings += 1;
                successful_latencies.push(latency_ms);
                stats.latencies.push(latency_ms);
                stats.min = stats.min.min(latency_ms);
                stats.max = stats.max.max(latency_ms);
                stats.avg += latency_ms;

                if let Some(code) = timing.status_code {
                    status_codes.push(code);
                }
            } else {
                // For timeouts and failures, record the actual timeout duration for scoring penalty
                let penalty_latency = if timing.error_message.as_ref()
                    .map_or(false, |msg| msg.contains("timeout") || msg.contains("timed out")) {
                    self.config.timeout_ms as f64 // Record full timeout duration for penalty
                } else {
                    0.0 // Other failures get 0
                };
                
                stats.latencies.push(penalty_latency);
                if let Some(error) = timing.error_message {
                    if stats.error_message.is_empty() {
                        stats.error_message = error;
                    }
                }
            }

            // Small delay between requests to avoid overwhelming the server
            if i < count - 1 {
                tokio::time::sleep(TimeUtils::duration_from_millis(10)).await;
            }
        }

        stats.test_duration_ms = test_start.elapsed().as_millis() as u64;
        stats.status_codes = status_codes;
        
        self.calculate_statistics(&mut stats, &successful_latencies);
        
        info!(
            "Ping test completed: {}/{} successful, avg: {:.2}ms, loss: {:.1}%",
            stats.successful_pings, stats.total_pings, stats.avg, stats.packet_loss
        );
        
        stats
    }

    fn calculate_statistics(&self, stats: &mut PingStats, successful_latencies: &[f64]) {
        let count = stats.total_pings;
        
        // Calculate packet loss percentage
        stats.packet_loss = ((count - stats.successful_pings) as f64 / count as f64) * 100.0;

        if stats.successful_pings > 0 {
            stats.avg /= stats.successful_pings as f64;

            // Calculate jitter and standard deviation
            if successful_latencies.len() > 1 {
                // Jitter: average absolute difference between consecutive measurements
                let mut jitter_sum = 0.0;
                for i in 1..successful_latencies.len() {
                    jitter_sum += (successful_latencies[i] - successful_latencies[i - 1]).abs();
                }
                stats.jitter = jitter_sum / (successful_latencies.len() - 1) as f64;

                // Standard deviation: measure of variability
                let variance_sum: f64 = successful_latencies
                    .iter()
                    .map(|&latency| (latency - stats.avg).powi(2))
                    .sum();
                stats.standard_deviation = (variance_sum / successful_latencies.len() as f64).sqrt();
            } else {
                // Single successful ping
                stats.jitter = 0.0;
                stats.standard_deviation = 0.0;
            }
        } else {
            // No successful pings
            stats.min = 0.0;
            stats.avg = 0.0;
            stats.jitter = 0.0;
            stats.standard_deviation = 0.0;
            if stats.error_message.is_empty() {
                stats.error_message = "All ping attempts failed".to_string();
            }
        }

        debug!(
            "Statistics calculated - avg: {:.2}ms, jitter: {:.2}ms, loss: {:.1}%, stddev: {:.2}ms",
            stats.avg, stats.jitter, stats.packet_loss, stats.standard_deviation
        );
    }

    /// Quick connectivity check without detailed metrics
    pub async fn test_connectivity(&self, url: &str) -> Result<bool> {
        let timing = self.ping_url_with_retry(url, 1).await;
        Ok(timing.success)
    }

    #[must_use]
    pub fn client(&self) -> &Client {
        &self.client
    }

    #[must_use]
    pub const fn config(&self) -> &AppConfig {
        &self.config
    }
}

/// Builder pattern for NetworkTester configuration
#[derive(Debug, Clone)]
pub struct NetworkTesterBuilder {
    config: Option<AppConfig>,
}

impl NetworkTesterBuilder {
    #[must_use]
    pub const fn new() -> Self {
        Self { config: None }
    }

    #[must_use]
    pub fn config(mut self, config: AppConfig) -> Self {
        self.config = Some(config);
        self
    }

    /// # Errors
    /// Returns error if HTTP client configuration fails
    pub fn build(self) -> Result<NetworkTester> {
        let config = self.config.unwrap_or_default();
        NetworkTester::new(config)
    }
}

impl Default for NetworkTesterBuilder {
    fn default() -> Self {
        Self::new()
    }
}
#[
cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_cache_buster_no_query() {
        let url = "https://example.com/ping";
        let result = NetworkTester::add_cache_buster(url).unwrap();
        
        assert!(result.starts_with("https://example.com/ping?cache_buster="));
        assert!(result.len() > url.len() + 15); // Should have timestamp
    }

    #[test]
    fn test_add_cache_buster_with_existing_query() {
        let url = "https://example.com/ping?existing=param";
        let result = NetworkTester::add_cache_buster(url).unwrap();
        
        assert!(result.starts_with("https://example.com/ping?existing=param&cache_buster="));
        assert!(result.contains("existing=param"));
        assert!(result.contains("cache_buster="));
    }

    #[test]
    fn test_add_cache_buster_invalid_url() {
        let url = "not-a-valid-url";
        let result = NetworkTester::add_cache_buster(url);
        
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_and_normalize_url() {
        // Test HTTPS URL
        let result = NetworkTester::validate_and_normalize_url("https://example.com").unwrap();
        assert_eq!(result, "https://example.com");

        // Test HTTP URL
        let result = NetworkTester::validate_and_normalize_url("http://example.com").unwrap();
        assert_eq!(result, "http://example.com");

        // Test domain without protocol (should add HTTPS)
        let result = NetworkTester::validate_and_normalize_url("example.com").unwrap();
        assert_eq!(result, "https://example.com");

        // Test empty URL
        let result = NetworkTester::validate_and_normalize_url("");
        assert!(result.is_err());
    }
}