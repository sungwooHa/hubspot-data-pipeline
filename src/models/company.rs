use super::traits::{Fetchable, HubSpotObject};
use crate::api::ApiClient;
use crate::utils::error::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Company {
    pub id: String,
    pub name: Option<String>,
    pub domain: Option<String>,
    // Add more fields as needed
}

impl HubSpotObject for Company {
    fn id(&self) -> &str {
        &self.id
    }

    fn object_type() -> &'static str {
        "company"
    }
}

#[async_trait]
impl Fetchable for Company {
    async fn fetch_all(client: &ApiClient) -> Result<Vec<Self>> {
        client.get::<Vec<Self>>("/crm/v3/objects/companies").await
    }

    async fn fetch_by_id(client: &ApiClient, id: &str) -> Result<Self> {
        client
            .get::<Self>(&format!("/crm/v3/objects/companies/{}", id))
            .await
    }
}
