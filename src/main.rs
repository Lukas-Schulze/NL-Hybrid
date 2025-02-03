mod gui;
mod fortnite_api;
mod config;
mod logger;

use gui::Interface;
use fortnite_api::FortniteAPI;
use config::UserConfig;
use logger::setup_logger;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    setup_logger();
    let config = UserConfig::load().unwrap_or_default();
    let api_client = FortniteAPI::new(&config.auth_token);
    Interface::run(api_client, config).await
}