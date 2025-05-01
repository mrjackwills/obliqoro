use serde::Deserialize;
use tokio::sync::broadcast::Sender;

use crate::{
    app_error::AppError,
    internal_message_handler::{InternalMessage, PackageInfo},
};

// Get a reqwest client, use application name and version an UserAgent, this should never err
fn get_client() -> Result<reqwest::Client, AppError> {
    reqwest::Client::builder()
        .connect_timeout(std::time::Duration::from_millis(5000))
        .gzip(true)
        .brotli(true)
        .user_agent(format!(
            "{}/{}",
            env!("CARGO_PKG_NAME"),
            env!("CARGO_PKG_VERSION")
        ))
        .build()
        .map_err(|i| AppError::Internal(i.to_string()))
}

#[derive(Debug, Deserialize)]
struct GitHubResponse {
    tag_name: String,
}

/// Check github to see if a new version is available, is executed in own thread, send result to frontend
pub fn parse_github(sx: Sender<InternalMessage>) {
    tokio::spawn(async move {
        let Ok(client) = get_client() else {
            return;
        };
        let url = "https://api.github.com/repos/mrjackwills/obliqoro/releases/latest";
        let Ok(response) = client.get(url).send().await else {
            return;
        };
        let Ok(body) = response.json::<GitHubResponse>().await else {
            return;
        };
        let latest_version = body.tag_name.replace('v', "");
        let info = PackageInfo {
            github_version: Some(latest_version),
            ..Default::default()
        };
        sx.send(InternalMessage::ToFrontEnd(
            crate::request_handlers::FrontEnd::PackageInfo(info),
        ))
        .ok();
    });
}
