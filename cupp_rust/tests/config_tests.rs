#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_config() {
        let config = load_config("path/to/a/valid/config.toml");
        assert!(config.is_ok());
    }
}