// SPDX-License-Identifier: MIT
//! Tier detection and enforcement for HeartOn Engine
//!
//! Provides zero-overhead tier detection via environment variable.

use once_cell::sync::Lazy;

/// HeartOn licensing tier
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tier {
    /// Community Edition - MIT licensed, 10M voxel limit
    Community,
    /// Indie Edition - $99/year, 1B voxel limit, single developer
    Indie,
    /// Studio Edition - $299/year, unlimited voxels, team license
    Studio,
}

/// Current tier detected from HEARTON_TIER environment variable
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
}
