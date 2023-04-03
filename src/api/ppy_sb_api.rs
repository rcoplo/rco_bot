use proc_qq::re_exports::{serde, serde_json};

static PPY_SB_API_URL: &str = "https://osu.ppy.sb/apiv2";


pub struct OsuSbApi {
    pub user_id:i32,
    pub user_name:String,
}

impl OsuSbApi {
    pub fn new() -> OsuSbApi {
        OsuSbApi {
            user_id: 0,
            user_name: "".to_string()
        }
    }

    pub async fn get_user_info(&self) -> crate::BotResult<OsuSbUserInfo> {
        let url = format!("{}/users/{}", PPY_SB_API_URL, self.user_name);
        let data = crate::utils::http_util::http_get(&url).await?;
        let result = serde_json::from_str::<OsuSbUserInfo>(data.as_str())?;
        Ok(result)
    }

}

#[derive(Debug,Clone,serde::Serialize,serde::Deserialize)]
pub struct OsuSbUserInfo{
    pub id:i32,
    pub username:String,
    pub avatar_url:String,
    pub country_code:String,
    pub last_visit:String,
    pub join_date:String,
    pub statistics:Statistics,
    pub playmode:String,
    pub previous_usernames:Vec<String>,
    pub is_active:bool,
    pub is_bot:bool,
    pub is_deleted:bool,
    pub is_online:bool,
    pub is_supporter:bool,
    pub pm_friends_only:bool,
    pub profile_colour:String,
    pub cover_url:String,
    pub has_supported:bool,
    pub max_blocks:i32,
    pub max_friends:i32,
    pub playstyle: Vec<String>,
    pub post_count:i32,
    pub profile_order: Vec<String>,
    pub cover: OsuSbCover,
}

#[derive(Debug,Clone,serde::Serialize,serde::Deserialize)]
pub struct Statistics{
    pub level:OsuSbLevel,
    pub grade_counts: GradeCounts,
    pub rank: OsuSbRank,
    pub pp: f32,
    pub global_rank: i64,
    pub ranked_score: i64,
    pub hit_accuracy: f64,
    pub play_count: i64,
    pub play_time: i64,
    pub total_score: i64,
    pub maximum_combo: i64,
    pub total_hits: i64,
    pub replays_watched_by_others: f32,
    pub is_ranked: bool,
}

#[derive(Debug,Clone,serde::Serialize,serde::Deserialize)]
pub struct OsuSbLevel{
    pub current:i32,
    pub progress:i32,
}

#[derive(Debug,Clone,serde::Serialize,serde::Deserialize)]
pub struct GradeCounts {
    pub ss:i32,
    pub ssh:i32,
    pub s:i32,
    pub sh:i32,
    pub a:i32,
}

#[derive(Debug,Clone,serde::Serialize,serde::Deserialize)]
pub struct OsuSbRank{
    pub global:i32,
    pub country:i32,
}

#[derive(Debug,Clone,serde::Serialize,serde::Deserialize)]
pub struct OsuSbCover{
    pub custom_url:String,
    pub url:String,
}
