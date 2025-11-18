// SPDX-License-Identifier: MIT
//! `HeartOn` Engine - MIT-licensed public layer
//!
//! This crate provides the Community Edition features of `HeartOn` Engine.

#![warn(missing_docs)]

pub mod budget;
pub mod capabilities;
pub mod debug;
pub mod hud;
pub mod metrics;
pub mod plugin;
pub mod tier;
pub mod voxel;

/// Re-export Bevy for convenience
pub use bevy;

/// Re-export main plugin
pub use plugin::HeartOnPublicPlugin;

/// Re-export capability types
pub use capabilities::{CapabilityConfig, RenderingPath};
