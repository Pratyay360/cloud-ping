//! Region and cloud provider models

use chrono::{DateTime, Utc};
use crate::time_utils::TimeUtils;
use crate::collection_utils::CollectionUtils;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::error::{CloudPingError, Result};
use super::utils::generate_uuid;

fn default_priority() -> f64 {
    1.0
}

fn default_enabled() -> bool {
    true
}

/// Geographic coordinates
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Coordinates {
    pub latitude: f64,  // Latitude in decimal degrees (-90 to 90)
    pub longitude: f64, // Longitude in decimal degrees (-180 to 180)
}

impl Coordinates {
    /// Create new coordinates
    pub fn new(latitude: f64, longitude: f64) -> Result<Self> {
        if !(-90.0..=90.0).contains(&latitude) {
            return Err(CloudPingError::validation("latitude", "must be between -90 and 90"));
        }
        if !(-180.0..=180.0).contains(&longitude) {
            return Err(CloudPingError::validation("longitude", "must be between -180 and 180"));
        }

        Ok(Self { latitude, longitude })
    }

    /// Calculate distance to another coordinate (Haversine formula)
    pub fn distance_to(&self, other: &Coordinates) -> f64 {
        const EARTH_RADIUS_KM: f64 = 6371.0;

        let lat1_rad = self.latitude.to_radians();
        let lat2_rad = other.latitude.to_radians();
        let delta_lat = (other.latitude - self.latitude).to_radians();
        let delta_lon = (other.longitude - self.longitude).to_radians();

        let a = (delta_lat / 2.0).sin().powi(2)
            + lat1_rad.cos() * lat2_rad.cos() * (delta_lon / 2.0).sin().powi(2);
        let c = 2.0 * a.sqrt().atan2((1.0 - a).sqrt());

        EARTH_RADIUS_KM * c
    }
}

/// Represents a network region/endpoint to test
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Region {
    /// Unique identifier for the region
    #[serde(default = "generate_uuid")]
    pub id: String,
    /// Human-readable name of the region
    pub name: String,
    /// URL endpoint to test
    pub url: String,
    /// Country code (ISO 3166-1 alpha-2)
    #[serde(default)]
    pub country: String,
    /// Provider name
    #[serde(default)]
    pub provider: String,
    /// Priority for testing (higher = more important)
    #[serde(default = "default_priority")]
    pub priority: f64,
    /// Geographic coordinates
    #[serde(default)]
    pub coordinates: Option<Coordinates>,
    /// Additional metadata
    #[serde(default)]
    pub metadata: HashMap<String, String>,
    /// Whether this region is enabled for testing
    #[serde(default = "default_enabled")]
    pub enabled: bool,
    /// When this region was created
    #[serde(default = "Utc::now")]
    pub created_at: DateTime<Utc>,
    /// When this region was last updated
    #[serde(default = "Utc::now")]
    pub updated_at: DateTime<Utc>,
}

impl Region {
    /// Create a new region with validation
    pub fn new(name: String, url: String) -> Result<Self> {
        Self::validate_name(&name)?;
        Self::validate_url(&url)?;

        Ok(Self {
            id: generate_uuid(),
            name,
            url,
            country: String::new(),
            provider: String::new(),
            priority: default_priority(),
            coordinates: None,
            metadata: CollectionUtils::new_hashmap(),
            enabled: true,
            created_at: TimeUtils::now(),
            updated_at: TimeUtils::now(),
        })
    }

    /// Create a region builder for fluent construction
    pub fn builder(name: String, url: String) -> Result<RegionBuilder> {
        Self::validate_name(&name)?;
        Self::validate_url(&url)?;
        Ok(RegionBuilder::new(name, url))
    }

    /// Validate region name
    fn validate_name(name: &str) -> Result<()> {
        if name.trim().is_empty() {
            return Err(CloudPingError::validation("name", "cannot be empty"));
        }
        if name.len() > 100 {
            return Err(CloudPingError::validation("name", "cannot exceed 100 characters"));
        }
        Ok(())
    }

