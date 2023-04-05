use crate::api::bili_api::{BiliApiRoom, BiliApiUp};
use crate::database::table::BiliPush;
use crate::{BotError, BotResult, CONTEXT, pool};


pub struct BiliPushImpl {}

pub enum BiliPushType {
    Dynamic(bool),
    Video(bool),
    Live(bool),
}

impl BiliPushType {
    pub fn new(ty: &str, b: bool) -> Option<BiliPushType> {
        match ty {
            "直播" => {
                Some(BiliPushType::Live(b))
            },
            "动态" => {
                Some(BiliPushType::Dynamic(b))
            },
            "视频" => {
                Some(BiliPushType::Video(b))
            },
            _ => {
                None
            }
        }
    }
}

impl BiliPushImpl {
    pub async fn insert_up(&self, uid: i64, group_id: i64) -> BotResult<BiliPush> {
        let bili_push = BiliPush::select_bili_push_by_uid(pool!(), uid, group_id).await?;
        let vec = BiliPush::select_by_column(pool!(), "uid", uid).await?;
        if vec.is_empty() {
            match bili_push {
                None => {
                    let api = BiliApiUp::get_user_info(uid).await;
                    match api {
                        None => {
                            Err(BotError::from("获取api失败喵..."))
                        }
                        Some(up) => {
                            let bili_push = BiliPush {
                                room_id: up.room_id,
                                uid,
                                uname: up.uname.clone(),
                                group_id,
                                live_push: true,
                                video_push: true,
                                dynamic_push: true,
                                ..Default::default()
                            };
                            match BiliPush::insert(pool!(), &bili_push).await {
                                Ok(_) => {
                                    Ok(bili_push)
                                }
                                Err(_) => {
                                    Err(BotError::from("添加数据库出错喵..."))
                                }
                            }
                        }
                    }
                }
                Some(_) => {
                    Err(BotError::from("本群已添加该up主喵..."))
                }
            }
        } else {
            let bili_push = BiliPush {
                room_id: vec[0].room_id,
                uid: vec[0].uid,
                uname: vec[0].uname.clone(),
                group_id,
                live_push: true,
                video_push: true,
                dynamic_push: true,
                ..Default::default()
            };
            match BiliPush::insert(pool!(), &bili_push).await {
                Ok(_) => {
                    Ok(bili_push)
                }
                Err(_) => {
                    Err(BotError::from("添加数据库出错喵..."))
                }
            }
        }
    }
    pub async fn insert_room(&self, room_id: i64, group_id: i64) -> BotResult<BiliPush> {
        let bili_push = BiliPush::select_bili_push_by_room_id(pool!(), room_id, group_id).await?;
        let vec = BiliPush::select_by_column(pool!(), "room_id", room_id).await?;
        if vec.is_empty() {
            let api = BiliApiRoom::get_live_info(room_id).await;
            match bili_push {
                None => {
                    match api {
                        None => {
                            Err(BotError::from("获取api数据出错喵...,请稍后重试喵..."))
                        }
                        Some(room) => {
                            match BiliApiUp::get_user_info(room.uid).await {
                                None => {
                                    Err(BotError::from("获取api数据出错喵...,请稍后重试喵..."))
                                }
                                Some(up) => {
                                    let bili_push = BiliPush {
                                        room_id,
                                        uid: up.uid,
                                        uname: up.uname.clone(),
                                        group_id,
                                        live_push: true,
                                        video_push: true,
                                        dynamic_push: true,
                                        ..Default::default()
                                    };
                                    match BiliPush::insert(pool!(), &bili_push).await {
                                        Ok(_) => {
                                            Ok(bili_push)
                                        }
                                        Err(_) => {
                                            Err(BotError::from("添加数据库出错喵..."))
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                Some(_) => {
                    Err(BotError::from("本群已添加该up主喵..."))
                }
            }
        } else {
            let bili_push = BiliPush {
                room_id,
                uid: vec[0].uid,
                uname: vec[0].uname.clone(),
                group_id,
                live_push: true,
                video_push: true,
                dynamic_push: true,
                ..Default::default()
            };
            match BiliPush::insert(pool!(), &bili_push).await {
                Ok(_) => {
                    Ok(bili_push)
                }
                Err(_) => {
                    Err(BotError::from("添加数据库出错喵..."))
                }
            }
        }
    }


    pub async fn select_push_switch(&self, uid: i64, group_id: i64) -> BotResult<BiliPush> {
        let bili_push = BiliPush::select_bili_push_by_uid(pool!(), uid, group_id).await?;
        match bili_push {
            None => Err(BotError::from("该up主本群还没有关注喵..., 请输入 /bili addup/addroom {uid}/{room_id} 关注喵!")),
            Some(data) => {
                return Ok(data);
            }
        }
    }

    pub async fn select_all(&self) -> BotResult<Vec<BiliPush>> {
        let bili_push = BiliPush::select_all(pool!()).await?;
        Ok(bili_push)
    }

    pub async fn select_all_by_group_id(&self, group_id: i64) -> BotResult<Vec<BiliPush>> {
        let bili_push = BiliPush::select_by_column(pool!(), "group_id", group_id).await?;
        Ok(bili_push)
    }
    pub async fn select_all_by_uid(&self, uid: i64) -> BotResult<Vec<BiliPush>> {
        let bili_push = BiliPush::select_by_column(pool!(), "uid", uid).await?;
        Ok(bili_push)
    }

    pub async fn unfollow_up(&self, uid: i64, group_id: i64) -> BotResult<BiliPush> {
        let bili_push = BiliPush::select_bili_push_by_uid(pool!(), uid, group_id).await?;
        match bili_push {
            None => Err(BotError::from("本群并没有关注该up主喵...")),
            Some(bili_push) => {
                match BiliPush::delect_bili_push_by_uid_group_id(pool!(), uid, bili_push.group_id).await {
                    Ok(_) => {
                        Ok(bili_push)
                    }
                    Err(err) => {
                        Err(BotError::from(err))
                    }
                }
            }
        }
    }
    pub async fn update_uname(&self, uid: i64, group_id: i64) -> BotResult<BiliPush> {
        let bili_push = BiliPush::select_bili_push_by_uid(pool!(), uid, group_id).await?;
        match bili_push {
            None => Err(BotError::from("本群并没有关注该up主喵...")),
            Some(mut bili_push) => {
                match BiliApiUp::get_user_info(uid).await {
                    None => {
                        Err(BotError::from("获取UP主数据失败喵...\n 请稍后重试..."))
                    }
                    Some(up) => {
                        bili_push.uname = up.uname;
                        match BiliPush::update_by_column(pool!(), &bili_push, "id").await {
                            Ok(_) => {
                                Ok(bili_push)
                            }
                            Err(err) => {
                                Err(BotError::from(err))
                            }
                        }
                    }
                }
            }
        }
    }

    pub async fn update_push(&self, uid: i64, group_id: i64, push_type: BiliPushType) -> BotResult<BiliPush> {
        let bili_push = BiliPush::select_bili_push_by_uid(pool!(), uid, group_id).await?;
        match bili_push {
            None => Err(BotError::from("该up主本群还没有关注喵..., 请输入 /bili addup/addroom {uid}/{room_id} 关注喵!")),
            Some(data) => {
                let bili_push = match push_type {
                    BiliPushType::Dynamic(dynamic_push) => {
                        BiliPush {
                            dynamic_push,
                            ..data.clone()
                        }
                    }
                    BiliPushType::Video(video_push) => {
                        BiliPush {
                            video_push,
                            ..data.clone()
                        }
                    }
                    BiliPushType::Live(live_push) => {
                        BiliPush {
                            live_push,
                            ..data.clone()
                        }
                    }
                };
                match BiliPush::update_by_column(pool!(), &bili_push, "uid").await {
                    Ok(_) => {
                        Ok(data)
                    }
                    Err(err) => {
                        Err(BotError::from(err))
                    }
                }
            }
        }
    }

    pub async fn update_live_status(&self, uid: i64, live_status: i32) -> BotResult<()> {
        let bili_push = self.select_all_by_uid(uid).await;
        match bili_push {
            Ok(mut bili_push) => {
                bili_push.iter_mut().for_each(|bili_push| {
                    bili_push.live_status = live_status;
                });
                match BiliPush::update_by_column_batch(pool!(), &bili_push, "id").await {
                    Ok(_) => {
                        Ok(())
                    }
                    Err(err) => {
                        Err(BotError::from(err))
                    }
                }
            }
            Err(err) => {
                Err(err)
            }
        }
    }

    pub async fn update_video_time(&self, uid: i64, video_time: i64) -> BotResult<()> {
        let bili_push = self.select_all_by_uid(uid).await;
        match bili_push {
            Ok(mut bili_push) => {
                bili_push.iter_mut().for_each(|bili_push| {
                    bili_push.latest_video_time = video_time;
                });
                match BiliPush::update_by_column_batch(pool!(), &bili_push, "id").await {
                    Ok(_) => {
                        Ok(())
                    }
                    Err(err) => {
                        Err(BotError::from(err))
                    }
                }
            }
            Err(err) => {
                Err(err)
            }
        }
    }

    pub async fn update_dynamic_time(&self, uid: i64, dynamic_time: i64) -> BotResult<()> {
        let bili_push = self.select_all_by_uid(uid).await;
        match bili_push {
            Ok(mut bili_push) => {
                bili_push.iter_mut().for_each(|bili_push| {
                    bili_push.latest_dynamic_time = dynamic_time;
                });
                match BiliPush::update_by_column_batch(pool!(), &bili_push, "id").await {
                    Ok(_) => {
                        Ok(())
                    }
                    Err(err) => {
                        Err(BotError::from(err))
                    }
                }
            }
            Err(err) => {
                Err(err)
            }
        }
    }
}

fn bili_push_err() -> BotResult<()> {
    Err(BotError::from("该up主本群还没有关注喵..., 请输入 /bili addup/addroom {uid}/{room_id} 关注喵!"))
}