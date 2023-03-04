use proc_qq::{event, MessageChainParseTrait, MessageContentTrait, MessageEvent, MessageSendToSourceTrait, MessageTargetTrait, Module, module};
use proc_qq::re_exports::ricq::structs::GroupMemberPermission;
use rbatis::dark_std::err;
use regex::RegexSet;
use crate::api::mc_status::get_minecraft_status;
use crate::{BotResult, CONTEXT};
use crate::database::table::McServer;
use crate::msg_util::{CanReply, MessageChain, text};
use crate::utils::Reg;

static ID: &'static str = "mc_server_status_get";
static NAME: &'static str = "mc服务器状态获取";

pub struct McServerStatusGetHelp {
    pub mod_name: String,
    pub help_text: Vec<String>,
}

impl Default for McServerStatusGetHelp {
    fn default() -> Self {
        McServerStatusGetHelp {
            mod_name: "mc".to_string(),
            help_text: vec![
                "mc_server_status_get Help",
                "----------------------------------------------------------------",
                "/list",
                "/list [简称]",
                "/mc add [简称] [url]",
                "/mc upname [简称] [name]",
                "/mc upurl [简称] [url]",
                "----------------------------------------------------------------",
                "/list      获取本群绑定服务器简称",
                "/list [简称]         获取本群绑定的服务器在线玩家名单",
                "/mc add [简称] [url]     在本群添加一个服务器到数据库",
                "/mc upname [简称] [name]       修改本群绑定的服务器的简称  需要管理员或群主",
                "/mc upurl [简称] [url]       修改本群绑定的服务器的简称 需要管理员或群主",
            ].iter().map(|str| str.to_string()).collect::<Vec<_>>(),
        }
    }
}

pub(crate) fn module() -> Module {
    module!(
        ID,
        NAME,
        mc_server_status_get,
    )
}

#[event]
async fn mc_server_status_get(event: &MessageEvent) -> anyhow::Result<bool> {
    let content = event.message_content();
    if event.is_group_message() {
        let (mc_bool, mc_array) = Reg::ex_msg(content.as_str(), &["/mc[\\s](.*)"], None);
        let (list_bool, list_array) = Reg::ex_msg(content.as_str(), &["/list[\\s]+(.*)"], None);
        let group_id = event.as_group_message().unwrap().inner.group_code;
        if Reg::ex(content.as_str(), &["/list $"], None) {
            return match CONTEXT.mc_server.select_server_all_by_group_id(group_id).await {
                None => {
                    event.at_text("本群一个服务器都没绑定喵!").await?;
                    Ok(true)
                }
                Some(vec) => {
                    if vec.len() == 0 {
                        event.at_text("本群一个服务器都没绑定喵!").await?;
                        return Ok(true);
                    }
                    let mut chain = MessageChain::new();
                    chain.text("list: \n");
                    for mc_server in vec {
                        chain.text(format!("    {} \n", mc_server.name));
                    }
                    event.send_message_to_source(chain.ok()).await?;
                    Ok(true)
                }
            };
        }
        if list_bool {
            return match CONTEXT.mc_server.select_server_by_name_group_id(list_array[1].as_str(), group_id).await {
                None => {
                    event.at_text("本群并没有这个服务器简称喵...").await?;
                    Ok(true)
                }
                Some(mc_server) => {
                    match get_minecraft_status(mc_server.url.as_str()).await {
                        Ok(status) => {
                            let mut chain = MessageChain::new();
                            chain.text(format!("{} Online: {}/{} Ping: {}\n", mc_server.name, status.players.online, status.players.max, status.ping));
                            let vec = status.players.sample
                                .iter()
                                .map(|sample| {
                                    sample.name.to_string()
                                }).collect::<Vec<_>>();
                            chain.text(format!("{:?}", vec)
                                .replace("\"", "")
                                .replace("[", "")
                                .replace("]", ""));
                            event.send_message_to_source(chain.ok()).await?;
                            Ok(true)
                        }
                        Err(err) => {
                            event.send_message_to_source(err.to_msg()).await?;
                            Ok(true)
                        }
                    }
                }
            };
        }
        if mc_bool {
            if mc_array[1].eq("add") {
                return match CONTEXT.mc_server.new(mc_array[2].as_str(), mc_array[3].as_str(), group_id).await {
                    Ok(_) => {
                        event.at_text("服务器添加成功喵!").await?;
                        Ok(true)
                    }
                    Err(err) => {
                        event.send_message_to_source(err.to_msg()).await?;
                        Ok(true)
                    }
                };
            }
            let map = event.client().get_group_admin_list(group_id).await?;
            let mut user_array = map.iter().filter_map(|(user_id, group_permission)| {
                match group_permission {
                    GroupMemberPermission::Owner => Some(user_id.to_string()),
                    GroupMemberPermission::Administrator => Some(user_id.to_string()),
                    GroupMemberPermission::Member => {
                        None
                    }
                }
            }).collect::<Vec<_>>();
            CONTEXT.config.super_admin.iter().for_each(|user_id| {
                user_array.push(user_id.clone());
            });
            if mc_array[1].eq("upname") {
                return if RegexSet::new(&user_array).unwrap().is_match(event.from_uin().to_string().as_str()) {
                    match CONTEXT.mc_server.update_name_by_name_group_id(mc_array[2].as_str(), group_id, mc_array[3].as_str()).await {
                        Ok(_) => {
                            event.at_text("修改服务器简称成功喵!").await?;
                            Ok(true)
                        }
                        Err(err) => {
                            event.send_message_to_source(err.to_msg()).await?;
                            Ok(true)
                        }
                    }
                } else {
                    event.at_text("你没有权限修改服务器简称喵...").await?;
                    return Ok(true);
                };
            }

            if mc_array[1].eq("upurl") {
                return if RegexSet::new(&user_array).unwrap().is_match(event.from_uin().to_string().as_str()) {
                    match CONTEXT.mc_server.update_url_by_name_group_id(mc_array[2].as_str(), group_id, mc_array[3].as_str()).await {
                        Ok(_) => {
                            event.at_text("修改服务器url成功喵!").await?;
                            Ok(true)
                        }
                        Err(err) => {
                            event.send_message_to_source(err.to_msg()).await?;
                            Ok(true)
                        }
                    }
                } else {
                    event.at_text("你没有权限修改服务器url喵...").await?;
                    return Ok(true);
                };
            }
        }
    }
    Ok(false)
}