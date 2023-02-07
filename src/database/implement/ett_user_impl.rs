use rbatis::rbdc::db::ExecResult;
use rbatis::rbdc::Error;
use crate::{BotError, BotResult, pool};
use crate::database::table::EttUser;

pub struct EttUserImpl{}


impl EttUserImpl {
    pub async fn ett_build_by_name_qq(&self,user_name:&String,user_id_qq:&i64) -> BotResult<()>{
        match self.ett_select_by_name_qq(user_id_qq).await {
            Ok(data) => {
                Err(BotError::from(format!("你已经绑定了一位叫 {} 的用户,解绑请使用 /ett untie",data.user_name)))
            }
            Err(_) => {
                EttUser::insert(pool!(),&EttUser{
                    user_name: user_name.clone(),
                    user_id_qq: *user_id_qq,
                    ..Default::default()
                }).await?;
                Ok(())
            }
        }
    }

    pub async fn ett_select_by_name_qq(&self,user_id_qq:&i64) -> BotResult<EttUser>{
        let ett_user = EttUser::select_user_by_qq(pool!(), user_id_qq).await?;
        match ett_user{
            Some(data) => {
                Ok(data)
            }
            None => {
                Err(BotError::from(format!("你还没有绑定,请使用 \n /ett build ett用户名 进行绑定")))
            }
        }
    }

    pub async fn ett_untie_by_qq(&self,user_id_qq:&i64) -> BotResult<String>{
        match self.ett_select_by_name_qq(user_id_qq).await {
            Ok(_) => {
                EttUser::delete_by_column(pool!(), "user_id_qq", user_id_qq).await?;
                Ok("解绑成功喵!".to_string())
            }
            Err(_) => {
                Err(BotError::from(format!("你还没有绑定用户喵...")))
            }
        }
    }

}