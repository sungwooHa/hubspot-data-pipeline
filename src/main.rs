use dotenv::dotenv;
use hubspot_data_pipeline::cli;
use hubspot_data_pipeline::utils::error::{HubSpotError, Result};
use log::info;
use std::env;

#[tokio::main]
async fn main() -> Result<()> {
    // 환경 변수 로드
    dotenv().ok();

    // 로깅 설정
    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "info");
    }
    env_logger::init();

    info!("Starting HubSpot Data Fetcher");

    // API 키 확인
    if env::var("HUBSPOT_API_KEY").is_err() {
        return Err(HubSpotError::MissingEnvVar("HUBSPOT_API_KEY".to_string()));
    }

    // CLI 실행
    cli::run().await?;

    info!("HubSpot Data Fetcher completed successfully");

    Ok(())
}
