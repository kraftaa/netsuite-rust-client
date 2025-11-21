use anyhow::Result;
use netsuite_client::{NetSuiteClient, AppConfig};
use tracing::{info, warn, error};

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt::init();
    
    info!("=== My Custom NetSuite App ===");
    
    // Load configuration
    let config = match AppConfig::load() {
        Ok(config) => {
            info!("✅ Configuration loaded successfully");
            config
        }
        Err(e) => {
            error!("❌ Failed to load configuration: {}", e);
            return Err(e);
        }
    };
    
    // Initialize NetSuite client
    let client = match NetSuiteClient::new(config) {
        Ok(client) => {
            info!("✅ NetSuite client initialized");
            client
        }
        Err(e) => {
            error!("❌ Failed to initialize NetSuite client: {}", e);
            return Err(e);
        }
    };
    
    // Test connection
    info!("🔍 Testing connection to NetSuite...");
    match client.test_connection().await {
        Ok(_) => info!("✅ Successfully connected to NetSuite"),
        Err(e) => {
            warn!("⚠️  Connection test failed: {}", e);
            info!("This might be normal if using placeholder credentials");
        }
    }
    
    // Try to fetch customers
    info!("👥 Attempting to fetch customers...");
    match client.get_customers(Some(5)).await {
        Ok(customers) => {
            info!("✅ Successfully fetched {} customers", customers.len());
            for (i, customer) in customers.iter().enumerate().take(3) {
                info!("   {}. {} ({})", 
                    i + 1, 
                    customer.entityid, 
                    customer.companyname.as_deref().unwrap_or("No company name")
                );
            }
        }
        Err(e) => {
            warn!("⚠️  Failed to fetch customers: {}", e);
            info!("This might be normal if using placeholder credentials");
        }
    }
    
    info!("=== App completed successfully ===");
    Ok(())
}