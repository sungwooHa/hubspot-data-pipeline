use crate::config::Config;
use crate::utils::error::Result;
use crate::utils::file::read_properties;
use crate::HubSpotFetcher;
use clap::Subcommand;
use std::path::PathBuf;

#[derive(Subcommand)]
pub enum Command {
    /// Fetch all contacts from HubSpot
    FetchContacts {
        #[clap(short, long)]
        properties_file: Option<PathBuf>,
    },
    /// Fetch all companies from HubSpot
    FetchCompanies {
        #[clap(short, long)]
        properties_file: Option<PathBuf>,
    },
    /// Fetch all deals from HubSpot
    FetchDeals {
        #[clap(short, long)]
        properties_file: Option<PathBuf>,
    },
}

pub async fn execute(command: Command) -> Result<()> {
    let config = Config::from_env()?;
    let fetcher = HubSpotFetcher::new(config);
    match command {
        Command::FetchContacts { properties_file } => {
            let properties = properties_file.map(read_properties).transpose()?;
            let contacts = fetcher.fetch_contacts(properties.as_deref()).await?;
            print_results("Contacts", contacts);
        }
        Command::FetchCompanies { properties_file } => {
            let properties = properties_file.map(read_properties).transpose()?;
            let companies = fetcher.fetch_companies(properties.as_deref()).await?;
            print_results("Companies", companies);
        }
        Command::FetchDeals { properties_file } => {
            let properties = properties_file.map(read_properties).transpose()?;
            let deals = fetcher.fetch_deals(properties.as_deref()).await?;
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
