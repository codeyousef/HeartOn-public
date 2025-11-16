#[cfg(target_family = "wasm")]
pub mod wasm;

pub fn detect_platform() -> &'static str {
    #[cfg(target_os = "windows")]
    return "windows";
    
    #[cfg(target_os = "linux")]
    return "linux";
    
    #[cfg(target_os = "macos")]
    return "macos";
    
    #[cfg(target_family = "wasm")]
    return "wasm";
    
    #[cfg(not(any(
        target_os = "windows",
        target_os = "linux",
        target_os = "macos",
        target_family = "wasm"
    )))]
    return "unknown";
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_platform_detection() {
        let platform = detect_platform();
        assert!(!platform.is_empty());
        assert!(matches!(platform, "windows" | "linux" | "macos" | "wasm" | "unknown"));
    }
}
