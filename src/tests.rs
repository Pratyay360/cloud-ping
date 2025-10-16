//! Comprehensive test suite for the cloud ping application.

#[cfg(test)]
mod tests {
    use crate::{
        config::AppConfig,
        error::CloudPingError,
        models::{CloudProvider, PingStats, Region, AlgorithmWeights, AggregatorState, ProbeRecord, scoring},
        network::NetworkTester,
    };
    use tempfile::NamedTempFile;

    /// Create a test configuration
    fn create_test_config() -> AppConfig {
        AppConfig {
            default_ping_count: 5,
            quick_ping_count: 2,
            timeout_ms: 1000,
            timeout: std::time::Duration::from_millis(1000),
            max_threads: 2,
            enable_color_output: false,
            save_results_to_file: false,
            results_filename: "test_results.json".to_string(),
            data_file: "test_data.json".to_string(),
            show_progress: false,
            retry_attempts: 1,
            retry_delay_ms: 50,
            retry_delay: std::time::Duration::from_millis(50),
            verbose: false,
            output_format: crate::OutputFormat::Json,
            user_agent: "test-agent".to_string(),
            validate_certificates: false,
        }
    }

    /// Create test regions
    #[allow(dead_code)]
    fn create_test_regions() -> Vec<Region> {
        vec![
            Region::new("Test Region 1".to_string(), "https://httpbin.org/delay/0".to_string()).unwrap(),
            Region::new("Test Region 2".to_string(), "https://httpbin.org/status/200".to_string()).unwrap(),
        ]
    }

    /// Create test provider
    #[allow(dead_code)]
    fn create_test_provider() -> CloudProvider {
        let mut provider = CloudProvider::new("Test Provider".to_string()).unwrap();
        for region in create_test_regions() {
            provider.add_region(region).unwrap();
        }
        provider
    }

