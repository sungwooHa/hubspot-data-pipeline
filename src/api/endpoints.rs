use super::ApiClient;
use crate::models::{Company, Contact, Deal};
use crate::utils::error::Result;
use serde::Serialize;

pub struct ContactsEndpoint;
pub struct CompaniesEndpoint;
pub struct DealsEndpoint;

#[derive(Serialize)]
struct QueryParams<'a> {
    limit: usize,
    #[serde(skip_serializing_if = "Option::is_none")]
    properties: Option<Vec<&'a str>>,
    archived: bool,
}

impl ContactsEndpoint {
    pub async fn fetch_all(
        client: &ApiClient,
        properties: Option<&[String]>,
    ) -> Result<Vec<Contact>> {
        let query = QueryParams {
            limit: 100, // 또는 원하는 기본값
            properties: properties.map(|props| props.iter().map(AsRef::as_ref).collect()),
            archived: false,
        };
        client
            .get_with_query("/crm/v3/objects/contacts", &query)
            .await
    }
}

impl CompaniesEndpoint {
    pub async fn fetch_all(
        client: &ApiClient,
        properties: Option<&[String]>,
    ) -> Result<Vec<Company>> {
        let query = QueryParams {
            limit: 100,
            properties: properties.map(|props| props.iter().map(AsRef::as_ref).collect()),
            archived: false,
        };
        client
            .get_with_query("/crm/v3/objects/companies", &query)
            .await
    }
}

impl DealsEndpoint {
    pub async fn fetch_all(client: &ApiClient, properties: Option<&[String]>) -> Result<Vec<Deal>> {
        let query = QueryParams {
            limit: 100,
            properties: properties.map(|props| props.iter().map(AsRef::as_ref).collect()),
            archived: false,
        };
        client.get_with_query("/crm/v3/objects/deals", &query).await
    }
}
// 각 엔드포인트에 대해 필요한 추가 메서드 구현 (예: fetch_by_id, create, update, delete)
