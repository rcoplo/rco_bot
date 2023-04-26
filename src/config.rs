use crate::resource_path;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RcoBotConfig {
    pub log: String,
    pub bot_config: BotConfig,
    pub ett: EttConfig,
    pub setu: SetuConfig,
    pub apex_api: String,
    pub sign_config: SignConfig,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct BotConfig {
    pub login_type: String,
    pub super_admin: Vec<String>,
    pub bot_name: Vec<String>,
    pub account_uin: i64,
    pub account_pwd: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct EttConfig {
    pub uin: String,
    pub pwd: String,
    pub cooldown: Option<i32>,
    pub timeout: Option<i32>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SetuConfig {
    pub recall_time: i32,
    pub whether_to_save_locally: bool,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SignConfig {
    pub scope: f64,
}


impl Default for RcoBotConfig {
    fn default() -> Self {
        let yml_data = std::fs::read_to_string(resource_path!("config","botconfig.yml"))
            .expect("config file not found");
        let config = serde_yaml::from_str::<RcoBotConfig>(&yml_data).unwrap();
        config
    }
}