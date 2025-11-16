// HeartOn Public API
// MIT Licensed

pub fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(version(), "0.1.0");
    }
}
