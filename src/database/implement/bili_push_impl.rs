use std::process::id;
use rbatis::rbdc::db::ExecResult;
use rbatis::rbdc::Error;
use serde_json::Value;
use tracing::info;
use crate::api::bili_api::BiliApi;
use crate::database::table::BiliPush;
use crate::{BotError, BotResult, pool};


pub struct BiliPushImpl{}
pub enum BiliPushType{
    Dynamic(bool),
    Video(bool),
    Live(bool),
}
impl BiliPushImpl {

    pub async fn insert(&self, uid:&i64, group_id:&i64) -> BotResult<BiliApi> {
        let bili_push = BiliPush::select_group_id_by_uid(pool!(), uid ).await.ok().unwrap();
        let api = BiliApi::new(uid).await;
        if bili_push.is_none(){
            let bili_push = BiliPush{
                room_id: api.room_id,
                uid:*uid,
                uname:api.uname.clone(),
                group_id:format!("{:?}",vec![*group_id]),
                dynamic_push:1,
                video_push:1,
                live_push:1,
                ..Default::default()
            };
            let res = BiliPush::insert(pool!(), &bili_push).await;
            if res.is_ok(){
                return Ok(api);
            }
        }
        Err(BotError::from("数据添加失败喵..."))
    }

    pub async fn update_group_id(&self, uid:&i64, group_id:&i64) -> BotResult<BiliPush> {
        let mut bili_push = BiliPush::select_group_id_by_uid(pool!(), uid).await?.unwrap();
        let mut vec1 = vec![];
        let mut res_v = serde_json::from_str::<Value>(bili_push.group_id.clone().as_str()).unwrap();
        let vec = res_v.as_array().unwrap();
        for id in vec {
            if id.as_i64() == Some(*group_id) {
                return Err(BotError::from("该up主本群已关注喵..."));
            }else {
                vec1.push(*group_id);
            }
            vec1.push(id.as_i64().unwrap());
        }
       let bili_push1 =  BiliPush{
            group_id:format!("{:?}",vec1),
            ..bili_push
        };

        let res = BiliPush::update_by_column(pool!(), &bili_push1, "uid").await;
        match res {
            Ok(_) => {
                Ok(bili_push1)
            }
            Err(err) => {
                Err(BotError::from(err))
            }
        }
    }

    pub async fn select_up_is_null(&self,uid:&i64) -> Option<()> {
        let bili_push = BiliPush::select_group_id_by_uid(pool!(), uid).await.ok()?;
        match bili_push {
            None => None,
            Some(_) => Some(())
        }
    }

    pub async fn select_push_switch(&self,uid:&i64) -> BotResult<(BiliPush,(bool,bool,bool))>{
        let bili_push = BiliPush::select_group_id_by_uid(pool!(), uid).await.ok().unwrap();
        match bili_push {
            None =>  Err(BotError::from("该up主本群还没有关注喵..., 请输入 /关注 <uid> 关注喵!")),
            Some(data) => {
                return Ok((data.clone(),( self.int_to_bool(&data.dynamic_push),
                           self.int_to_bool(&data.video_push),
                          self.int_to_bool(&data.live_push),)));
            }
        }

    }

    pub async fn select_all(&self) -> BotResult<Vec<BiliPush>> {
        let res = BiliPush::select_all(pool!()).await;
        match res {
            Ok(bili_push) => {
                Ok(bili_push)
            }
            Err(err) => Err(BotError::from(err))
        }
    }

    pub async fn select_list(&self,group_id:&i64) -> BotResult<Vec<(i64,String)>> {
        let res = BiliPush::select_all(pool!()).await;
        match res {
            Ok(bili_push) => {
                let mut vec1 = vec![];
                for bili_push in bili_push {
                    let mut res_v = serde_json::from_str::<Value>(bili_push.group_id.clone().as_str()).unwrap();
                    let vec = res_v.as_array().unwrap();
                    for id in vec {
                        if id.as_i64() == Some(*group_id) {
                            vec1.push((bili_push.uid,bili_push.uname.clone()));
                        }
                    }
                }
                Ok(vec1)
            }
            Err(err) => Err(BotError::from(err))
        }
    }

