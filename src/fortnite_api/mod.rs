use reqwest::Client;
use serde_json::Value;
use std::collections::HashMap;

pub struct FortniteAPI {
    client: Client,
    auth_token: String,
    base_url: String,
}

impl FortniteAPI {
    pub fn new(auth_token: &str) -> Self {
        Self {
            client: Client::new(),
            auth_token: auth_token.to_string(),
            base_url: "https://api.fortnite.com/v2".to_string(),
        }
    }

    pub async fn unlock_all(&self) -> Result<Value, reqwest::Error> {
        self.client
            .post(&format!("{}/unlock_all", self.base_url))
            .header("Authorization", &self.auth_token)
            .send()
            .await?
            .json()
            .await
    }
}