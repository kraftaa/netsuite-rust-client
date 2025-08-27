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
