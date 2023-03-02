use async_trait::async_trait;
use etternaonline_api::Error;
use etternaonline_api::v2::{Session, UserDetails};
use once_cell::sync::Lazy;
use proc_qq::{event, event_fn, MessageChainParseTrait, MessageContentTrait, MessageEvent, MessageEventProcess, MessageSendToSourceTrait, Module, module, ModuleEventHandler, ModuleEventProcess};
use rbatis::dark_std::err;
use crate::{BotResult, CONTEXT};
use crate::msg_util::MessageChain;
use crate::utils::http_util::http_get_image;
use crate::utils::image::ett::{ETT_CLIENT};
use crate::utils::image::ett::ett_user_info_image::EttUserInfoImage;
use crate::utils::Reg;

static ID: &'static str = "ett user module";
static NAME: &'static str = "Ett用户模块";

pub(crate) fn module() -> Module {
    module!(
        ID,
        NAME,
        ett_user_info_handle,
    )
}

#[event]
async fn ett_user_info_handle(event: &MessageEvent) -> anyhow::Result<bool>{
    let content = event.message_content();
    let (reg_build, msg_array_build) = Reg::ex_msg(content.as_str(), &["/ett[\\s]+build[\\s]+(.*)"], None);
    let reg_untie = Reg::ex(content.as_str(), &["/ett[\\s]+untie $"], None);
    let reg_info = Reg::ex(content.as_str(), &["/ett[\\s]+info $"], None);
    let (reg_info_name, msg_array_info_name) = Reg::ex_msg(content.as_str(), &["/ett[\\s]+info[\\s]+(.*)"], None);

    if reg_build {
        return match CONTEXT.ett.ett_build_by_name_qq(&msg_array_build[2], &&event.from_uin()).await {
            Ok(_) => {
                event.send_message_to_source("绑定成功喵!".parse_message_chain()).await?;
                Ok(true)
            }
            Err(err) => {
                event.send_message_to_source(err.to_msg()).await?;
                Ok(true)
            }
        }
    }
    if reg_untie {
        let ett_user = CONTEXT.ett.ett_select_by_name_qq(&event.from_uin()).await;
        return match ett_user {
            Ok(ett_user) => {
                match CONTEXT.ett.ett_untie_by_qq(&event.from_uin()).await {
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
    if reg_info {
        let ett_user = CONTEXT.ett.ett_select_by_name_qq(&event.from_uin()).await;
        return match ett_user {
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
                                let image = http_get_image(&format!("https://etternaonline.com/avatars/{}", data.avatar_url)).await?;
                                let string = serde_json::to_string(&data.rating).unwrap_or_default();
                                tracing::debug!("{:?}",&string);
                                CONTEXT.ett.ett_update_rating_time_by_qq(&event.from_uin(), string, chrono::Local::now().naive_utc()).await?;

                                let res = EttUserInfoImage::new(data, Some(ett_user.rating), ett_user.update_time).ok(&image);
                                match res {
                                    Ok(ok) => {
                                        let chain = MessageChain::new().image_vec(ok, &event).await.ok();
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
    if reg_info_name {
        return match ETT_CLIENT.as_ref() {
            Err(err) => {
                event.send_message_to_source(format!("ett client 错误 ,请联系bot主人 Error: {}", err).parse_message_chain()).await?;
                Ok(true)
            }
            Ok(session) => {
                match session.user_details(msg_array_info_name[2].as_str()) {
                    Ok(data) => {
                        tracing::debug!("{:?}",&data);
                        let image = http_get_image(&format!("https://etternaonline.com/avatars/{}", data.avatar_url)).await?;
                        let res = EttUserInfoImage::new(data, None, chrono::Local::now().naive_local()).ok(&image);
                        match res {
                            Ok(ok) => {
                                let chain = MessageChain::new().image_vec(ok, &event).await.ok();
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
    }
    Ok(false)
}

