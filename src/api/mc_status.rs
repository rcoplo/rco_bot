use crate::{BotError, BotResult};
use crate::utils::http_util::http_get;

const STATUS_API: &str = "https://minecraft.pleshkov.dev/status/";


pub async fn get_minecraft_status(url: &str) -> BotResult<McStatus> {
    let data = http_get(format!("{}{}", STATUS_API, url).as_str()).await?;
    match serde_json::from_str::<McStatus>(data.as_str()) {
        Ok(data) => Ok(data),
        Err(_) => Err(BotError::from("获取服务器信息失败...")),
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct McStatus {
    pub description: McStatusDescriptionText,
    pub players: McStatusPlayers,
    pub version: McStatusVersion,
    pub favicon: String,
    #[serde(rename = "forgeData")]
    pub forge_data: Option<McStatusForgeData>,
    pub ping: i32,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct McStatusDescriptionText {
    pub text: String,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct McStatusPlayers {
    pub max: i32,
    pub online: i32,
    pub sample: Vec<Sample>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Sample {
    pub id: String,
    pub name: String,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct McStatusVersion {
    pub name: String,
    pub protocol: i32,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct McStatusForgeData {
    #[serde(rename = "fmlNetworkVersion")]
    pub fml_network_version: i32,
    pub d: String,
    pub channels: serde_json::Value,
    pub mods: serde_json::Value,
    pub truncated: bool,
}

