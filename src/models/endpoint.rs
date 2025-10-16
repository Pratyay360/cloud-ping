//! Network endpoint definitions and probe types
//!
//! Defines target endpoints for network testing with metadata support.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Network probe types for different testing methods
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ProbeType {
    TCP,
    HTTP,
    ICMP,
}

impl Default for ProbeType {
    fn default() -> Self {
        ProbeType::TCP
    }
}

impl ProbeType {
    pub fn default_port(&self) -> u16 {
        match self {
            ProbeType::TCP => 80,
            ProbeType::HTTP => 80,
            ProbeType::ICMP => 0, // ICMP doesn't use ports
        }
    }

    /// # OPS: ICMP requires root privileges on most systems
    pub fn requires_privileges(&self) -> bool {
        matches!(self, ProbeType::ICMP)
    }
}

/// Network endpoint with probe configuration and metadata
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Endpoint {
    pub id: String,
    pub host: String,
    pub port: u16,
    pub probe_type: ProbeType,
    pub metadata: HashMap<String, String>,
}

impl Endpoint {
    pub fn new(id: String, host: String, port: u16, probe_type: ProbeType) -> Self {
        Self {
            id,
            host,
            port,
            probe_type,
            metadata: crate::collection_utils::CollectionUtils::new_hashmap(),
        }
    }

    pub fn with_metadata(
        id: String,
        host: String,
        port: u16,
        probe_type: ProbeType,
        metadata: HashMap<String, String>,
    ) -> Self {
        Self {
            id,
            host,
            port,
            probe_type,
            metadata,
        }
    }

    pub fn address(&self) -> String {
        if self.probe_type == ProbeType::ICMP {
            self.host.clone()
        } else {
            format!("{}:{}", self.host, self.port)
        }
    }

    pub fn get_metadata(&self, key: &str) -> Option<&String> {
        self.metadata.get(key)
    }

    pub fn set_metadata(&mut self, key: String, value: String) {
        self.metadata.insert(key, value);
    }

    pub fn is_valid(&self) -> bool {
        !self.id.is_empty() 
            && !self.host.is_empty() 
            && (self.probe_type == ProbeType::ICMP || self.port > 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_endpoint_creation() {
        let endpoint = Endpoint::new(
            "test".to_string(),
            "example.com".to_string(),
            80,
            ProbeType::HTTP,
        );

        assert_eq!(endpoint.id, "test");
        assert_eq!(endpoint.host, "example.com");
        assert_eq!(endpoint.port, 80);
        assert_eq!(endpoint.probe_type, ProbeType::HTTP);
        assert!(endpoint.is_valid());
    }

    #[test]
    fn test_probe_type_defaults() {
        assert_eq!(ProbeType::TCP.default_port(), 80);
        assert_eq!(ProbeType::HTTP.default_port(), 80);
        assert_eq!(ProbeType::ICMP.default_port(), 0);
        
        assert!(!ProbeType::TCP.requires_privileges());
        assert!(!ProbeType::HTTP.requires_privileges());
        assert!(ProbeType::ICMP.requires_privileges());
    }

    #[test]
    fn test_endpoint_address() {
        let tcp_endpoint = Endpoint::new(
            "tcp".to_string(),
            "example.com".to_string(),
            80,
            ProbeType::TCP,
        );
        assert_eq!(tcp_endpoint.address(), "example.com:80");

        let icmp_endpoint = Endpoint::new(
            "icmp".to_string(),
            "example.com".to_string(),
            0,
            ProbeType::ICMP,
        );
        assert_eq!(icmp_endpoint.address(), "example.com");
    }
}