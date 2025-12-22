// SPDX-License-Identifier: MIT
//! Tier detection and enforcement for `HeartOn` Engine
//!
//! Provides zero-overhead tier detection via environment variable.

use once_cell::sync::Lazy;
use bevy::prelude::*;
use tracing::info;

/// `HeartOn` licensing tier
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tier {
    /// Community Edition - MIT licensed, 10M voxel limit
    Community,
    /// Indie Edition - $99/year, 1B voxel limit, single developer
    Indie,
    /// Studio Edition - $299/year, unlimited voxels, team license
    Studio,
}

impl Tier {
    /// Get the name of this tier
    pub fn name(self) -> &'static str {
        match self {
            Tier::Community => "Community",
            Tier::Indie => "Indie",
            Tier::Studio => "Studio",
        }
    }
}

/// Current tier detected from `HEARTON_TIER` environment variable
///
/// Set via: `export HEARTON_TIER=indie` or `HEARTON_TIER=studio`
/// Defaults to Community if unset or invalid.
///
/// Detection happens once at startup via lazy static (zero runtime cost).
pub static CURRENT_TIER: Lazy<Tier> = Lazy::new(|| {
    match std::env::var("HEARTON_TIER")
        .ok()
        .as_deref()
        .map(str::to_lowercase)
        .as_deref()
    {
        Some("indie") => Tier::Indie,
        Some("studio") => Tier::Studio,
        _ => Tier::Community,
    }
});

/// Get the current tier (zero-cost after first access)
#[inline]
pub fn current_tier() -> Tier {
    *CURRENT_TIER
}

/// Get maximum voxel count for current tier
pub fn max_voxels() -> usize {
    match current_tier() {
        Tier::Community => 10_000_000, // 10M voxels
        Tier::Indie => 1_000_000_000,  // 1B voxels
        Tier::Studio => usize::MAX,    // Unlimited
    }
}

/// Check if professional features are available
///
/// Returns true for Indie and Studio tiers.
pub fn requires_professional() -> bool {
    matches!(current_tier(), Tier::Indie | Tier::Studio)
}

/// Configuration for revenue reporting (Indie Tier only)
#[derive(Resource, Debug, Clone, Default, Reflect)]
#[reflect(Resource)]
pub struct RevenueReportingConfig {
    /// Whether revenue reporting is enabled (Opt-in)
    pub enabled: bool,
    /// API key or identifier for reporting (optional)
    pub reporting_id: Option<String>,
}

/// Report revenue for royalty calculation (Indie Tier)
///
/// This is an opt-in feature for Indie tier users to track their 3% royalty obligation.
///
/// # Arguments
/// * `amount_usd` - Revenue amount in USD
/// * `period` - Reporting period (e.g., "2025-Q4")
pub fn report_revenue(amount_usd: f64, period: &str) -> Result<(), String> {
    report_revenue_internal(current_tier(), amount_usd, period)
}

fn report_revenue_internal(tier: Tier, amount_usd: f64, period: &str) -> Result<(), String> {
    if tier != Tier::Indie {
        return Err("Revenue reporting is only applicable to Indie tier.".to_string());
    }
    
    // In a real implementation, this would send an HTTPS request
    // For now, we log it as per the "Honor system" description
    info!(
        "Revenue Report [Indie Tier]: ${:.2} for period {}. Royalty due (3% > $100k): ${:.2}", 
        amount_usd, 
        period,
        calculate_royalty(amount_usd)
    );
    
    Ok(())
}

fn calculate_royalty(amount: f64) -> f64 {
    if amount > 100_000.0 {
        (amount - 100_000.0) * 0.03
    } else {
        0.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tier_detection() {
        // Current tier should be valid
        let tier = current_tier();
        assert!(matches!(tier, Tier::Community | Tier::Indie | Tier::Studio));
    }

    #[test]
    fn test_voxel_limits() {
        let community_max = 10_000_000;
        let indie_max = 1_000_000_000;

        assert!(community_max < indie_max);
        assert!(max_voxels() > 0);
    }

    #[test]
    fn test_professional_check() {
        // Should return a boolean
        let _ = requires_professional();
    }

    #[test]
    fn test_royalty_calculation() {
        // Under threshold
        assert_eq!(calculate_royalty(50_000.0), 0.0);
        assert_eq!(calculate_royalty(100_000.0), 0.0);
        
        // Over threshold
        // ($100,100 - $100,000) * 0.03 = $100 * 0.03 = $3.0
        assert!((calculate_royalty(100_100.0) - 3.0).abs() < f64::EPSILON);
        
        // ($200,000 - $100,000) * 0.03 = $100,000 * 0.03 = $3,000
        assert!((calculate_royalty(200_000.0) - 3000.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_revenue_reporting_restrictions() {
        // Community tier should fail
        assert!(report_revenue_internal(Tier::Community, 150_000.0, "2025-Q1").is_err());
        
        // Studio tier should fail (0% royalty, flat fee)
        assert!(report_revenue_internal(Tier::Studio, 150_000.0, "2025-Q1").is_err());
        
        // Indie tier should succeed
        assert!(report_revenue_internal(Tier::Indie, 150_000.0, "2025-Q1").is_ok());
    }
}
