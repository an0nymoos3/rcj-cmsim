use anyhow::Result;
use mongodb::{options::ClientOptions, Client};

pub async fn connect_to_mongo() -> Result<Client> {
    // Parse a connection string into an options struct.
    let mut client_options = ClientOptions::parse("mongodb://localhost:27017").await?;

    // Manually set an option.
    client_options.app_name = Some("RCJ-CMSim".to_string());

    // Get a handle to the deployment.
    Client::with_options(client_options)
}
