use chrono::NaiveDateTime;
use crate::{BotError, BotResult, pool};
use crate::database::table::EttUser;


pub struct EttUserImpl {}


impl EttUserImpl {
    pub async fn ett_build_by_name_qq(&self, user_name: &str, user_id_qq: i64) -> BotResult<()> {
        match self.ett_select_by_name_qq(user_id_qq).await {
            Ok(data) => {
                Err(BotError::from(format!("你已经绑定了一位叫 {} 的用户,解绑请使用 /ett untie 喵!", data.user_name)))
            }
            Err(_) => {
                let option = EttUser::select_user_by_user_name(pool!(), user_name).await?;
                match option {
                    None => {
                        EttUser::insert(pool!(), &EttUser {
                            user_name: user_name.to_string(),
                            user_id_qq,
                            update_time: chrono::Local::now().naive_local(),
                            ..Default::default()
                        }).await?;
                    }
                    Some(data) => {
                        return Err(BotError::from(format!("你所绑定的用户名({})已经被({})绑定喵...,如果该用户名是你的请联系他喵!", data.user_name, data.user_id_qq)));
                    }
                }
                Ok(())
            }
        }
    }

    pub async fn ett_select_by_name_qq(&self, user_id_qq: i64) -> BotResult<EttUser> {
        let ett_user = EttUser::select_user_by_qq(pool!(), user_id_qq).await?;
        match ett_user {
            Some(data) => {
                Ok(data)
            }
            None => {
                Err(BotError::from(format!("你还没有绑定喵...,请使用 \n /ett build ett用户名 进行绑定喵!")))
            }
        }
    }

    pub async fn ett_untie_by_qq(&self, user_id_qq: i64) -> BotResult<String> {
        match self.ett_select_by_name_qq(user_id_qq).await {
            Ok(_) => {
                EttUser::delete_by_column(pool!(), "user_id_qq", user_id_qq).await?;
                Ok("解绑成功喵!".to_string())
            }
            Err(_) => {
                Err(BotError::from(format!("你还没有绑定喵...,请使用 \n /ett build ett用户名 进行绑定喵!")))
            }
        }
    }
    pub async fn ett_update_rating_time_by_qq(&self, user_id_qq: i64, rating: String, time: NaiveDateTime) -> BotResult<()> {
        match self.ett_select_by_name_qq(user_id_qq).await {
            Ok(data) => {
                EttUser::update_by_column(pool!(), &EttUser {
                    update_time: time,
                    rating,
                    ..data
                }, "user_id_qq").await?;
                Ok(())
            }
            Err(_) => {
                return Err(BotError::from(format!("你还没有绑定喵...,请使用 \n /ett build ett用户名 进行绑定喵!")));
            }
        }
    }
}