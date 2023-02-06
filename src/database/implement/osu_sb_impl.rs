use std::fmt::Formatter;
use rbatis::rbdc::Error;
use crate::{BotError, BotResult, pool};
use crate::api::ppy_sb_api::OsuSbApi;
use crate::database::table::OsuSb;

pub struct OsuSbImpl{}

impl OsuSbImpl{
    pub async fn bind_by_user_name(&self,user_name:&String,user_id_qq:&i64) -> BotResult<bool>{
        let osu_sb = self.select_user_by_user_name(user_name).await;
        match osu_sb {
            Ok(sb) => {
                Err(BotError::from(format!("你已绑定一位为 {}({}) 的用户喵...", sb.user_name, sb.user_id)))
            }
            Err(_) => {
                let result = OsuSbApi::new().get_user_info().await;
                match result {
                    Ok(data) => {
                       let osu =  OsuSb{
                            user_id: data.id,
                            user_name: data.username,
                            user_id_qq: *user_id_qq,
                            mode: "osu".to_string(),
                            ..Default::default()
                        };
                        OsuSb::insert(pool!(),&osu).await?;
                        Ok(true)
                    }
                    Err(err) => {
                        Err(BotError::from(err))
                    }
                }

            }
        }

        
    }
    
    pub async fn select_user_by_user_name(&self,user_name:&String) -> BotResult<OsuSb>{
        let result = OsuSb::select_user_by_user_name(pool!(),user_name).await;
        match result {
            Ok(data) => {
                Ok(data.unwrap())
            }
            Err(err) => {
                Err(BotError::from(err))
            }
        }
    }

    pub async fn update_mode_by_user_id(&self,user_id:&i32,mode:OsuSbMode) -> BotResult<bool>{
        let result = OsuSb::select_user_by_user_id(pool!(),user_id).await;
        match result {
            Ok(data) => {
                let sb = data.unwrap();
                let osu_sb =  OsuSb{
                    mode:mode.to_string(),
                    ..sb
                };
                OsuSb::update_by_column(pool!(), &osu_sb, "id").await?;
                Ok(true)
            }
            Err(err) => {
                Err(BotError::from("你还没有绑定账号喵... ,\n 请输入!bind 用户名 进行绑定喵!"))
            }
        }
    }

}


pub enum OsuSbMode {
    Osu, //戳泡泡
    Taiko, // 太鼓
    Fruits, // 接shift
    Mania, // 钢琴块
}


impl std::fmt::Display for OsuSbMode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            OsuSbMode::Osu => write!(f,"osu"),
            OsuSbMode::Taiko => write!(f,"taiko"),
            OsuSbMode::Fruits => write!(f,"fruits"),
            OsuSbMode::Mania => write!(f,"mania"),
        }
    }
}