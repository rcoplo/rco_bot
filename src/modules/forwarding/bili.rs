use std::collections::{HashMap, HashSet};
use std::future::Future;
use std::iter;
use std::pin::Pin;
use std::sync::{Arc};
use proc_qq::{Client, event, GroupMessageEvent, MessageSendToSourceTrait, Module, module};
use proc_qq::re_exports::{anyhow, tracing};
use proc_qq::re_exports::ricq::RQResult;


use crate::{BotResult, CONTEXT};
use crate::api::bili_api::{BiliApiUp};
use crate::database::implement::bili_push_impl::{BiliPushType};
use crate::database::table::BiliPush;

use crate::msg_util::{CanReply, MessageChain};
use crate::scheduler::{ScheduledJob};
use crate::utils::http_util::http_get_image;


pub fn module() -> Module {
    module!(
        "bili_push",
        "bili推送",
        bili_push_all,
    )
}


pub struct BiliPushTask;


impl ScheduledJob for BiliPushTask {
    fn cron(&self) -> &'static str {
        "0 0/3 * * * ?"
    }

    fn call(&self, client: Arc<Client>) -> Pin<Box<dyn Future<Output=()> + Send>> {
        Box::pin(async move {
            match CONTEXT.bili_push.select_all().await {
                Ok(b) => {
                    let mut vec_live_push = HashMap::new();
                    let mut live_push_data = vec![];
                    b.iter().for_each(|b| {
                        if b.live_push {
                            vec_live_push.insert(b.uid, b.clone());
                        }
                    });
                    tracing::debug!("{:?}",&vec_live_push);
                    for (uid, bili_push) in vec_live_push {
                        match BiliApiUp::get_user_info(uid).await {
                            None => {
                                tracing::info!("查询: {} ({}) 失败喵...",bili_push.uname,bili_push.uid);
                            }
                            Some(up) => {
                                live_push_data.push((bili_push, up));
                            }
                        }
                    }
                    tracing::debug!("{:?}",&live_push_data);
                    let mut chain = MessageChain::new();
                    for (b, up) in live_push_data.iter() {
                        if up.live_status != 0 && b.live_status == 0 {
                            tracing::info!("构建开播信息");
                            chain.text(&format!("UP主 {} ({})开播啦喵!\n", up.uname, up.uid));
                            chain.text(&format!("Title: {} \n", up.title));
                            chain.text(&format!("https://live.bilibili.com/{}\n", up.room_id));
                            match http_get_image(up.user_cover.as_str()).await {
                                Ok(by) => {
                                    let image = client.rq_client.upload_group_image(b.group_id, by).await;
                                    chain.image_bytes_task(image).await;
                                }
                                Err(err) => {
                                    chain.text(err.to_string());
                                }
                            }
                            CONTEXT.bili_push.update_live_status(up.uid, up.live_status).await;
                        } else {
                            if up.live_status == 0 && b.live_status != 0 {
                                tracing::info!("构建下播信息");
                                chain.text(format!("UP主 {}({})下播了喵...", b.uname, b.uid));
                            }
                            CONTEXT.bili_push.update_live_status(up.uid, up.live_status).await;
                        }
                        if !chain.is_empty() {
                            match CONTEXT.bili_push.select_all_by_uid(up.uid).await {
                                Ok(b) => {
                                    for group_id in b.iter()
                                        .filter_map(|b| {
                                            if b.live_push {
                                                Some(b.group_id)
                                            } else {
                                                None
                                            }
                                        }).collect::<Vec<_>>() {
                                        client.rq_client.send_group_message(group_id, chain.build()).await.unwrap();
                                    }
                                    chain.clear();
                                }
                                Err(_) => {}
                            }
                        }
                    }
                }
                Err(_) => {}
            }

            tracing::info!("结束推送");
        })
    }
}

