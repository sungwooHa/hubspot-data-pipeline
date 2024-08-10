use super::ApiClient;
use crate::models::{Company, Contact, Deal};
use crate::utils::error::Result;

pub struct ContactsEndpoint;
pub struct CompaniesEndpoint;
pub struct DealsEndpoint;

impl ContactsEndpoint {
    pub async fn fetch_all(client: &ApiClient) -> Result<Vec<Contact>> {
        client.get("/crm/v3/objects/contacts").await
    }
}

impl CompaniesEndpoint {
    pub async fn fetch_all(client: &ApiClient) -> Result<Vec<Company>> {
        client.get("/crm/v3/objects/companies").await
    }
}

impl DealsEndpoint {
    pub async fn fetch_all(client: &ApiClient) -> Result<Vec<Deal>> {
        client.get("/crm/v3/objects/deals").await
    }
}
// 각 엔드포인트에 대해 필요한 추가 메서드 구현 (예: fetch_by_id, create, update, delete)
