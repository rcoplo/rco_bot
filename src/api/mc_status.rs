use proc_qq::re_exports::{serde, serde_json, tracing};
use crate::{BotError, BotResult};
use crate::utils::http_util::http_get;

const STATUS_API_JAVA: &str = "https://api.mcstatus.io/v2/status/java/";
const STATUS_API_BEDROCK: &str = "https://api.mcstatus.io/v2/status/bedrock/";


pub async fn get_minecraft_status_java(url: &str) -> BotResult<McStatusJava> {
    let data = http_get(format!("{}{}", STATUS_API_JAVA, url).as_str()).await?;
    match serde_json::from_str::<McStatusJava>(data.as_str()) {
        Ok(data) => Ok(data),
        Err(err) => Err(BotError::from(format!("获取服务器信息失败喵... Error: {}", err))),
    }
}

pub async fn get_minecraft_status_bedrock(url: &str) -> BotResult<McStatusBedrock> {
    let data = http_get(format!("{}{}", STATUS_API_BEDROCK, url).as_str()).await?;
    match serde_json::from_str::<McStatusBedrock>(data.as_str()) {
        Ok(data) => Ok(data),
        Err(err) => Err(BotError::from(format!("获取服务器信息失败喵... Error: {}", err))),
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct McStatusJava {
    pub online: bool,
    pub host: String,
    pub port: i32,
    pub eula_blocked: bool,
    pub retrieved_at: i64,
    pub expires_at: i64,
    pub version: McStatusVersionJava,
    pub players: McStatusPlayersJava,
    pub motd: McStatusMotdJava,
    pub icon: Option<String>,
    pub mods: Vec<McStatusModsJava>,

}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct McStatusVersionJava {
    pub name_raw: String,
    pub name_clean: String,
    pub name_html: String,
    pub protocol: i32,

}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct McStatusPlayersJava {
    pub online: i32,
    pub max: i32,
    pub list: Vec<McStatusListJava>,

}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct McStatusListJava {
    pub uuid: String,
    pub name_raw: String,
    pub name_clean: String,
    pub name_html: String,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct McStatusMotdJava {
    pub raw: String,
    pub clean: String,
    pub html: String,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct McStatusModsJava {
    pub name: String,
    pub version: String,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct McStatusBedrock {
    pub online: bool,
    pub host: String,
    pub port: i32,
    pub eula_blocked: bool,
    pub retrieved_at: i64,
    pub expires_at: i64,
    pub version: McStatusVersionBedrock,
    pub players: McStatusPlayersBedrock,
    pub motd: McStatusMotdBedrock,
    pub gamemode: String,
    pub server_id: String,
    pub edition: String,

}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct McStatusVersionBedrock {
    pub name: String,
    pub version: String,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct McStatusPlayersBedrock {
    pub online: String,
    pub max: String,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct McStatusMotdBedrock {
    pub raw: String,
    pub clean: String,
    pub html: String,
}

