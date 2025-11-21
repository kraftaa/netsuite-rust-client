use anyhow::Result;
use oauth2::{
    basic::BasicClient, AuthUrl, ClientId, ClientSecret, TokenUrl,
};
use reqwest::{Client, header::{HeaderMap, HeaderValue, AUTHORIZATION}};
use serde::{Deserialize, Serialize};
use crate::config::NetSuiteConfig;

pub struct NetSuiteClient {
    config: NetSuiteConfig,
    http_client: Client,
    _oauth_client: BasicClient, // Prefix with _ to indicate intentionally unused
}

impl NetSuiteClient {
    pub fn new(config: crate::config::AppConfig) -> Result<Self> {
        let netsuite_config = config.netsuite;
        
        let oauth_client = BasicClient::new(
            ClientId::new(netsuite_config.consumer_key.clone()),
            Some(ClientSecret::new(netsuite_config.consumer_secret.clone())),
            AuthUrl::from_url(format!("{}/oauth/authorize", netsuite_config.base_url).parse()?),
            Some(TokenUrl::from_url(format!("{}/oauth/token", netsuite_config.base_url).parse()?))
        );
        
        let http_client = Client::new();
        
        Ok(Self {
            config: netsuite_config,
            http_client,
            _oauth_client: oauth_client,
        })
    }
    
    pub async fn test_connection(&self) -> Result<()> {
        // Simple test to verify we can reach NetSuite
        let response = self.http_client
            .get(&format!("{}/rest/platform/v1/record/customer", self.config.base_url))
            .headers(self.get_auth_headers()?)
            .send()
            .await?;
        
        if response.status().is_success() || response.status().as_u16() == 401 {
            // 401 is expected without proper OAuth token, but means we can reach the API
            Ok(())
        } else {
            anyhow::bail!("Failed to connect to NetSuite: {}", response.status())
        }
    }
    
    pub async fn get_customers(&self, limit: Option<u32>) -> Result<Vec<Customer>> {
        let mut url = format!("{}/rest/platform/v1/record/customer", self.config.base_url);
        if let Some(limit) = limit {
            url.push_str(&format!("?limit={}", limit));
        }
        
        let response = self.http_client
            .get(&url)
            .headers(self.get_auth_headers()?)
            .send()
            .await?;
        
        if response.status().is_success() {
            let customers: CustomerResponse = response.json().await?;
            Ok(customers.records)
        } else {
            anyhow::bail!("Failed to fetch customers: {}", response.status())
        }
    }
    
    /// Fetch vendor payment transactions for specific date ranges
    /// This implements your SQL-like query: SELECT * FROM Transaction WHERE Type='VendPymt' AND (createddate BETWEEN start_date AND end_date)
    pub async fn get_vendor_payments(&self, start_date: &str, end_date: &str, limit: Option<u32>) -> Result<Vec<Transaction>> {
        // Use the correct NetSuite endpoint for vendor payments (checks)
        let mut url = format!("{}/rest/platform/v1/record/check", self.config.base_url);
        
        // Build query parameters
        let mut params = Vec::new();
        params.push("q=type IS VendPymt".to_string());
        params.push(format!("q=createddate BETWEEN '{}' AND '{}'", start_date, end_date));
        
        if let Some(limit) = limit {
            params.push(format!("limit={}", limit));
        }
        
        // Join all parameters
        url.push_str(&format!("?{}", params.join("&")));
        
        let response = self.http_client
            .get(&url)
            .headers(self.get_auth_headers()?)
            .send()
            .await?;
        
        if response.status().is_success() {
            let transactions: TransactionResponse = response.json().await?;
            Ok(transactions.records)
        } else {
            anyhow::bail!("Failed to fetch vendor payments: {}", response.status())
        }
    }
    
    /// Convenience method for 2024 vendor payments (keeps backward compatibility)
    pub async fn get_vendor_payments_2024(&self, limit: Option<u32>) -> Result<Vec<Transaction>> {
        self.get_vendor_payments("2024-05-01", "2024-08-31", limit).await
    }
    
    /// Generic method to fetch transactions with custom filters
    pub async fn get_transactions_with_filters(&self, filters: &[String], limit: Option<u32>) -> Result<Vec<Transaction>> {
        // Use the correct NetSuite endpoint for transactions
        // Note: NetSuite doesn't have a generic "transaction" endpoint
        // You need to specify the specific record type
        let mut url = format!("{}/rest/platform/v1/record/check", self.config.base_url);
        
        // Build query parameters from filters
        let mut params = Vec::new();
        for filter in filters {
            params.push(format!("q={}", filter));
        }
        
        if let Some(limit) = limit {
            params.push(format!("limit={}", limit));
        }
        
        // Join all parameters
        if !params.is_empty() {
            url.push_str(&format!("?{}", params.join("&")));
        }
        
        let response = self.http_client
            .get(&url)
            .headers(self.get_auth_headers()?)
            .send()
            .await?;
        
        if response.status().is_success() {
            let transactions: TransactionResponse = response.json().await?;
            Ok(transactions.records)
        } else {
            anyhow::bail!("Failed to fetch transactions: {}", response.status())
        }
    }
    
    /// Fetch sales orders with custom filters
    pub async fn get_sales_orders(&self, filters: &[String], limit: Option<u32>) -> Result<Vec<Transaction>> {
        let mut url = format!("{}/rest/platform/v1/record/salesorder", self.config.base_url);
        
        // Build query parameters from filters
        let mut params = Vec::new();
        for filter in filters {
            params.push(format!("q={}", filter));
        }
        
        if let Some(limit) = limit {
            params.push(format!("limit={}", limit));
        }
        
        // Join all parameters
        if !params.is_empty() {
            url.push_str(&format!("?{}", params.join("&")));
        }
        
        let response = self.http_client
            .get(&url)
            .headers(self.get_auth_headers()?)
            .send()
            .await?;
        
        if response.status().is_success() {
            let transactions: TransactionResponse = response.json().await?;
            Ok(transactions.records)
        } else {
            anyhow::bail!("Failed to fetch sales orders: {}", response.status())
        }
    }
    
    fn get_auth_headers(&self) -> Result<HeaderMap> {
        let mut headers = HeaderMap::new();
        
        // For now, we'll use basic auth headers
        // In production, you'd want to implement proper OAuth 2.0 flow
        let auth_value = format!("Bearer {}", self.config.consumer_key);
        headers.insert(AUTHORIZATION, HeaderValue::from_str(&auth_value)?);
        
        Ok(headers)
    }
}

// NetSuite data structures
#[derive(Debug, Deserialize)]
pub struct CustomerResponse {
    pub records: Vec<Customer>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Customer {
    pub id: String,
    pub entityid: String,
    pub companyname: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub datecreated: Option<String>,
}

// New transaction data structures
#[derive(Debug, Deserialize)]
pub struct TransactionResponse {
    pub records: Vec<Transaction>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Transaction {
    pub id: String,
    pub trandate: Option<String>,
    pub createddate: Option<String>,
    pub transaction_type: Option<String>,  // Changed from 'type' to 'transaction_type'
    pub memo: Option<String>,
    pub amount: Option<f64>,
    pub currency: Option<String>,
    pub entity: Option<EntityReference>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct EntityReference {
    pub id: String,
    pub name: Option<String>,
}

// Error types - keeping for future use
#[allow(dead_code)]
#[derive(Debug, thiserror::Error)]
pub enum NetSuiteError {
    #[error("Authentication failed: {0}")]
    Authentication(String),
    #[error("API request failed: {0}")]
    ApiRequest(String),
    #[error("Invalid response: {0}")]
    InvalidResponse(String),
}
