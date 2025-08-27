use anyhow::Result;
use tracing::{info, error};

mod netsuite_client;
mod config;
mod cli;

use netsuite_client::NetSuiteClient;
use config::AppConfig;
use cli::Cli;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt::init();
    
    info!("Starting NetSuite Rust Client");
    
    // Check if user wants CLI mode
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 && args[1] == "cli" {
        // Run in CLI mode
        let cli = Cli::new()?;
        cli.run().await?;
        return Ok(());
    }
    
    // Run in basic mode
    run_basic_mode().await?;
    
    Ok(())
}

async fn run_basic_mode() -> Result<()> {
    info!("Running in basic mode");
    
    // Load configuration
    let config = AppConfig::load()?;
    info!("Configuration loaded successfully");
    
    // Initialize NetSuite client
    let client = NetSuiteClient::new(config)?;
    info!("NetSuite client initialized");
    
    // Example: Test connection
    match client.test_connection().await {
        Ok(_) => info!("Successfully connected to NetSuite"),
        Err(e) => error!("Failed to connect to NetSuite: {}", e),
    }
    
    Ok(())
}
