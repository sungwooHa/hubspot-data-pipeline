use crate::utils::error::{HubSpotError, Result};
use serde::Deserialize;
use std::env;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub hubspot_api_key: String,
    pub api_base_url: String,
    pub timeout_seconds: u64,
    pub max_retries: u32,
}

impl Config {
    pub fn from_env() -> Result<Self> {
        Ok(Config {
            hubspot_api_key: env::var("HUBSPOT_API_KEY")
                .map_err(|_| HubSpotError::InvalidApiKey("HUBSPOT_API_KEY".to_string()))?,
            api_base_url: env::var("API_BASE_URL")
                .unwrap_or_else(|_| "https://api.hubapi.com".to_string()),
            timeout_seconds: env::var("TIMEOUT_SECONDS")
                .unwrap_or_else(|_| "30".to_string())
                .parse()
                .map_err(|_| HubSpotError::MissingEnvVar("TIMEOUT_SECONDS".to_string()))?,
            max_retries: env::var("MAX_RETRIES")
                .unwrap_or_else(|_| "3".to_string())
                .parse()
                .map_err(|_| HubSpotError::MissingEnvVar("MAX_RETRIES".to_string()))?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn test_config_from_env() {
        env::set_var("HUBSPOT_API_KEY", "test_api_key");
        env::set_var("API_BASE_URL", "https://test.api.com");
        env::set_var("TIMEOUT_SECONDS", "60");
        env::set_var("MAX_RETRIES", "5");

        let config = Config::from_env().unwrap();

        assert_eq!(config.hubspot_api_key, "test_api_key");
        assert_eq!(config.api_base_url, "https://test.api.com");
        assert_eq!(config.timeout_seconds, 60);
        assert_eq!(config.max_retries, 5);
    }

    #[test]
    fn test_config_default_values() {
        env::remove_var("API_BASE_URL");
        env::remove_var("TIMEOUT_SECONDS");
        env::remove_var("MAX_RETRIES");

        let config = Config::from_env().unwrap();

        assert_eq!(config.api_base_url, "https://api.hubapi.com/");
        assert_eq!(config.timeout_seconds, 30);
        assert_eq!(config.max_retries, 3);
    }

    #[test]
    #[should_panic(expected = "MissingEnvVar")]
    fn test_missing_api_key() {
        env::remove_var("HUBSPOT_API_KEY");
        Config::from_env().unwrap();
    }
}
