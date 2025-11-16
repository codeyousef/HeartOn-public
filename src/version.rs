// HeartOn Version Information
// MIT Licensed

pub const HEARTON_VERSION: &str = "0.13.2-hearton.1";
pub const BEVY_VERSION: &str = "0.13.2";
pub const BUILD_COMMIT: &str = env!("GIT_HASH");

pub fn full_version_string() -> String {
    format!(
        "HeartOn Engine {} (Bevy {} @ {})",
        HEARTON_VERSION,
        BEVY_VERSION,
        &BUILD_COMMIT[..8]
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn version_format() {
        assert!(HEARTON_VERSION.starts_with("0.13.2-hearton"));
        assert_eq!(BEVY_VERSION, "0.13.2");
    }

    #[test]
    fn full_version_contains_components() {
        let full = full_version_string();
        assert!(full.contains("HeartOn Engine"));
        assert!(full.contains(HEARTON_VERSION));
        assert!(full.contains("Bevy"));
    }
}
