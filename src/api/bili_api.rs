use reqwest::Error;
use crate::utils::http_util::http_get;

static BILI_LIVE: &'static str = "https://api.live.bilibili.com/room/v1/Room/get_info";
static BILI_USER: &'static str = "https://api.live.bilibili.com/live_user/v1/Master/info";
#[derive(Debug,Default)]
pub struct BiliApi{
    pub room_id:i64,
    pub uid:i64,
    pub uname:String,
}

impl BiliApi {
    pub async fn new(uid:&i64) -> Self {
        let data = http_get(&format!("{}?uid={}", BILI_USER, uid)).await;
        match data {
            Ok(data) => {
                let json = serde_json::from_str::<serde_json::Value>(data.as_str()).unwrap();
                Self{
                    room_id: json["data"]["room_id"].as_i64().unwrap(),
                    uid:*uid,
                    uname: json["data"]["info"]["uname"].to_string(),
                }
            }
            Err(_) => {
                Self::default()
            }
        }
    }

    /// [获取直播间信息](https://github.com/SocialSisterYi/bilibili-API-collect/blob/master/live/info.md#%E8%8E%B7%E5%8F%96%E7%9B%B4%E6%92%AD%E9%97%B4%E4%BF%A1%E6%81%AF)
    pub async fn get_live_info(&self) -> Option<serde_json::Value>{
        let data = http_get(&format!("{}?room_id={}", BILI_LIVE, self.room_id)).await;
        match data {
            Ok(data) => {
                Some(serde_json::from_str::<serde_json::Value>(data.as_str()).unwrap())
            }
            Err(err) => {
                None
            }
        }

    }

    /// [获取主播信息](https://github.com/SocialSisterYi/bilibili-API-collect/blob/master/live/info.md#%E8%8E%B7%E5%8F%96%E4%B8%BB%E6%92%AD%E4%BF%A1%E6%81%AF)

    pub async fn get_user_info(&self)-> Option<serde_json::Value>{
        let data = http_get(&format!("{}?uid={}", BILI_USER, self.uid)).await;
        match data {
            Ok(data) => {
                Some(serde_json::from_str::<serde_json::Value>(data.as_str()).unwrap())
            }
            Err(err) => {
                None
            }
        }
    }
}