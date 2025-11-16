use bevy_app::Plugin;

pub mod effects;
pub mod config;
pub mod budget;

pub use effects::{
    PostEffect, PostEffectParams, EffectQuality, EffectFormat, 
    EffectInput, EffectOutput, SimpleBloom, ToneMapping, Vignette
};
pub use config::{PostFxConfig, EffectChain};
pub use budget::PostFxBudget;

pub struct HeartOnPostFxPublicPlugin;

impl Plugin for HeartOnPostFxPublicPlugin {
    fn build(&self, app: &mut bevy_app::App) {
        app.init_resource::<PostFxBudget>()
            .add_systems(bevy_app::Update, budget::update_budget_system);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_plugin_builds() {
        let mut app = bevy_app::App::new();
        app.add_plugins(HeartOnPostFxPublicPlugin);
    }
}