    /// Validate region URL
    fn validate_url(url: &str) -> Result<()> {
        if url.trim().is_empty() {
            return Err(CloudPingError::validation("url", "cannot be empty"));
        }

        // Basic URL validation
        if !url.starts_with("http://") && !url.starts_with("https://") {
            return Err(CloudPingError::validation("url", "must start with http:// or https://"));
        }

        // Try to parse URL
        url::Url::parse(url)
            .map_err(|_| CloudPingError::validation("url", "invalid URL format"))?;

        Ok(())
    }

    /// Validate the region data
    pub fn validate(&self) -> Result<()> {
        Self::validate_name(&self.name)?;
        Self::validate_url(&self.url)?;

        if self.priority < 0.0 {
            return Err(CloudPingError::validation("priority", "cannot be negative"));
        }

        if let Some(ref coords) = self.coordinates {
            Coordinates::new(coords.latitude, coords.longitude)?;
        }

        Ok(())
    }

    /// Update the region's updated_at timestamp
    pub fn touch(&mut self) {
        self.updated_at = TimeUtils::now();
    }

    /// Get metadata value by key
    pub fn get_metadata(&self, key: &str) -> Option<&String> {
        self.metadata.get(key)
    }

    /// Set metadata value
    pub fn set_metadata(&mut self, key: String, value: String) {
        self.metadata.insert(key, value);
        self.touch();
    }

    /// Calculate distance to another region
    pub fn distance_to(&self, other: &Region) -> Option<f64> {
        match (&self.coordinates, &other.coordinates) {
            (Some(coords1), Some(coords2)) => Some(coords1.distance_to(coords2)),
            _ => None,
        }
    }

    /// Check if region is in a specific country
    pub fn is_in_country(&self, country_code: &str) -> bool {
        self.country.eq_ignore_ascii_case(country_code)
    }

    /// Get display name with provider
    pub fn display_name(&self) -> String {
        if self.provider.is_empty() {
            self.name.clone()
        } else {
            format!("{} ({})", self.name, self.provider)
        }
    }
}

/// Builder pattern for Region construction
pub struct RegionBuilder {
    region: Region,
}

impl RegionBuilder {
    fn new(name: String, url: String) -> Self {
        Self {
            region: Region {
                id: generate_uuid(),
                name,
                url,
                country: String::new(),
                provider: String::new(),
                priority: default_priority(),
                coordinates: None,
                metadata: CollectionUtils::new_hashmap(),
                enabled: true,
                created_at: TimeUtils::now(),
                updated_at: TimeUtils::now(),
            },
        }
    }

    // Configuration methods for building regions
    pub fn country(mut self, country: String) -> Self {
        self.region.country = country;
        self
    }

    pub fn provider(mut self, provider: String) -> Self {
        self.region.provider = provider;
        self
    }

    pub fn priority(mut self, priority: f64) -> Self {
        self.region.priority = priority;
        self
    }

    pub fn coordinates(mut self, latitude: f64, longitude: f64) -> Result<Self> {
        self.region.coordinates = Some(Coordinates::new(latitude, longitude)?);
        Ok(self)
    }

    pub fn metadata(mut self, metadata: HashMap<String, String>) -> Self {
        self.region.metadata = metadata;
        self
    }

    pub fn enabled(mut self, enabled: bool) -> Self {
        self.region.enabled = enabled;
        self
    }

    /// Build the region after validation
    pub fn build(self) -> Result<Region> {
        self.region.validate()?;
        Ok(self.region)
    }
}

/// Represents a cloud provider with multiple regions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CloudProvider {
    /// Unique identifier for the provider
    #[serde(default = "generate_uuid")]
    pub id: String,
    /// Provider name
    pub name: String,
    /// Provider category (e.g., "Major Cloud", "CDN", "Gaming")
    #[serde(default)]
    pub category: String,
    /// List of regions for this provider
    pub regions: Vec<Region>,
    /// Provider metadata
    #[serde(default)]
    pub metadata: HashMap<String, String>,
    /// Whether this provider is enabled
    #[serde(default = "default_enabled")]
    pub enabled: bool,
    /// When this provider was created
    #[serde(default = "Utc::now")]
    pub created_at: DateTime<Utc>,
    /// When this provider was last updated
    #[serde(default = "Utc::now")]
    pub updated_at: DateTime<Utc>,
}