    pub async fn unfollow_up(&self,uid:&i64,group_id:&i64) -> BotResult<()> {
        let bili_push = BiliPush::select_group_id_by_uid(pool!(), uid).await.ok().unwrap();
        match bili_push {
            None => Err(BotError::from("该up主本群没有关注喵...")),
            Some(bili_push) => {
                let mut vec1 = vec![];
                let mut res_v = serde_json::from_str::<Value>(bili_push.group_id.clone().as_str()).unwrap();
                let vec = res_v.as_array().unwrap();
                for id in vec {
                    if id.as_i64() == Some(*group_id) {
                        continue;
                    }
                    vec1.push(id.as_i64().unwrap());
                }
                let bili_push1 =  BiliPush{
                    group_id:format!("{:?}",vec1),
                    ..bili_push
                };
                let res = BiliPush::update_by_column(pool!(), &bili_push1, "uid").await;
                match res {
                    Ok(_) => {
                        Ok(())
                    }
                    Err(err) => {
                        Err(BotError::from(err))
                    }
                }
            }
        }

    }
    pub async fn update_push(&self,uid:&i64,push_type:&BiliPushType) -> BotResult<()>{
        let bili_push = BiliPush::select_group_id_by_uid(pool!(), uid).await.ok().unwrap();
        match bili_push {
            None =>  Err(BotError::from("该up主本群还没有关注喵..., 请输入 /关注 <uid> 关注喵!")),
            Some(data) => {
               let bili_push =  match push_type {
                    BiliPushType::Dynamic(bool) => {
                        let dynamic_push = self.bool_to_int(bool);
                        BiliPush{
                            dynamic_push,
                            ..data
                        }
                    }
                    BiliPushType::Video(bool) => {
                        let video_push = self.bool_to_int(bool);
                        BiliPush{
                            video_push,
                            ..data
                        }
                    }
                    BiliPushType::Live(bool) => {
                        let live_push = self.bool_to_int(bool);
                        BiliPush{
                            live_push,
                            ..data
                        }
                    }
                };
                let res = BiliPush::update_by_column(pool!(), &bili_push, "uid").await;
                match res {
                    Ok(_) => {
                        Ok(())
                    }
                    Err(err) => {
                        Err(BotError::from(err))
                    }
                }
            }
        }
    }
    pub async fn update_live_status(&self,uid:&i64,live_status:&i32) ->BotResult<()>{
        let bili_push = BiliPush::select_group_id_by_uid(pool!(), uid).await.ok().unwrap();
        match bili_push {
            None => Err(BotError::from("该up主本群还没有关注喵..., 请输入 /关注 <uid> 关注喵!")),
            Some(data) => {
                let bili_push = BiliPush{
                    live_status:*live_status,
                    ..data
                };
                let res = BiliPush::update_by_column(pool!(), &bili_push, "uid").await;
                match res {
                    Ok(_) => {
                        Ok(())
                    }
                    Err(err) => {
                        Err(BotError::from(err))
                    }
                }
            }
        }
    }

    pub async fn update_video_time(&self,uid:&i64,video_time:&i64) ->BotResult<()>{
        let bili_push = BiliPush::select_group_id_by_uid(pool!(), uid).await.ok().unwrap();
        match bili_push {
            None => Err(BotError::from("该up主本群还没有关注喵..., 请输入 /关注 <uid> 关注喵!")),
            Some(data) => {
                let bili_push = BiliPush{
                    latest_video_time:*video_time,
                    ..data
                };
                let res = BiliPush::update_by_column(pool!(), &bili_push, "uid").await;
                match res {
                    Ok(_) => {
                        Ok(())
                    }
                    Err(err) => {
                        Err(BotError::from(err))
                    }
                }
            }
        }
    }

    pub async fn update_dynamic_time(&self,uid:&i64,dynamic_time:&i64) ->BotResult<()>{
        let bili_push = BiliPush::select_group_id_by_uid(pool!(), uid).await.ok().unwrap();
        match bili_push {
            None => Err(BotError::from("该up主本群还没有关注喵..., 请输入 /关注 <uid> 关注喵!")),
            Some(data) => {
                let bili_push = BiliPush{
                    latest_dynamic_time:*dynamic_time,
                    ..data
                };
                let res = BiliPush::update_by_column(pool!(), &bili_push, "uid").await;
                match res {
                    Ok(_) => {
                        Ok(())
                    }
                    Err(err) => {
                        Err(BotError::from(err))
                    }
                }
            }
        }
    }

    fn int_to_bool(&self,int:&i8) -> bool {
        match int {
            1 => true ,
            _ => false
        }
    }
    fn bool_to_int(&self,bool:&bool) -> i8 {
        match *bool {
            true => 1,
            _ => 0
        }
    }

}