use chrono::{Timelike};
use proc_qq::{event, GroupMessageEvent, MessageChainParseTrait, MessageContentTrait, MessageEvent, MessageSendToSourceTrait, Module, module};
use proc_qq::re_exports::ricq::structs::GroupMemberPermission;
use regex::RegexSet;
use crate::api::mc_status::{get_minecraft_status_bedrock, get_minecraft_status_java};
use crate::{CONTEXT};
use crate::database::implement::mc_server_impl::McServerType;
use crate::msg_util::{CanReply, MessageChain, OrderPermissionTrait};


pub(crate) fn module() -> Module {
    module!(
        "mc_server_status_get",
        "mc服务器状态获取",
        mc_server_status_list,
        mc_server_status,
    )
}

#[event(bot_command = "/list {name}")]
async fn mc_server_status_list(event: &GroupMessageEvent, name: Option<String>) -> anyhow::Result<bool> {
    if let Some(name) = name {
        return match CONTEXT.mc_server.select_server_by_name_group_id(name.to_uppercase().as_str(), event.inner.group_code).await {
            None => {
                event.at_text("本群并没有这个服务器简称喵...").await?;
                Ok(true)
            }
            Some(mc_server) => {
                match McServerType::new(mc_server.server_type.as_str()) {
                    Ok(server_type) => {
                        match server_type {
                            McServerType::JAVA => {
                                match get_minecraft_status_java(mc_server.url.as_str()).await {
                                    Ok(status) => {
                                        if status.online {
                                            let mut chain = MessageChain::new();
                                            chain.text(format!("{} Online: {}/{}\n", mc_server.name, status.players.online, status.players.max));
                                            let vec = status.players.list
                                                .iter()
                                                .map(|list| {
                                                    list.name_raw.to_owned()
                                                }).collect::<Vec<_>>();
                                            if vec.len() == 0 {
                                                chain.text("没有玩家在服务器喵...");
                                            } else {
                                                chain.text(format!("{:?}", vec)
                                                    .replace("\"", "")
                                                    .replace("[", "")
                                                    .replace("]", ""));
                                            }
                                            let cache_time = chrono::NaiveDateTime::from_timestamp_millis(status.expires_at).unwrap_or_default();
                                            let time = chrono::Utc::now().naive_utc();
                                            if cache_time.minute() > time.minute() {
                                                let duration = cache_time.time() - time.time();
                                                if duration.num_seconds() < 55 {
                                                    chain.text(format!("\n数据还剩{}秒刷新喵!", duration.num_seconds()));
                                                }
                                            }
                                            event.send_message_to_source(chain.build()).await?;
                                            Ok(true)
                                        } else {
                                            event.send_message_to_source("服务器当前不在线喵...".parse_message_chain()).await?;
                                            Ok(true)
                                        }
                                    }
                                    Err(err) => {
                                        event.send_message_to_source(err.to_msg()).await?;
                                        Ok(true)
                                    }
                                }
                            }
                            McServerType::Bedrock => {
                                match get_minecraft_status_bedrock(mc_server.url.as_str()).await {
                                    Ok(status) => {
                                        if status.online {
                                            let mut chain = MessageChain::new();
                                            chain.text(format!("{} Players: {}/{}\n", mc_server.name, status.players.online, status.players.max));
                                            chain.text("Bedrock版无法获取到玩家列表喵!");
                                            let cache_time = chrono::NaiveDateTime::from_timestamp_millis(status.expires_at).unwrap_or_default();
                                            let time = chrono::Utc::now().naive_utc();
                                            if cache_time.minute() > time.minute() {
                                                let duration = cache_time.time() - time.time();
                                                if duration.num_seconds() < 55 {
                                                    chain.text(format!("\n数据还剩{}秒刷新喵!", duration.num_seconds()));
                                                }
                                            }
                                            event.send_message_to_source(chain.build()).await?;
                                            Ok(true)
                                        } else {
                                            event.send_message_to_source("服务器当前不在线喵...".parse_message_chain()).await?;
                                            Ok(true)
                                        }
                                    }
                                    Err(err) => {
                                        event.send_message_to_source(err.to_msg()).await?;
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
        };
    } else {
        match CONTEXT.mc_server.select_server_all_by_group_id(event.inner.group_code).await {
            None => {
                event.send("本群没有绑定服务器喵...").await?;
                return Ok(true);
            }
            Some(v) => {
                if v.is_empty() {
                    event.send("本群没有绑定服务器喵...").await?;
                    return Ok(true);
                }
                let mut chain = MessageChain::new();
                chain.text("当前可用服务器列表:\n");
                for (i, server) in v.iter().enumerate() {
                    chain.text(&format!("{}. {}\n", i + 1, server.name));
                }
                chain.text("可用指令:  /list {name}");
                event.send_message_to_source(chain.build()).await?;
                return Ok(true);
            }
        }
    }
}

#[event(bot_command = "/mc {mc_type} {name} {new_data}")]
async fn mc_server_status(event: &GroupMessageEvent, mc_type: Option<String>, name: Option<String>, new_data: Option<String>) -> anyhow::Result<bool> {
    if let Some(mc_type) = mc_type {
        if event.is_admin().await {
            return match mc_type.as_str() {
                "add" => {
                    if let (Some(name), Some(new_data)) = (name, new_data) {
                        match CONTEXT.mc_server.new(name.to_uppercase().as_str(), new_data.as_str(), event.inner.group_code, Ok(McServerType::JAVA)).await {
                            Ok(_) => {
                                event.reply_text("服务器添加成功喵!").await?;
                                Ok(true)
                            }
                            Err(err) => {
                                event.send_message_to_source(err.to_msg()).await?;
                                Ok(true)
                            }
                        }
                    } else {
                        event.send("参数不够喵...,\n 指令: /mc add {name} {Address}").await?;
                        Ok(true)
                    }
                }
                "upname" => {
                    if let (Some(name), Some(new_data)) = (name, new_data) {
                        match CONTEXT.mc_server.update_name_by_name_group_id(name.to_uppercase().as_str(), event.inner.group_code, new_data.to_uppercase().as_str()).await {
                            Ok(_) => {
                                event.reply_text("修改服务器简称成功喵!").await?;
                                Ok(true)
                            }
                            Err(err) => {
                                event.send_message_to_source(err.to_msg()).await?;
                                Ok(true)
                            }
                        }
                    } else {
                        event.send("参数不够喵...,\n 指令: /mc upname {name} {new_name}").await?;
                        Ok(true)
                    }
                }
                "upurl" => {
                    if let (Some(name), Some(new_data)) = (name, new_data) {
                        match CONTEXT.mc_server.update_url_by_name_group_id(name.to_uppercase().as_str(), event.inner.group_code, new_data.as_str()).await {
                            Ok(_) => {
                                event.reply_text("修改服务器url成功喵!").await?;
                                Ok(true)
                            }
                            Err(err) => {
                                event.send_message_to_source(err.to_msg()).await?;
                                Ok(true)
                            }
                        }
                    } else {
                        event.send("参数不够喵...,\n 指令: /mc upname {name} {new_url}").await?;
                        Ok(true)
                    }
                }
                "uptype" => {
                    if let (Some(name), Some(new_data)) = (name, new_data) {
                        match CONTEXT.mc_server.update_server_type_by_name_group_id(name.to_uppercase().as_str(), event.inner.group_code, McServerType::new(new_data.to_uppercase().as_str())).await {
                            Ok(_) => {
                                event.reply_text("修改服务器type成功喵!").await?;
                                Ok(true)
                            }
                            Err(err) => {
                                event.send_message_to_source(err.to_msg()).await?;
                                Ok(true)
                            }
                        }
                    } else {
                        event.send("参数不够喵...,\n 指令: /mc uptype {name} {new_type} \nnew_type可用参数:[JE,BE]").await?;
                        Ok(true)
                    }
                }
                "d" => {
                    if let (Some(name), Some(new_data)) = (name, new_data) {
                        match CONTEXT.mc_server.delete_server_by_name_group_id(name.to_uppercase().as_str(), event.inner.group_code).await {
                            Ok(_) => {
                                event.reply_text("删除服务器成功喵!").await?;
                                Ok(true)
                            }
                            Err(err) => {
                                event.send_message_to_source(err.to_msg()).await?;
                                Ok(true)
                            }
                        }
                    } else {
                        event.send("参数不够喵...,\n 指令: /mc d {name}").await?;
                        Ok(true)
                    }
                }
                _ => {
                    event.send_message_to_source("没有这个子指令喵...".parse_message_chain()).await?;
                    Ok(true)
                }
            };
        } else {
            event.reply_text("你没有权限使用该指令喵...").await?;
            return Ok(true);
        }
    } else {
        event.send_message_to_source(
            MessageChain::new()
                .text("可用子指令:\n")
                .text(">    add\n")
                .text(">    upname\n")
                .text(">    upurl\n")
                .text(">    uptype\n")
                .text(">    d\n")
                .text("此命令仅管理员/群主可用")
                .build()
        ).await?;
        return Ok(true);
    }
}