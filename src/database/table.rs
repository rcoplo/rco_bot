use chrono::NaiveDateTime;
use serde::{Deserialize, Deserializer};
use serde_json::Value;


pub fn deserde_from_int<'de, D>(deserializer: D) -> Result<bool, D::Error>
    where
        D: Deserializer<'de>,
{
    let i = u32::deserialize(deserializer)?;
    Ok(i == 0)
}


#[derive(Debug , Clone,Default,serde::Deserialize,serde::Serialize)]
pub struct BiliPush{
    pub id:i32,
    pub room_id:i64,
    pub uid:i64,
    pub uname:String,
    pub group_id:String,
    pub live_status:i32,
    pub latest_video_time:i64,
    pub latest_dynamic_time:i64,
    #[serde(deserialize_with = "deserde_from_int")]
    pub live_push:bool,
    #[serde(deserialize_with = "deserde_from_int")]
    pub video_push:bool,
    #[serde(deserialize_with = "deserde_from_int")]
    pub dynamic_push:bool,
}
#[derive(Debug , Clone,Default,serde::Deserialize,serde::Serialize)]
pub struct Sign {
    pub id:i32,
    pub sign_time:NaiveDateTime,
    pub user_id:i64,
    pub favorability:f64,
}

#[derive(Debug , Clone,Default,serde::Deserialize,serde::Serialize)]
pub struct OsuSb{
    pub id:i32,
    pub user_id:i32,
    pub user_name:String,
    pub user_id_qq:i64,
    pub mode:String
}
#[derive(Debug , Clone,Default,serde::Deserialize,serde::Serialize)]
pub struct EttUser{
    pub id:i32,
    pub user_name:String,
    pub user_id_qq:i64,
    pub rating:String,
    pub custom_background:String,
}