impl CloudProvider {
    /// Create a new cloud provider
    pub fn new(name: String) -> Result<Self> {
        if name.trim().is_empty() {
            return Err(CloudPingError::validation("name", "cannot be empty"));
        }

        Ok(Self {
            id: generate_uuid(),
            name,
            category: String::new(),
            regions: Vec::new(),
            metadata: CollectionUtils::new_hashmap(),
            enabled: true,
            created_at: TimeUtils::now(),
            updated_at: TimeUtils::now(),
        })
    }

    /// Add a region to this provider
    pub fn add_region(&mut self, mut region: Region) -> Result<()> {
        region.provider = self.name.clone();
        region.validate()?;
        self.regions.push(region);
        self.touch();
        Ok(())
    }

    /// Remove a region by ID
    pub fn remove_region(&mut self, region_id: &str) -> bool {
        let initial_len = self.regions.len();
        self.regions.retain(|r| r.id != region_id);
        let removed = self.regions.len() != initial_len;
        if removed {
            self.touch();
        }
        removed
    }

    /// Get enabled regions only
    pub fn enabled_regions(&self) -> Vec<&Region> {
        self.regions.iter().filter(|r| r.enabled).collect()
    }

    /// Get regions by country
    pub fn regions_in_country(&self, country_code: &str) -> Vec<&Region> {
        self.regions.iter().filter(|r| r.is_in_country(country_code)).collect()
    }

    /// Get region by ID
    pub fn get_region(&self, region_id: &str) -> Option<&Region> {
        self.regions.iter().find(|r| r.id == region_id)
    }

    /// Get mutable region by ID
    pub fn get_region_mut(&mut self, region_id: &str) -> Option<&mut Region> {
        self.regions.iter_mut().find(|r| r.id == region_id)
    }

    /// Validate the provider data
    pub fn validate(&self) -> Result<()> {
        if self.name.trim().is_empty() {
            return Err(CloudPingError::validation("name", "cannot be empty"));
        }

        for region in &self.regions {
            region.validate()?;
        }

        Ok(())
    }

    /// Update the provider's updated_at timestamp
    pub fn touch(&mut self) {
        self.updated_at = TimeUtils::now();
    }

    /// Get total region count
    pub fn total_regions(&self) -> usize {
        self.regions.len()
    }

    /// Get enabled region count
    pub fn enabled_region_count(&self) -> usize {
        self.enabled_regions().len()
    }

    /// Set category
    pub fn set_category(&mut self, category: String) {
        self.category = category;
        self.touch();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_coordinates() {
        let coords = Coordinates::new(40.7128, -74.0060).unwrap(); // NYC
        let other_coords = Coordinates::new(34.0522, -118.2437).unwrap(); // LA

        let distance = coords.distance_to(&other_coords);
        assert!(distance > 3900.0 && distance < 4000.0); // Approximately 3944 km
    }

    #[test]
    fn test_region_creation() {
        let region = Region::new("Test Region".to_string(), "https://example.com".to_string()).unwrap();
        assert_eq!(region.name, "Test Region");
        assert_eq!(region.url, "https://example.com");
        assert!(region.enabled);
    }

    #[test]
    fn test_region_builder() {
        let region = Region::builder("Test".to_string(), "https://example.com".to_string())
            .unwrap()
            .country("US".to_string())
            .provider("Test Provider".to_string())
            .priority(2.0)
            .coordinates(40.7128, -74.0060)
            .unwrap()
            .build()
            .unwrap();

        assert_eq!(region.country, "US");
        assert_eq!(region.provider, "Test Provider");
        assert_eq!(region.priority, 2.0);
        assert!(region.coordinates.is_some());
    }

    #[test]
    fn test_cloud_provider() {
        let mut provider = CloudProvider::new("Test Provider".to_string()).unwrap();
        let region = Region::new("Test Region".to_string(), "https://example.com".to_string()).unwrap();
        
        provider.add_region(region).unwrap();
        assert_eq!(provider.total_regions(), 1);
        assert_eq!(provider.enabled_region_count(), 1);
    }

    #[test]
    fn test_validation_errors() {
        assert!(Region::new("".to_string(), "https://example.com".to_string()).is_err());
        assert!(Region::new("Test".to_string(), "invalid-url".to_string()).is_err());
        assert!(Coordinates::new(91.0, 0.0).is_err()); // Invalid latitude
        assert!(CloudProvider::new("".to_string()).is_err());
    }
}