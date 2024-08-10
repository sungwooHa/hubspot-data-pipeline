mod client;
mod endpoints;

pub use client::ApiClient;
pub use endpoints::{CompaniesEndpoint, ContactsEndpoint, DealsEndpoint};

// You might want to define some common constants or types here
pub const API_BASE_URL: &str = "https://api.hubapi.com/";

// If there are any api-wide utility functions, you can define them here
pub fn build_api_url(endpoint: &str) -> String {
    format!("{}{}", API_BASE_URL, endpoint)
}
