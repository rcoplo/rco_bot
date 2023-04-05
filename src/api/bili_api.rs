use proc_qq::re_exports::{serde_json, tracing};
use proc_qq::re_exports::tracing::info;
use crate::utils::http_util::http_get;

static BILI_LIVE: &'static str = "https://api.live.bilibili.com/room/v1/Room/get_info";
static BILI_API: &'static str = "https://api.bilibili.com/x/space/acc/info";

#[derive(Debug, Clone)]
pub struct BiliApiRoom {
    pub room_id: i64,
    pub uid: i64,
    pub title: String,
    pub user_cover: String,
    pub live_status: i32,
}

#[derive(Debug, Clone)]
pub struct BiliApiUp {
    pub uid: i64,
    pub room_id: i64,
    pub uname: String,
    pub face: String,
    pub title: String,
    pub user_cover: String,
    pub live_status: i32,
}

impl BiliApiRoom {
    /// [获取直播间信息](https://github.com/SocialSisterYi/bilibili-API-collect/blob/master/live/info.md#%E8%8E%B7%E5%8F%96%E7%9B%B4%E6%92%AD%E9%97%B4%E4%BF%A1%E6%81%AF)
    pub async fn get_live_info(room_id: i64) -> Option<BiliApiRoom> {
        let data = http_get(format!("{}?room_id={}", BILI_LIVE, room_id).as_str()).await;
        match data {
            Ok(data) => {
                let v = serde_json::from_str::<serde_json::Value>(data.as_str()).unwrap();
                Some(Self {
                    room_id,
                    uid: v["data"]["uid"].as_i64().unwrap_or_default(),
                    title: v["data"]["title"].as_str().unwrap_or_default().to_string(),
                    user_cover: v["data"]["user_cover"].as_str().unwrap_or_default().to_string(),
                    live_status: v["data"]["live_status"].as_i64().unwrap_or_default() as i32,
                })
            },
            Err(err) => None,
        }
    }
}

impl BiliApiUp {
    /// [获取主播信息](https://github.com/SocialSisterYi/bilibili-API-collect/blob/master/live/info.md#%E8%8E%B7%E5%8F%96%E4%B8%BB%E6%92%AD%E4%BF%A1%E6%81%AF)
    pub async fn get_user_info(uid: i64) -> Option<BiliApiUp> {
        let data = http_get(format!("{}?mid={}", BILI_API, uid).as_str()).await;
        match data {
            Ok(data) => {
                let v = serde_json::from_str::<serde_json::Value>(data.as_str()).unwrap();
                Some(Self {
                    room_id: v["data"]["live_room"]["roomid"].as_i64().unwrap_or_default(),
                    uid,
                    uname: v["data"]["name"].as_str().unwrap_or_default().to_string(),
                    face: v["data"]["face"].as_str().unwrap_or_default().to_string(),
                    title: v["data"]["live_room"]["title"].as_str().unwrap_or_default().to_string(),
                    user_cover: v["data"]["live_room"]["cover"].as_str().unwrap_or_default().to_string(),
                    live_status: v["data"]["live_room"]["liveStatus"].as_i64().unwrap_or_default() as i32,
                })
            },
            Err(err) => None,
        }
    }
}

