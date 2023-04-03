use proc_qq::{event, event_fn, MessageChainParseTrait, MessageEvent, MessageSendToSourceTrait, Module, module, ModuleEventHandler, ModuleEventProcess};
use proc_qq::re_exports::tracing;

use crate::{BotResult, CONTEXT};

use crate::msg_util::{CanReply, MessageChain};
use crate::utils::http_util::http_get_image;
use crate::utils::image::ett::{ETT_CLIENT};
use crate::utils::image::ett::ett_user_info_image::EttUserInfoImage;

use crate::utils::ToJson;

pub(crate) fn module() -> Module {
    module!(
        "ett_user",
        "Ett用户获取",
        ett_module,
    )
}

#[event(bot_command = "/ett {et_type} {name}")]
async fn ett_module(event: &MessageEvent, et_type: Option<String>, name: Option<String>) -> anyhow::Result<bool> {
    if let Some(et_type) = et_type {
        return match et_type.as_str() {
            "build" => {
                if let Some(name) = name {
                    match CONTEXT.ett.ett_build_by_name_qq(name.as_str(), event.from_uin()).await {
                        Ok(_) => {
                            event.send_message_to_source("绑定成功喵!".parse_message_chain()).await?;
                            Ok(true)
                        }
                        Err(err) => {
                            event.send_message_to_source(err.to_msg()).await?;
                            Ok(true)
                        }
                    }
                } else {
                    event.at_text("用户名不能为空喵...").await?;
                    Ok(true)
                }
            }
            "untie" => {
                let ett_user = CONTEXT.ett.ett_select_by_name_qq(event.from_uin()).await;
                match ett_user {
                    Ok(ett_user) => {
                        match CONTEXT.ett.ett_untie_by_qq(event.from_uin()).await {
                            Ok(str) => {
                                event.send_message_to_source(str.parse_message_chain()).await?;
                                Ok(true)
                            }
                            Err(err) => {
                                event.send_message_to_source(err.to_msg()).await?;
                                Ok(true)
                            }
                        }
                    }
                    Err(err) => {
                        event.send_message_to_source(err.to_msg()).await?;
                        Ok(true)
                    }
                }
            }
            "info" => {
                if let Some(name) = name {
                    match ETT_CLIENT.as_ref() {
                        Err(err) => {
                            event.send_message_to_source(format!("ett client 错误 ,请联系bot主人 Error: {}", err).parse_message_chain()).await?;
                            Ok(true)
                        }
                        Ok(session) => {
                            match session.user_details(name.as_str()) {
                                Ok(data) => {
                                    tracing::debug!("{:?}",&data);
                                    let image = http_get_image(format!("https://etternaonline.com/avatars/{}", data.avatar_url).as_str()).await?;
                                    let res = EttUserInfoImage::new(data, None, chrono::Local::now().naive_local()).build(&image);
                                    match res {
                                        Ok(ok) => {
                                            let chain = MessageChain::new().image_bytes(ok, &event).await.build();
                                            event.send_message_to_source(chain).await?;
                                            Ok(true)
                                        }
                                        Err(err) => {
                                            event.send_message_to_source(err.to_string().parse_message_chain()).await?;
                                            Ok(true)
                                        }
                                    }
                                }
                                Err(err) => {
                                    event.send_message_to_source(err.to_string().parse_message_chain()).await?;
                                    Ok(true)
                                }
                            }
                        }
                    }
                } else {
                    let ett_user = CONTEXT.ett.ett_select_by_name_qq(event.from_uin()).await;
                    match ett_user {
                        Ok(ett_user) => {
                            match ETT_CLIENT.as_ref() {
                                Err(err) => {
                                    event.send_message_to_source(format!("ett client 错误 ,请联系bot主人 Error: {}", err).parse_message_chain()).await?;
                                    Ok(true)
                                }
                                Ok(session) => {
                                    match session.user_details(ett_user.user_name.as_str()) {
                                        Ok(data) => {
                                            tracing::debug!("{:?}",&data);
                                            let image = http_get_image(format!("https://etternaonline.com/avatars/{}", data.avatar_url).as_str()).await?;

                                            CONTEXT.ett.ett_update_rating_time_by_qq(event.from_uin(), String::struct_to_json(&data.rating), chrono::Local::now().naive_utc()).await?;

                                            let res = EttUserInfoImage::new(data, Some(ett_user.rating), ett_user.update_time).build(&image);
                                            match res {
                                                Ok(ok) => {
                                                    let chain = MessageChain::new().image_bytes(ok, &event).await.build();
                                                    event.send_message_to_source(chain).await.unwrap();
                                                    Ok(true)
                                                }
                                                Err(err) => {
                                                    event.send_message_to_source(err.to_string().parse_message_chain()).await?;
                                                    Ok(true)
                                                }
                                            }
                                        }
                                        Err(err) => {
                                            event.send_message_to_source(err.to_string().parse_message_chain()).await?;
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
            }
            _ => {
                event.at_text("没有这个子指令喵...").await?;
                Ok(true)
            }
        }
    } else {
        event.reply(
            MessageChain::new()
                .text("可用子指令:")
                .text(">    info")
                .text(">    build")
                .text(">    untie")
                .build()
        ).await?;
        return Ok(true);
    }
}

