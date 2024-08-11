use super::traits::{Fetchable, HubSpotObject};
use crate::api::ApiClient;
use crate::utils::error::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Deal {
    pub id: String,
    pub deal_name: Option<String>,
    pub amount: Option<f64>,
    pub stage: Option<String>,
    // Add more fields as needed
}

impl HubSpotObject for Deal {
    fn id(&self) -> &str {
        &self.id
    }

    fn object_type() -> &'static str {
        "deal"
    }
}

#[async_trait]
impl Fetchable for Deal {
    async fn fetch_all(client: &ApiClient) -> Result<Vec<Self>> {
        client.get::<Vec<Self>>("/crm/v3/objects/deals").await
    }

    async fn fetch_by_id(client: &ApiClient, id: &str) -> Result<Self> {
        client
            .get::<Self>(&format!("/crm/v3/objects/deals/{}", id))
            .await
    }
}
