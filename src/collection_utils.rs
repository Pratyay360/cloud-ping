//! Collection utilities - thin wrappers for consistency

use std::collections::HashMap;

/// Collection utilities - mostly re-exports for API consistency
pub struct CollectionUtils;

impl CollectionUtils {
    /// Create a new HashMap with default capacity
    #[inline]
    pub fn new_hashmap<K, V>() -> HashMap<K, V> {
        HashMap::new()
    }

    /// Create a HashMap with specified capacity
    #[inline]
    pub fn hashmap_with_capacity<K, V>(capacity: usize) -> HashMap<K, V> {
        HashMap::with_capacity(capacity)
    }

    /// Create a metadata HashMap from key-value pairs
    #[inline]
    pub fn create_metadata(pairs: &[(&str, &str)]) -> HashMap<String, String> {
        pairs.iter()
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hashmap_creation() {
        let map: HashMap<String, i32> = CollectionUtils::new_hashmap();
        assert!(map.is_empty());

        let map_with_capacity: HashMap<String, i32> = CollectionUtils::hashmap_with_capacity(10);
        assert!(map_with_capacity.is_empty());
    }

    #[test]
    fn test_metadata_creation() {
        let pairs = [("name", "test-region"), ("url", "https://test.com")];
        let metadata = CollectionUtils::create_metadata(&pairs);
        assert_eq!(metadata.get("name"), Some(&"test-region".to_string()));
        assert_eq!(metadata.get("url"), Some(&"https://test.com".to_string()));
        assert_eq!(metadata.len(), 2);
    }
}