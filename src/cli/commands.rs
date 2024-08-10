use crate::config::Config;
use crate::models::{Company, Contact, Deal};
use crate::utils::error::Result;
use crate::HubSpotFetcher;
use clap::Subcommand;

#[derive(Subcommand)]
pub enum Command {
    /// Fetch all contacts from HubSpot
    FetchContacts,
    /// Fetch all companies from HubSpot
    FetchCompanies,
    /// Fetch all deals from HubSpot
    FetchDeals,
}

pub async fn execute(command: Command) -> Result<()> {
    let config = Config::from_env()?;
    let fetcher = HubSpotFetcher::new(config);

    match command {
        Command::FetchContacts => {
            let contacts = fetcher.fetch_contacts().await?;
            print_results("Contacts", contacts);
        }
        Command::FetchCompanies => {
            let companies = fetcher.fetch_companies().await?;
            print_results("Companies", companies);
        }
        Command::FetchDeals => {
            let deals = fetcher.fetch_deals().await?;
            print_results("Deals", deals);
        }
    }

    Ok(())
}

fn print_results<T: std::fmt::Debug>(entity_type: &str, entities: Vec<T>) {
    println!("Fetched {} {}:", entities.len(), entity_type);
    for entity in entities {
        println!("{:#?}", entity);
    }
}
