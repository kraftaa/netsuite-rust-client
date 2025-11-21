use anyhow::Result;
use netsuite_client::{NetSuiteClient, Customer};
use netsuite_client::AppConfig;
use tracing::{info, warn, error};

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt::init();
    
    info!("=== NetSuite Client Example ===");
    
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
    info!("Testing connection to NetSuite...");
    match client.test_connection().await {
        Ok(_) => info!("✅ Successfully connected to NetSuite"),
        Err(e) => {
            warn!("⚠️  Connection test failed (expected without real credentials): {}", e);
            info!("This is normal when using placeholder credentials");
        }
    }
    
    // Example: Try to fetch customers (will fail without real credentials)
    info!("Attempting to fetch customers...");
    match client.get_customers(Some(5)).await {
        Ok(customers) => {
            info!("✅ Successfully fetched {} customers", customers.len());
            for customer in customers.iter().take(3) {
                info!("   - {} ({})", customer.entityid, customer.companyname.as_deref().unwrap_or("No company name"));
            }
        }
        Err(e) => {
            warn!("⚠️  Failed to fetch customers (expected without real credentials): {}", e);
            info!("This is normal when using placeholder credentials");
        }
    }
    
    info!("=== Example completed ===");
    Ok(())
}

// Example of how to work with customer data
fn process_customers(customers: Vec<Customer>) {
    info!("Processing {} customers", customers.len());
    
    let active_customers: Vec<&Customer> = customers
        .iter()
        .filter(|c| c.companyname.is_some())
        .collect();
    
    info!("Found {} customers with company names", active_customers.len());
    
    // Example: Group by company name length
    let mut by_name_length: std::collections::HashMap<usize, Vec<&Customer>> = std::collections::HashMap::new();
    
    for customer in active_customers {
        let name_length = customer.companyname.as_ref().unwrap().len();
        by_name_length.entry(name_length).or_insert_with(Vec::new).push(customer);
    }
    
    for (length, customers) in by_name_length {
        info!("{} customers have company names with {} characters", customers.len(), length);
    }
}
