use anyhow::Result;
use crate::netsuite_client::NetSuiteClient;
use crate::config::AppConfig;
use tracing::{info, warn};
use std::io::{self, Write};

pub struct Cli {
    client: NetSuiteClient,
}

impl Cli {
    pub fn new() -> Result<Self> {
        let config = AppConfig::load()?;
        let client = NetSuiteClient::new(config)?;
        Ok(Self { client })
    }
    
    pub async fn run(&self) -> Result<()> {
        info!("NetSuite Rust Client CLI");
        info!("Type 'help' for available commands, 'quit' to exit");
        
        loop {
            print!("netsuite> ");
            io::stdout().flush()?;
            
            let mut input = String::new();
            io::stdin().read_line(&mut input)?;
            let input = input.trim();
            
            match input {
                "help" => self.show_help(),
                "test" => self.test_connection().await,
                "customers" => self.list_customers().await,
                "quit" | "exit" => {
                    info!("👋 Goodbye!");
                    break;
                }
                "" => continue,
                _ => {
                    warn!("Unknown command: '{}'. Type 'help' for available commands.", input);
                }
            }
        }
        
        Ok(())
    }
    
    fn show_help(&self) {
        println!("\nAvailable commands:");
        println!("  help                  - Show this help message");
        println!("  test                  - Test connection to NetSuite");
        println!("  customers             - List customers (requires real credentials)");
        println!("  transactions          - List recent transactions (requires real credentials)");
        println!("  vendor_payments       - List vendor payments for May-Aug 2024 (requires real credentials)");
        println!("  vendor_payments_custom - List vendor payments for custom date range (requires real credentials)");
        println!("  quit                  - Exit the CLI");
        println!("  exit                  - Exit the CLI");
        println!();
    }
    
    async fn test_connection(&self) {
        info!("Testing connection to NetSuite...");
        match self.client.test_connection().await {
            Ok(_) => info!("✅ Successfully connected to NetSuite"),
            Err(e) => {
                warn!("⚠️  Connection test failed: {}", e);
                info!("This is normal when using placeholder credentials");
            }
        }
    }
    
    async fn list_customers(&self) {
        info!("Fetching customers...");
        match self.client.get_customers(Some(10)).await {
            Ok(customers) => {
                info!("✅ Successfully fetched {} customers", customers.len());
                for (i, customer) in customers.iter().enumerate().take(5) {
                    println!("  {}. {} ({})", 
                        i + 1, 
                        customer.entityid, 
                        customer.companyname.as_deref().unwrap_or("No company name")
                    );
                }
                if customers.len() > 5 {
                    println!("  ... and {} more", customers.len() - 5);
                }
            }
            Err(e) => {
                warn!("⚠️  Failed to fetch customers: {}", e);
                info!("This is normal when using placeholder credentials");
            }
        }
    }
    
    async fn list_transactions(&self) {
        info!("Fetching recent transactions...");
        match self.client.get_transactions_with_filters(&[], Some(10)).await {
            Ok(transactions) => {
                info!("✅ Successfully fetched {} transactions", transactions.len());
                for (i, transaction) in transactions.iter().enumerate().take(5) {
                    println!("  {}. {} - {} - ${:.2} ({})", 
                        i + 1, 
                        transaction.id,
                        transaction.transaction_type.as_deref().unwrap_or("Unknown"),
                        transaction.amount.unwrap_or(0.0),
                        transaction.createddate.as_deref().unwrap_or("No date")
                    );
                }
                if transactions.len() > 5 {
                    println!("  ... and {} more", transactions.len() - 5);
                }
            }
            Err(e) => {
                warn!("⚠️  Failed to fetch transactions: {}", e);
                info!("This is normal when using placeholder credentials");
            }
        }
    }
    
    async fn list_vendor_payments(&self) {
        info!("Fetching vendor payments for May-Aug 2024...");
        match self.client.get_vendor_payments_2024(Some(20)).await {
            Ok(transactions) => {
                info!("✅ Successfully fetched {} vendor payments", transactions.len());
                for (i, transaction) in transactions.iter().enumerate().take(5) {
                    println!("  {}. {} - ${:.2} - {} ({})", 
                        i + 1, 
                        transaction.id,
                        transaction.amount.unwrap_or(0.0),
                        transaction.memo.as_deref().unwrap_or("No memo"),
                        transaction.createddate.as_deref().unwrap_or("No date")
                    );
                }
                if transactions.len() > 5 {
                    println!("  ... and {} more", transactions.len() - 5);
                }
            }
            Err(e) => {
                warn!("⚠️  Failed to fetch vendor payments: {}", e);
                info!("This is normal when using placeholder credentials");
            }
        }
    }
    
    async fn list_vendor_payments_custom(&self) {
        info!("Fetching vendor payments for custom date range...");
        
        // Example: Get vendor payments for Q1 2024
        let start_date = "2024-01-01";
        let end_date = "2024-03-31";
        
        info!("Date range: {} to {}", start_date, end_date);
        
        match self.client.get_vendor_payments(start_date, end_date, Some(20)).await {
            Ok(transactions) => {
                info!("✅ Successfully fetched {} vendor payments", transactions.len());
                for (i, transaction) in transactions.iter().enumerate().take(5) {
                    println!("  {}. {} - ${:.2} - {} ({})", 
                        i + 1, 
                        transaction.id,
                        transaction.amount.unwrap_or(0.0),
                        transaction.memo.as_deref().unwrap_or("No memo"),
                        transaction.createddate.as_deref().unwrap_or("No date")
                    );
                }
                if transactions.len() > 5 {
                    println!("  ... and {} more", transactions.len() - 5);
                }
            }
            Err(e) => {
                warn!("⚠️  Failed to fetch vendor payments: {}", e);
                info!("This is normal when using placeholder credentials");
            }
        }
    }
}