    #[test]
    fn test_config_validation() {
        let mut config = create_test_config();
        assert!(config.validate().is_ok());

        // Test invalid configurations
        config.default_ping_count = 0;
        assert!(config.validate().is_err());

        config.default_ping_count = 5;
        config.max_threads = 0;
        assert!(config.validate().is_err());

        config.max_threads = 150;
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_scoring_weights_validation() {
        let mut weights = AlgorithmWeights::default();
        assert!(weights.is_valid());

        // Test invalid weights (don't sum to 1.0)
        weights.latency = 0.5;
        weights.jitter = 0.5;
        weights.packet_loss = 0.5;
        weights.consistency = 0.5;
        weights.availability = 0.5;
        assert!(!weights.is_valid());

        // Test normalization
        weights.normalize();
        assert!((weights.latency + weights.jitter + weights.packet_loss + weights.consistency + weights.availability - 1.0).abs() < 0.001);
    }

    #[test]
    fn test_region_creation_and_validation() {
        // Valid region
        let region = Region::new("Test".to_string(), "https://example.com".to_string());
        assert!(region.is_ok());
        let region = region.unwrap();
        assert!(region.validate().is_ok());

        // Invalid regions
        assert!(Region::new("".to_string(), "https://example.com".to_string()).is_err());
        assert!(Region::new("Test".to_string(), "".to_string()).is_err());
        assert!(Region::new("Test".to_string(), "invalid-url".to_string()).is_err());
    }

    #[test]
    fn test_cloud_provider_operations() {
        let mut provider = CloudProvider::new("Test Provider".to_string()).unwrap();
        assert!(provider.validate().is_ok());

        let region = Region::new("Test Region".to_string(), "https://example.com".to_string()).unwrap();
        assert!(provider.add_region(region).is_ok());
        assert_eq!(provider.regions.len(), 1);
        assert_eq!(provider.enabled_regions().len(), 1);

        // Test with disabled region
        let mut disabled_region = Region::new("Disabled".to_string(), "https://example.com".to_string()).unwrap();
        disabled_region.enabled = false;
        assert!(provider.add_region(disabled_region).is_ok());
        assert_eq!(provider.regions.len(), 2);
        assert_eq!(provider.enabled_regions().len(), 1);
    }

    #[test]
    fn test_ping_stats_calculations() {
        let mut stats = PingStats::new(10);
        stats.latencies = vec![10.0, 20.0, 30.0, 40.0, 50.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stats.successful_pings = 5;
        stats.avg = 150.0; // Will be divided by successful_pings

        // Test basic calculations
        assert_eq!(stats.success_rate(), 50.0);
        assert_eq!(stats.median_latency(), 30.0);
        assert!(stats.percentile_95() > 0.0);
        assert!(stats.is_successful());

        // Test failed stats
        let failed_stats = PingStats::new(5);
        assert!(!failed_stats.is_successful());
        assert_eq!(failed_stats.success_rate(), 0.0);
        assert_eq!(failed_stats.median_latency(), 0.0);
    }

    #[test]
    fn test_comprehensive_score_calculation() {
        let weights = AlgorithmWeights::default();
        let mut state = AggregatorState::new("test".to_string(), 100, 1000);
        
        // Add some successful records
        for i in 0..10 {
            let record = ProbeRecord::success("test".to_string(), 25.0 + i as f64);
            state.add_record(record, 0.1);
        }

        let score = scoring::compute_score(&state, &weights);

        assert!(score.score > 0.0);
        assert!(score.score <= 100.0);
        assert!(!score.grade.to_string().is_empty());
        assert!(score.components.latency_score > 0.0);
        assert!(score.components.availability_score > 0.0);
    }

    #[test]
    fn test_url_validation_and_normalization() {
        // Valid URLs
        assert!(NetworkTester::validate_and_normalize_url("https://example.com").is_ok());
        assert!(NetworkTester::validate_and_normalize_url("http://example.com").is_ok());
        
        // URLs that need normalization
        let normalized = NetworkTester::validate_and_normalize_url("example.com").unwrap();
        assert!(normalized.starts_with("https://"));
        
        let ip_normalized = NetworkTester::validate_and_normalize_url("8.8.8.8").unwrap();
        assert!(ip_normalized.starts_with("https://") || ip_normalized.starts_with("http://"));

        // Invalid URLs
        assert!(NetworkTester::validate_and_normalize_url("").is_err());
        // Note: "not-a-url" gets normalized to "https://not-a-url" which is valid URL format
    }

    #[test]
    fn test_error_types() {
        let config_error = CloudPingError::config("test message");
        assert!(matches!(config_error, CloudPingError::Config { .. }));

        let validation_error = CloudPingError::validation("field", "message");
        assert!(matches!(validation_error, CloudPingError::Validation { .. }));

        let timeout_error = CloudPingError::timeout(5000);
        assert!(matches!(timeout_error, CloudPingError::Timeout { .. }));
    }

    #[tokio::test]
    async fn test_network_tester_creation() {
        let config = create_test_config();
        let network_tester = NetworkTester::new(config);
        assert!(network_tester.is_ok());
    }

    #[tokio::test]
    async fn test_benchmark_creation() {
        let config = create_test_config();
        let benchmark = crate::ConnectionBenchmark::new(config);
        assert!(benchmark.is_ok());
    }

    #[test]
    fn test_output_format_serialization() {
        use crate::OutputFormat;
        
        let json_format = OutputFormat::Json;
        let serialized = serde_json::to_string(&json_format).unwrap();
        assert_eq!(serialized, "\"json\"");
        
        let deserialized: OutputFormat = serde_json::from_str(&serialized).unwrap();
        assert!(matches!(deserialized, OutputFormat::Json));
    }

    #[tokio::test]
    async fn test_data_loader_with_invalid_file() {
        use crate::DataLoader;
        
        let result = DataLoader::load_cloud_providers("nonexistent.json").await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_data_loader_with_valid_json() {
        use crate::DataLoader;
        
        // Create temporary JSON file
        let temp_file = NamedTempFile::new().unwrap();
        let json_content = r#"{
            "Test Provider": {
                "regions": [
                    {
                        "name": "Test Region",
                        "url": "https://example.com"
                    }
                ]
            }
        }"#;
        
        tokio::fs::write(temp_file.path(), json_content).await.unwrap();
        
        let result = DataLoader::load_cloud_providers(temp_file.path().to_str().unwrap()).await;
        assert!(result.is_ok());
        
        let providers = result.unwrap();
        assert_eq!(providers.len(), 1);
        assert_eq!(providers[0].name, "Test Provider");
        assert_eq!(providers[0].regions.len(), 1);
    }

    #[test]
    fn test_performance_categories() {
        let mut stats = PingStats::new(10);
        stats.successful_pings = 10;
        
        // Premium performance
        stats.avg = 15.0;
        stats.packet_loss = 0.0;
        stats.jitter = 3.0;
        assert_eq!(stats.get_performance_category(), "Premium");
        
        // Poor performance
        stats.avg = 300.0;
        stats.packet_loss = 10.0;
        stats.jitter = 100.0;
        assert_eq!(stats.get_performance_category(), "Poor");
        
        // Unreachable
        stats.successful_pings = 0;
        assert_eq!(stats.get_performance_category(), "Unreachable");
    }

    #[test]
    fn test_qos_grade_calculation() {
        let weights = AlgorithmWeights::default();
        
        // Excellent performance
        let mut stats = PingStats::new(10);
        stats.successful_pings = 10;
        stats.avg = 10.0;
        stats.jitter = 2.0;
        stats.packet_loss = 0.0;
        
        let grade = stats.calculate_qos_grade(&weights);
        assert!(grade > 90.0);
        // The grade should be A+ for excellent performance
        assert_eq!(stats.get_qos_letter_grade(grade), "A+ (Excellent)");
        
        // Poor performance
        stats.avg = 200.0;
        stats.jitter = 50.0;
        stats.packet_loss = 5.0;
        
        let poor_grade = stats.calculate_qos_grade(&weights);
        assert!(poor_grade < 60.0); // Adjusted threshold
    }
}