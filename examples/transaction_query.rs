use anyhow::Result;
use netsuite_client::{NetSuiteClient, AppConfig};
use tracing::{info, warn, error};

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt::init();
    
    info!("=== NetSuite Transaction Query Example ===");
    
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
    
    // Example 1: Fetch vendor payments for May-Aug 2024
    info!("Example 1: Fetching vendor payments for May-Aug 2024...");
    info!("This implements: SELECT * FROM Transaction WHERE Type='VendPymt' AND (createddate like '%05/%/2024%' OR createddate like '%06/%/2024%' OR createddate like '%07/%/2024%' OR createddate like '%08/%/2024%')");

    match client.get_vendor_payments_2024(Some(10)).await {
        Ok(transactions) => {
            info!("✅ Successfully fetched {} vendor payments", transactions.len());
            for (i, transaction) in transactions.iter().enumerate().take(3) {
                info!("   {}. {} - ${:.2} - {} ({})", 
                    i + 1, 
                    transaction.id,
                    transaction.amount.unwrap_or(0.0),
                    transaction.memo.as_deref().unwrap_or("No memo"),
                    transaction.createddate.as_deref().unwrap_or("No date")
                );
            }
        }
        Err(e) => {
            warn!("⚠️  Failed to fetch vendor payments: {}", e);
            info!("This is normal when using placeholder credentials");
        }
    }
    
    // Example 2: Custom filters
    info!("Example 2: Using custom filters...");
    let custom_filters = vec![
        "type IS VendPymt".to_string(),
        "createddate BETWEEN '2024-05-01' AND '2024-08-31'".to_string(),
    ];
    
    match client.get_transactions_with_filters(&custom_filters, Some(10)).await {
        Ok(transactions) => {
            info!("✅ Successfully fetched {} transactions with custom filters", transactions.len());
            for (i, transaction) in transactions.iter().enumerate().take(3) {
                info!("   {}. {} - ${:.2} - {} ({})", 
                    i + 1, 
                    transaction.id,
                    transaction.amount.unwrap_or(0.0),
                    transaction.transaction_type.as_deref().unwrap_or("Unknown"),
                    transaction.createddate.as_deref().unwrap_or("No date")
                );
            }
        }
        Err(e) => {
            warn!("⚠️  Failed to fetch transactions with custom filters: {}", e);
            info!("This is normal when using placeholder credentials");
        }
    }
    
    // Example 3: Different transaction types
    info!("Example 3: Fetching different transaction types...");
    let sales_filters = vec![
        "type IS SalesOrd".to_string(),
        "createddate BETWEEN '2024-01-01' AND '2024-12-31'".to_string(),
    ];
    
    match client.get_transactions_with_filters(&sales_filters, Some(5)).await {
        Ok(transactions) => {
            info!("✅ Successfully fetched {} sales orders", transactions.len());
            for (i, transaction) in transactions.iter().enumerate().take(3) {
                info!("   {}. {} - {} - ${:.2}", 
                    i + 1, 
                    transaction.id,
                    transaction.transaction_type.as_deref().unwrap_or("Unknown"),
                    transaction.amount.unwrap_or(0.0)
                );
            }
        }
        Err(e) => {
            warn!("⚠️  Failed to fetch sales orders: {}", e);
            info!("This is normal when using placeholder credentials");
        }
    }
    
    info!("=== Transaction Query Examples Completed ===");
    Ok(())
}