#[event(bot_command = "/bili {command}")]
async fn bili_push_all(event: &GroupMessageEvent, command: Vec<String>) -> anyhow::Result<bool> {
    tracing::info!("{:?}",&command);
    if command[0].is_empty() {
        event.send_message_to_source(
            MessageChain::new()
                .text("可用 {command}:\n")
                .text(">    addroom\n")
                .text(">    addup\n")
                .text(">    live\n")
                .text(">    dy\n")
                .text(">    video\n")
                .text(">    list\n")
                .text(">    d\n")
                .text(">    upuname\n")
                .text("指令/bili {command} {uid/room_id} {on/off}")
                .build()
        ).await?;
        return Ok(true);
    }
    let subcommand = command[0].as_str();
    let command_1 = command.get(1);
    let command_sw = command.get(2);
    let group_id = event.inner.group_code;
    match subcommand {
        "addroom" => {
            if let Some(command_1) = command_1 {
                match command_1.parse::<i64>() {
                    Ok(room_id) => {
                        match CONTEXT.bili_push.insert_room(room_id, group_id).await {
                            Ok(b) => {
                                event.at_text(&format!("关注主播 {}({}) 成功喵!", b.uname, b.uid)).await?;
                                return Ok(true);
                            }
                            Err(err) => {
                                event.send_message_to_source(err.to_msg()).await?;
                                return Ok(true);
                            }
                        }
                    }
                    Err(_) => {
                        event.at_text("你输入的不是整数喵...").await?;
                        return Ok(true);
                    }
                }
            } else {
                event.at_text("缺少房间号喵... \n指令: /bili addroom 房间号").await?;
                return Ok(true);
            }
        },
        "addup" => {
            if let Some(command_1) = command_1 {
                match command_1.parse::<i64>() {
                    Ok(uid) => {
                        match CONTEXT.bili_push.insert_up(uid, group_id).await {
                            Ok(b) => {
                                event.at_text(&format!("关注up主 {}({}) 成功喵!", b.uname, b.uid)).await?;
                                return Ok(true);
                            }
                            Err(err) => {
                                event.send_message_to_source(err.to_msg()).await?;
                                return Ok(true);
                            }
                        }
                    }
                    Err(_) => {
                        event.at_text("你输入的不是整数喵...").await?;
                        return Ok(true);
                    }
                }
            } else {
                event.at_text("缺少用户id喵... \n指令: /bili addup 用户id").await?;
                return Ok(true);
            }
        },
        "live" => {
            if let Some(command_1) = command_1 {
                push_sw(command_1, command_sw, &event, "直播").await?;
                return Ok(true);
            } else {
                event.at_text("缺少用户id喵... \n指令: /bili live 用户id on/off").await?;
                return Ok(true);
            }
        },
        "dy" => {
            if let Some(command_1) = command_1 {
                push_sw(command_1, command_sw, &event, "动态").await?;
                return Ok(true);
            } else {
                event.at_text("缺少用户id喵... \n指令: /bili dy 用户id on/off").await?;
                return Ok(true);
            }
        },
        "video" => {
            if let Some(command_1) = command_1 {
                push_sw(command_1, command_sw, &event, "视频").await?;
                return Ok(true);
            } else {
                event.at_text("缺少用户id喵... \n指令: /bili video 用户id on/off").await?;
                return Ok(true);
            }
        },
        "list" => {
            if let Some(command_1) = command_1 {
                match command_1.parse::<i64>() {
                    Ok(uid) => {
                        match CONTEXT.bili_push.select_push_switch(uid, group_id).await {
                            Ok(up) => {
                                event.send_message_to_source(
                                    MessageChain::new()
                                        .text(format!("UP主 {}({})的推送开关:\n", up.uname, up.uid))
                                        .text(format!("1.live -> {}\n", if up.live_push { "on" } else { "off" }))
                                        .text(format!("2.dy -> {}\n", if up.dynamic_push { "on" } else { "off" }))
                                        .text(format!("3.video -> {}\n", if up.video_push { "on" } else { "off" }))
                                        .text("输入/bili {type} {uid} on/off  打开或关闭指定推送")
                                        .build()
                                ).await?;
                                return Ok(true);
                            }
                            Err(err) => {
                                event.send_message_to_source(err.to_msg()).await?;
                                return Ok(true);
                            }
                        }
                    }
                    Err(_) => {
                        event.at_text("你输入的不是整数喵...").await?;
                        return Ok(true);
                    }
                }
            } else {
                match CONTEXT.bili_push.select_all_by_group_id(group_id).await {
                    Ok(v) => {
                        if v.is_empty() {
                            event.at_text("这个群没有订阅任何人喵...").await?;
                            return Ok(true);
                        }
                        let mut chain = MessageChain::new();
                        chain.text("本群关注的UP主列表:");
                        for x in v {
                            chain.text(format!("\n>   {}({})", x.uname, x.uid));
                        }
                        event.send_message_to_source(chain.build()).await?;
                        return Ok(true);
                    }
                    Err(err) => {
                        event.at_text("查询错误喵...请联系主人喵...").await?;
                        return Ok(true);
                    }
                }
                return Ok(true);
            }
        },
        "d" => {
            if let Some(command_1) = command_1 {
                match command_1.parse::<i64>() {
                    Ok(uid) => {
                        match CONTEXT.bili_push.unfollow_up(uid, group_id).await {
                            Ok(up) => {
                                event.at_text(&format!("取消up主 {}({}) 成功喵!", up.uname, up.uid)).await?;
                                return Ok(true);
                            }
                            Err(err) => {
                                event.send_message_to_source(err.to_msg()).await?;
                                return Ok(true);
                            }
                        }
                    }
                    Err(_) => {
                        event.at_text("你输入的不是整数喵...").await?;
                        return Ok(true);
                    }
                }
            } else {
                event.at_text("缺少用户id喵... \n指令: /bili d 用户id").await?;
                return Ok(true);
            }
        },
        "upuname" => {
            if let Some(command_1) = command_1 {
                match command_1.parse::<i64>() {
                    Ok(uid) => {
                        match CONTEXT.bili_push.update_uname(uid, group_id).await {
                            Ok(up) => {
                                event.at_text(&format!("更新up主 {}({})数据成功喵!", up.uname, up.uid)).await?;
                                return Ok(true);
                            }
                            Err(err) => {
                                event.send_message_to_source(err.to_msg()).await?;
                                return Ok(true);
                            }
                        }
                    }
                    Err(_) => {
                        event.at_text("你输入的不是整数喵...").await?;
                        return Ok(true);
                    }
                }
            } else {
                event.at_text("缺少用户id喵... \n指令: /bili upuname 用户id").await?;
                return Ok(true);
            }
        },
        _ => {
            event.at_text("没有这个子指令喵...,\n输入 /bili 查看可用参数").await?;
            return Ok(true);
        }
    }
}

