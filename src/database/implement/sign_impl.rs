use chrono::NaiveDateTime;
use crate::{BotError, BotResult, pool};
use crate::database::table::Sign;

pub struct SignImpl{}


impl SignImpl {
    pub async fn insert(&self,sign:&Sign) ->BotResult<bool>{
        Sign::insert(pool!(), sign).await?;
        Ok(true)
    }

    pub async fn update_sign_time(&self,sign_time:&NaiveDateTime,user_id:&i64) -> BotResult<bool>{
        let sign = self.select_sign(user_id).await?;
        let sign  = Sign{
            sign_time:* sign_time,
            ..sign.clone()
        };
        Sign::update_by_column(pool!(), &sign, "id").await?;
        Ok(true)
    }

    pub async fn select_sign(&self,user_id:&i64)-> BotResult<Sign>{
        let sign = Sign::select_sign(pool!(), user_id).await?;
        match sign {
            None => {
                Err(BotError::from("null"))
            }
            Some(data) => Ok(data),
        }
    }
}