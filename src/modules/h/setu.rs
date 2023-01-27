use proc_qq::{event, MessageChainAppendTrait, MessageChainParseTrait, MessageChainPointTrait, MessageContentTrait, MessageEvent, MessageSendToSourceTrait, Module, module, TextEleParseTrait};
use proc_qq::re_exports::ricq_core::msg::elem::{FlashImage, GroupImage, Text};
use crate::api::lolicon::{get_lolicon, get_lolicon_r18, Setu};
use crate::msg_util::MessageChain;
use crate::utils::{CanReply, Reg};

static ID: &'static str = "setu";
static NAME: &'static str = "涩图";


#[event]
async fn setu(msg:&MessageEvent) -> anyhow::Result<bool>{
    let content = msg.message_content();
    if msg.is_group_message() {
        if Reg::ex(&content,&["色图","涩图"],Some(&[Reg::All])){
            let setu = get_lolicon().await;
            match setu {
                None => {
                    msg.send_message_to_source("发送涩图失败喵!".parse_message_chain()).await?;
                }
                Some(data) => {
                    let chain = MessageChain::new()
                        .text(format!("title: {}\n",data.title))
                        .text(format!("pid: {}\n",data.pid))
                        .text(format!("author: {}\n",data.author))
                        .image(&data.urls.original,&msg)
                        .await
                        .ok();
                    return if msg.send_message_to_source(chain).await.is_ok() {
                        Ok(true)
                    } else {
                        Ok(false)
                    }

                }
            }
        }
    }
    if msg.is_private_message(){
        return if msg.send_message_to_source("涩图指令不支持私聊使用喵...".parse_message_chain()).await.is_ok() {
            Ok(true)
        } else {
            Ok(false)
        }
    }
    if msg.is_temp_message(){
        return if msg.send_message_to_source("涩图指令不支持临时私聊使用喵...".parse_message_chain()).await.is_ok() {
            Ok(true)
        } else {
            Ok(false)
        }
    }
    Ok(false)
}

pub(crate) fn setu_module() -> Module {
    module!(
        ID,
        NAME,
        setu,
    )
}