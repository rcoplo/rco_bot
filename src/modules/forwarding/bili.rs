use proc_qq::{event, GroupMessageEvent, MessageChainParseTrait, MessageChainPointTrait, MessageContentTrait, MessageEvent, MessageSendToSourceTrait, Module, module};
use serde_json::Value;

use crate::{BotError, BotResult, CONTEXT};
use crate::api::bili_api::BiliApi;
use crate::chrome_util::{bili_dynamic_screenshot, bili_video_screenshot};
use crate::database::table::BiliPush;
use crate::msg_util::MessageChain;
use crate::utils::Reg;

static ID: &'static str = "bili_push";
static NAME: &'static str = "bili推送";
static BILI_HELP: &'static str =
    r##"bili推送 Help
    [/关注 <uid>]
    - 关注一个up主
    [/取消关注 <uid>]
    - 取消关注一个up主
    [/查看关注列表]
    - 查看本群关注的up主列表
    [/查看关注 <uid>]
    - 查看关注的up主的推送开关
    "##;

pub fn module() -> Module{
    module!(
        ID,
        NAME,
        bili_push_all,
    )
}
#[event]
async fn bili_push_all(event:&GroupMessageEvent)-> anyhow::Result<bool>{
    let content = event.message_content();
    if Reg::ex(content.as_str(), &["/关注[\\s]*[0-9]*"], None) {
        bili_push_concern(&event, &content).await?;
        return Ok(true);
    } else if Reg::ex(content.as_str(), &["/取消关注[\\s]*[0-9]*"], None) {
        bili_push_delete(&event, &content).await?;
        return Ok(true);
    } else if Reg::ex(content.as_str(), &["/查看关注列表"], None) {
        bili_push_select_all(&event).await?;
        return Ok(true);
    } else if Reg::ex(content.as_str(), &["/查看关注[\\s]*[0-9]*"], None) {
        bili_push_select(&event, &content).await?;
        return Ok(true);
    }
    Ok(false)
}

async fn bili_push_concern(event:&GroupMessageEvent,content:&String) -> anyhow::Result<bool> {
    let bili_id = content.replace("/关注", "").replace(" ", "").parse::<i64>();
    let bot_res = match bili_id {
        Ok(uid) => {
            if CONTEXT.bili_push.select_up_is_null(&uid).await.is_some() {
                let bot_res = CONTEXT.bili_push.update_group_id(&uid, &event.inner.group_code).await;
                match bot_res {
                    Ok(data) => {
                        Ok(BiliApi{
                            room_id: data.room_id,
                            uid,
                            uname: data.uname,
                        })
                    }
                    Err(err) => {Err(err)}
                }
            }else {
                let bot_res = CONTEXT.bili_push.insert(&uid, &event.inner.group_code).await;
                bot_res
            }
        }
        Err(err) => {
            Err(BotError::from(err.to_string()))
        }
    };
    match bot_res {
        Ok(data) => {
            event.send_message_to_source(format!("关注up主 {}({}) 成功喵!", data.uname.as_str(), data.uid).parse_message_chain()).await?;
            return Ok(true);
        }
        Err(err) => {
            if let BotError::MsgChain(msg) = err {
                event.send_message_to_source(msg).await?;
                return Ok(true);
            }

        }
    }
    Ok(false)
}

async fn bili_push_delete(event:&GroupMessageEvent,content:&String) -> anyhow::Result<bool>{
    let bili_id = content.replace("/取消关注", "").replace(" ", "").parse::<i64>();
    match bili_id {
        Ok(id) => {
            let res = CONTEXT.bili_push.unfollow_up(&id, &event.inner.group_code).await;
            match res {
                Ok(_) => {
                    event.send_message_to_source(format!("取消关注up主 ({}) 成功喵!", id).parse_message_chain()).await?;
                    return Ok(true);
                }
                Err(err) => {
                    if let BotError::MsgChain(msg) = err {
                        event.send_message_to_source(msg).await?;
                        return Ok(true);
                    }
                }
            }
        }
        Err(err) => {
            event.send_message_to_source(err.to_string().parse_message_chain()).await?;
            return Ok(true);
        }
    }
    Ok(false)
}
#[event]
async fn bili_push_update_push(event:&GroupMessageEvent) -> anyhow::Result<bool>{
    let content = event.message_content();
    if Reg::ex(content.as_str(), &["/test"], None) {}
    Ok(false)
}

async fn bili_push_select_all(event:&GroupMessageEvent) -> anyhow::Result<bool>{
    let res = CONTEXT.bili_push.select_list(&event.inner.group_code).await;
    return match res {
        Ok(data) => {
            let mut chain = MessageChain::new();
            chain.text("以下是本群的关注列表: \n");
            for (uid, uname) in data {
                chain.text(format!("{}({}) \n", uname, uid));
            }
            event.send_message_to_source(chain.ok()).await?;
            Ok(true)
        }
        Err(err) => {
            event.send_message_to_source(err.to_string().parse_message_chain()).await?;
            Ok(true)
        }
    }
}
async fn bili_push_select(event:&GroupMessageEvent,content:&String) -> anyhow::Result<bool>{
    let bili_id = content.replace("/查看关注", "").replace(" ", "").parse::<i64>();
    return match bili_id {
        Ok(id) => {
            let result = CONTEXT.bili_push.select_push_switch(&id).await;
            return match result {
                Ok((bili, (dy,v,l))) => {
                    let mut chain = MessageChain::new();
                    chain.text(format!("{}({}):\n", bili.uname, bili.uid));
                    chain.text(format!("动态推送: {}\n", switch(&dy)));
                    chain.text(format!("视频推送: {}\n", switch(&v)));
                    chain.text(format!("直播推送: {}\n", switch(&l)));
                    event.send_message_to_source(chain.ok()).await?;
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


#[event]
async fn bili_live_push(event:&GroupMessageEvent) -> anyhow::Result<bool>{
    let vec = CONTEXT.bili_push.select_all().await?;
    // for x in vec {
    //     let (bili_push,(dy,v,l)) = CONTEXT.bili_push.select_push_switch(&x.uid).await?;
    //     if dy {
    //         let mut chain = MessageChain::new();
    //         chain.text(format!("{} 发送新动态了喵!\n",bili_push.uname));
    //         let result = bili_dynamic_screenshot(&bili_push.uid).await?;
    //         chain.image_or(&result,&MessageEvent::GroupMessage(event.clone()));
    //         let vec1 = bili_push.to_vec();
    //         for x in vec1 {
    //             event.client.send_group_message(x,chain.ok()).await?;
    //         }
    //     }
    // }
    Ok(false)
}

fn switch(bool:&bool) -> String{
    if *bool{
        "开".to_string()
    }else {
        "关".to_string()
    }

}


async fn bili_live_push_impl() -> MessageChain {

    todo!()
}



impl BiliPush{
    fn to_vec(&self) -> Vec<i64>{
        serde_json::from_str(self.group_id.as_str()).unwrap()
    }
}