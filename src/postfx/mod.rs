// SPDX-License-Identifier: MIT
//! Public Post-FX module.

use bevy::prelude::*;

pub trait PostEffect: Send + Sync + 'static {
    fn name(&self) -> &str;
}

pub struct HeartOnPublicPostFxPlugin;

impl Plugin for HeartOnPublicPostFxPlugin {
    fn build(&self, app: &mut App) {
        // Register basic effects
    }
}
