use super::traits::{Fetchable, HubSpotObject};
use crate::api::ApiClient;
use crate::utils::error::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Contact {
    pub id: String,
    pub firstname: Option<String>,
    pub lastname: Option<String>,
    pub email: Option<String>,
    // Add more fields as needed
}

impl HubSpotObject for Contact {
    fn id(&self) -> &str {
        &self.id
    }

    fn object_type() -> &'static str {
        "contact"
    }
}

#[async_trait]
impl Fetchable for Contact {
    async fn fetch_all(client: &ApiClient) -> Result<Vec<Self>> {
        client.get::<Vec<Self>>("/crm/v3/objects/contacts").await
    }

    async fn fetch_by_id(client: &ApiClient, id: &str) -> Result<Self> {
        client
            .get::<Self>(&format!("/crm/v3/objects/contacts/{}", id))
            .await
    }
}
