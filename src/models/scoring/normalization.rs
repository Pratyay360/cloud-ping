//! Normalization functions for converting raw metrics to normalized scores (0-100)

/// Normalize latency in milliseconds to a score (0-100)
/// Lower latency = higher score
pub fn normalize_latency_ms(latency_ms: Option<f64>) -> f64 {
    match latency_ms {
        Some(latency) if latency <= 0.0 => 100.0,
        Some(latency) => {
            // Excellent: < 20ms, Good: < 50ms, Fair: < 100ms, Poor: < 200ms, Bad: >= 200ms
            match latency {
                l if l < 20.0 => 100.0 - (l / 20.0) * 10.0,  // 90-100
                l if l < 50.0 => 90.0 - ((l - 20.0) / 30.0) * 20.0,  // 70-90
                l if l < 100.0 => 70.0 - ((l - 50.0) / 50.0) * 20.0,  // 50-70
                l if l < 200.0 => 50.0 - ((l - 100.0) / 100.0) * 30.0,  // 20-50
                _ => (200.0 / latency).min(20.0),  // 0-20
            }
        }
        None => 0.0,
    }
}

/// Normalize jitter in milliseconds to a score (0-100)
/// Lower jitter = higher score
pub fn normalize_jitter_ms(jitter_ms: f64) -> f64 {
    if jitter_ms <= 0.0 {
        return 100.0;
    }
    
    // Excellent: < 5ms, Good: < 15ms, Fair: < 30ms, Poor: < 50ms, Bad: >= 50ms
    match jitter_ms {
        j if j < 5.0 => 100.0 - (j / 5.0) * 10.0,  // 90-100
        j if j < 15.0 => 90.0 - ((j - 5.0) / 10.0) * 20.0,  // 70-90
        j if j < 30.0 => 70.0 - ((j - 15.0) / 15.0) * 20.0,  // 50-70
        j if j < 50.0 => 50.0 - ((j - 30.0) / 20.0) * 30.0,  // 20-50
        _ => (50.0 / jitter_ms).min(20.0),  // 0-20
    }
}

/// Normalize packet loss percentage to a score (0-100)
/// Lower packet loss = higher score
pub fn normalize_loss_percent(loss_percent: f64) -> f64 {
    if loss_percent <= 0.0 {
        return 100.0;
    }
    
    // Excellent: 0%, Good: < 0.1%, Fair: < 0.5%, Poor: < 2%, Bad: >= 2%
    match loss_percent {
        l if l < 0.1 => 100.0 - (l / 0.1) * 10.0,  // 90-100
        l if l < 0.5 => 90.0 - ((l - 0.1) / 0.4) * 20.0,  // 70-90
        l if l < 2.0 => 70.0 - ((l - 0.5) / 1.5) * 20.0,  // 50-70
        l if l < 5.0 => 50.0 - ((l - 2.0) / 3.0) * 30.0,  // 20-50
        _ => (5.0 / loss_percent).min(20.0),  // 0-20
    }
}

/// Normalize consistency score (already 0-100, just clamp)
pub fn normalize_consistency_score(consistency: f64) -> f64 {
    consistency.clamp(0.0, 100.0)
}

/// Normalize availability percentage (already 0-100, just clamp)
pub fn normalize_availability_percent(availability: f64) -> f64 {
    availability.clamp(0.0, 100.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normalize_latency_ms() {
        assert_eq!(normalize_latency_ms(Some(0.0)), 100.0);
        assert!(normalize_latency_ms(Some(10.0)) > 90.0);
        assert!(normalize_latency_ms(Some(25.0)) > 70.0);
        assert!(normalize_latency_ms(Some(75.0)) > 50.0);
        assert!(normalize_latency_ms(Some(150.0)) > 20.0);
        assert!(normalize_latency_ms(Some(300.0)) < 20.0);
        assert_eq!(normalize_latency_ms(None), 0.0);
    }

    #[test]
    fn test_normalize_jitter_ms() {
        assert_eq!(normalize_jitter_ms(0.0), 100.0);
        assert!(normalize_jitter_ms(2.0) > 90.0);
        assert!(normalize_jitter_ms(10.0) > 70.0);
        assert!(normalize_jitter_ms(25.0) > 50.0);
        assert!(normalize_jitter_ms(40.0) > 20.0);
        assert!(normalize_jitter_ms(100.0) < 20.0);
    }

    #[test]
    fn test_normalize_loss_percent() {
        assert_eq!(normalize_loss_percent(0.0), 100.0);
        assert!(normalize_loss_percent(0.05) > 90.0);
        assert!(normalize_loss_percent(0.3) > 70.0);
        assert!(normalize_loss_percent(1.0) > 50.0);
        assert!(normalize_loss_percent(3.0) > 20.0);
        assert!(normalize_loss_percent(10.0) < 20.0);
    }
}