// HeartOn Public API
// MIT Licensed

pub mod version;
pub mod capabilities;
pub mod budget;
pub mod hud;
pub mod voxel;
pub mod plugin;

pub use version::{HEARTON_VERSION, BEVY_VERSION};
pub use capabilities::HeartOnCapabilities;
pub use budget::HeartOnBudget;
pub use plugin::{HeartOnPublicPlugin, HeartOnPublicSettings};

pub fn engine_version() -> &'static str {
    version::HEARTON_VERSION
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(engine_version(), "0.13.2-hearton.1");
    }
}
