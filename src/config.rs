use serde::{Deserialize, Serialize};
use tokio::fs::File;
use tokio::io::AsyncReadExt;

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthConfig {
    pub name: String,
    pub client_id: String,
    pub scope: String,
    pub refer: String,
}

pub async fn get_config() -> Vec<AuthConfig> {
    let mut data = String::new();
    File::open("config.json")
        .await
        .unwrap()
        .read_to_string(&mut data)
        .await
        .unwrap();

    let data: Vec<AuthConfig> = serde_json::from_str(&data).unwrap();
    data
}
