mod api;
pub mod cli;
mod config;
mod models;
pub mod utils;

pub use api::{ApiClient, CompaniesEndpoint, ContactsEndpoint, DealsEndpoint};
pub use config::Config;
pub use models::{Company, Contact, Deal, Fetchable, HubSpotObject};
pub use utils::error::{HubSpotError, Result};

use dotenv::dotenv;

/// Initialize the HubSpot client with the API key from environment variables.
///
/// # Errors
///
/// Returns an error if the HUBSPOT_API_KEY environment variable is not set.
pub fn init() -> Result<HubSpotFetcher> {
    dotenv().ok();

    let config = Config::from_env()?;
    Ok(HubSpotFetcher::new(config))
}

/// Represents the HubSpot data fetcher library.
pub struct HubSpotFetcher {
    client: ApiClient,
}

impl HubSpotFetcher {
    /// Create a new HubSpotFetcher instance.
    pub fn new(config: Config) -> Self {
        Self {
            client: ApiClient::new(&config),
        }
    }

    /// Fetch all contacts from HubSpot.
    pub async fn fetch_contacts(&self, properties: Option<&[String]>) -> Result<Vec<Contact>> {
        ContactsEndpoint::fetch_all(&self.client, properties).await
    }

    /// Fetch all companies from HubSpot.
    pub async fn fetch_companies(&self, properties: Option<&[String]>) -> Result<Vec<Company>> {
        CompaniesEndpoint::fetch_all(&self.client, properties).await
    }

    /// Fetch all deals from HubSpot.
    pub async fn fetch_deals(&self, properties: Option<&[String]>) -> Result<Vec<Deal>> {
        DealsEndpoint::fetch_all(&self.client, properties).await
    }

    // Add more methods as needed for other operations
}
