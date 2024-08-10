use reqwest::{Client, StatusCode};
use serde::de::DeserializeOwned;
use std::time::Duration;

use crate::config::Config;
use crate::utils::error::{HubSpotError, Result};

pub struct ApiClient {
    client: Client,
    config: Config,
}

impl ApiClient {
    pub fn new(config: &Config) -> Self {
        let client = Client::builder()
            .timeout(Duration::from_secs(config.timeout_seconds))
            .build()
            .expect("Failed to create HTTP client");

        Self {
            client,
            config: config.clone(),
        }
    }

    pub async fn get<T>(&self, endpoint: &str) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let url = format!("{}{}", self.config.api_base_url, endpoint);
        let response = self
            .client
            .get(&url)
            .header(
                "Authorization",
                format!("Bearer {}", self.config.hubspot_api_key),
            )
            .send()
            .await?;

        self.handle_response(response).await
    }

    async fn handle_response<T>(&self, response: reqwest::Response) -> Result<T>
    where
        T: DeserializeOwned,
    {
        match response.status() {
            StatusCode::OK | StatusCode::CREATED => response
                .json::<T>()
                .await
                .map_err(|e| HubSpotError::DeserializationError(e.to_string())),
            StatusCode::UNAUTHORIZED => Err(HubSpotError::InvalidApiKey(
                self.config.hubspot_api_key.clone(),
            )),
            StatusCode::NOT_FOUND => Err(HubSpotError::ResourceNotFound),
            StatusCode::TOO_MANY_REQUESTS => Err(HubSpotError::RateLimitExceeded),
            _ => {
                let status_code = response.status().as_u16();
                let error_body = response
                    .text()
                    .await
                    .map_err(|e| HubSpotError::NetworkError(e.to_string()))?;
                Err(HubSpotError::ApiError {
                    status_code,
                    message: error_body,
                })
            }
        }
    }
}
