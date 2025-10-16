//! Cloud provider data loading and parsing
//!
//! Handles loading and parsing cloud provider configurations from JSON files
//! with support for nested data structures and multiple provider formats.

use anyhow::{Context, Result};
use serde_json;
use serde_path_to_error;


use crate::models::{CloudProvider, Region, utils::generate_uuid};

/// Utilities for loading cloud provider data from JSON files
pub struct DataLoader;

impl DataLoader {
    /// Load and parse cloud provider configurations from JSON file
    pub async fn load_cloud_providers(filename: &str) -> Result<Vec<CloudProvider>> {
        let content = tokio::fs::read_to_string(filename)
            .await
            .context("Failed to read data file")?;

        let data: serde_json::Value = {
            let mut deserializer = serde_json::Deserializer::from_str(&content);
            serde_path_to_error::deserialize(&mut deserializer)
                .context("Failed to parse JSON data file")?
        };
        let actual_data = Self::extract_nested_json(data)?;
        
        Self::parse_providers(actual_data)
    }

    fn extract_nested_json(data: serde_json::Value) -> Result<serde_json::Value> {
        if let Some(output) = data.get("output") {
            if let Some(json_string) = output.as_str() {
                let mut deserializer = serde_json::Deserializer::from_str(json_string);
                Ok(serde_path_to_error::deserialize(&mut deserializer)
                    .context("Failed to parse nested JSON in 'output' field")?)
            } else {
                Ok(data)
            }
        } else {
            Ok(data)
        }
    }

    fn parse_providers(actual_data: serde_json::Value) -> Result<Vec<CloudProvider>> {
        let mut providers = Vec::new();

        if let Some(obj) = actual_data.as_object() {
            for (key, value) in obj {
                if let Some(value_obj) = value.as_object() {
                    // Check if this is a direct provider (has regions at this level)
                    if value_obj.get("regions").is_some() {
                        // This is a provider directly at the top level
                        let provider = Self::parse_single_provider(key, value)?;
                        providers.push(provider);
                    } else {
                        // This might be a category containing providers
                        for (provider_name, provider_data) in value_obj {
                            // Check if this looks like a provider (has regions)
                            if provider_data.get("regions").is_some() {
                                let mut provider = Self::parse_single_provider(provider_name, provider_data)?;
                                provider.category = key.clone();
                                providers.push(provider);
                            }
                        }
                    }
                }
            }
        }

        Ok(providers)
    }

    fn parse_single_provider(
        provider_name: &str,
        provider_data: &serde_json::Value,
    ) -> Result<CloudProvider> {
        let now = crate::time_utils::TimeUtils::now();
        let mut provider = CloudProvider {
            id: generate_uuid(),
            name: provider_name.to_string(),
            category: String::new(),
            regions: Vec::new(),
            metadata: crate::collection_utils::CollectionUtils::new_hashmap(),
            enabled: true,
            created_at: now,
            updated_at: now,
        };

        if let Some(regions_data) = provider_data.get("regions") {
            Self::parse_standard_regions(&mut provider, regions_data)?;
        } else if provider_name == "Gaming Servers" {
            Self::parse_gaming_servers(&mut provider, provider_data)?;
        }

        Ok(provider)
    }

    fn parse_standard_regions(
        provider: &mut CloudProvider,
        regions_data: &serde_json::Value,
    ) -> Result<()> {
        if let Some(regions_array) = regions_data.as_array() {
            for region_value in regions_array {
                match serde_path_to_error::deserialize::<_, Region>(region_value.clone()) {
                    Ok(region) => provider.regions.push(region),
                    Err(e) => {
                        eprintln!("Warning: Failed to parse region in {}: {}", provider.name, e);
                    }
                }
            }
        }
        Ok(())
    }

    fn parse_gaming_servers(
        provider: &mut CloudProvider,
        provider_data: &serde_json::Value,
    ) -> Result<()> {
        if let Some(games_obj) = provider_data.as_object() {
            for (game_name, game_data) in games_obj {
                if let Some(regions_data) = game_data.get("regions") {
                    if let Some(regions_array) = regions_data.as_array() {
                        for region_value in regions_array {
                            match serde_path_to_error::deserialize::<_, Region>(region_value.clone()) {
                                Ok(mut region) => {
                                    region.name = format!("{} - {}", game_name, region.name);
                                    provider.regions.push(region);
                                }
                                Err(e) => {
                                    eprintln!("Warning: Failed to parse gaming region in {} - {}: {}", provider.name, game_name, e);
                                }
                            }
                        }
                    }
                }
            }
        }
        Ok(())
    }
}