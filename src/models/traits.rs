use crate::api::ApiClient;
use crate::utils::error::Result;
use async_trait::async_trait;
use serde::{de::DeserializeOwned, Serialize};

pub trait HubSpotObject: Serialize + DeserializeOwned {
    fn id(&self) -> &str;
    fn object_type() -> &'static str;
}

#[async_trait]
pub trait Fetchable: HubSpotObject {
    async fn fetch_all(client: &ApiClient) -> Result<Vec<Self>>;
    async fn fetch_by_id(client: &ApiClient, id: &str) -> Result<Self>;
}

// #[async_trait]
// pub trait Creatable: HubSpotObject {
//     async fn create(client: &ApiClient, data: &Self) -> Result<Self>;
// }

// #[async_trait]
// pub trait Updatable: HubSpotObject {
//     async fn update(client: &ApiClient, data: &Self) -> Result<Self>;
// }

// #[async_trait]
// pub trait Deletable: HubSpotObject {
//     async fn delete(client: &ApiClient, id: &str) -> Result<()>;
// }
