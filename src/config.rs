#[derive(Debug,Clone,serde::Serialize,serde::Deserialize)]
pub struct RcoBotConfig {
    pub debug: bool,
    pub chrome_driver_url: String,
    pub login_type:String,
    pub account:Account,
    pub setu:SetuConfig,
    pub super_admin:Vec<String>,
    pub bot_name:Vec<String>,
}
#[derive(Debug,Clone,serde::Serialize,serde::Deserialize)]
pub struct Account {
    pub uin:i64,
    pub pwd:String,
}

#[derive(Debug,Clone,serde::Serialize,serde::Deserialize)]
pub struct SetuConfig{
    pub recall_time:i32,
    pub whether_to_save_locally:bool,
}


impl Default for RcoBotConfig {
    fn default() -> Self {
        let yml_data = std::fs::read_to_string("./resources/config/botconfig.yml")
            .expect("config file not found");
        let config = serde_yaml::from_str::<RcoBotConfig>(&yml_data).unwrap();
        config
    }
}