async fn push_sw(command_1: &str, command_sw: Option<&String>, event: &GroupMessageEvent, ty: &str) -> anyhow::Result<bool> {
    return match command_1.parse::<i64>() {
        Ok(uid) => {
            match CONTEXT.bili_push.select_push_switch(uid, event.inner.group_code).await {
                Ok(b) => {
                    match command_sw {
                        None => {
                            event.at_text("使用 type:[live,dy,video] 参数格式:\n /bili {type} {uid} on/off \nlive:直播 \ndy: 动态 \nvideo: 视频").await?;
                            Ok(true)
                        }
                        Some(command_sw) => {
                            match command_sw.as_str() {
                                "on" => {
                                    match BiliPushType::new(ty, true) {
                                        None => {
                                            event.at_text("没有这种类型喵...").await?;
                                            Ok(true)
                                        }
                                        Some(t) => {
                                            match CONTEXT.bili_push.update_push(uid, event.inner.group_code, t).await {
                                                Ok(b) => {
                                                    event.at_text(&format!("开启up主 {}({}) {ty}推送成功喵!", b.uname, b.uid)).await?;
                                                    Ok(true)
                                                }
                                                Err(err) => {
                                                    event.send_message_to_source(err.to_msg()).await?;
                                                    Ok(true)
                                                }
                                            }
                                        }
                                    }
                                }
                                "off" => {
                                    match BiliPushType::new(ty, false) {
                                        None => {
                                            event.at_text("没有这种类型喵...").await?;
                                            Ok(true)
                                        }
                                        Some(t) => {
                                            match CONTEXT.bili_push.update_push(uid, event.inner.group_code, t).await {
                                                Ok(b) => {
                                                    event.at_text(&format!("关闭up主 {}({}) {ty}推送成功喵!", b.uname, b.uid)).await?;
                                                    Ok(true)
                                                }
                                                Err(err) => {
                                                    event.send_message_to_source(err.to_msg()).await?;
                                                    Ok(true)
                                                }
                                            }
                                        }
                                    }
                                }
                                _ => {
                                    event.at_text("没有这个参数喵...").await?;
                                    Ok(true)
                                }
                            }
                        }
                    }
                }
                Err(err) => {
                    event.send_message_to_source(err.to_msg()).await?;
                    Ok(true)
                }
            }
        }
        Err(_) => {
            event.at_text("你输入的不是整数喵...").await?;
            Ok(true)
        }
    }
}
