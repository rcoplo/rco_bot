use std::sync::Arc;
use std::time::Duration;

use crate::msg_util::{MessageChain, CanReply};

use proc_qq::{module, MessageChainParseTrait, MessageEvent, MessageSendToSourceTrait, Module, event, ClientTrait, MessageChainPointTrait, MessageContentTrait};
use proc_qq::re_exports::{anyhow, tokio, tracing};
use proc_qq::re_exports::ricq::{Client};
use proc_qq::re_exports::ricq_core::structs::MessageReceipt;
use crate::CONTEXT;
use crate::api::setu_api::{LoliconApiBuilder};

pub(crate) fn module() -> Module {
    module!(
        "setu",
        "涩图",
        setu,
        setur,
    )
}

enum SetuType {
    Array(LoliconApiBuilder),
    Single(LoliconApiBuilder),
}

#[event(bot_command = "/色图 {data}")]
async fn setu(event: &MessageEvent, data: Vec<String>) -> anyhow::Result<bool> {
    let setu = setu_builder(data, false);
    setu_send(event, setu).await?;
    Ok(false)
}

#[event(bot_command = "/色图r {data}")]
async fn setur(event: &MessageEvent, data: Vec<String>) -> anyhow::Result<bool> {
    let setu = setu_builder(data, true);
    setu_send(event, setu).await?;
    Ok(false)
}

async fn delete_msg(client: Arc<Client>, data: MessageReceipt, group_id: i64) -> anyhow::Result<()> {
    let client = client.clone();
    tokio::spawn(async move {
        tokio::time::sleep(Duration::from_secs(CONTEXT.config.setu.recall_time as u64)).await;
        client.recall_group_message(
            group_id,
            data.seqs,
            data.rands,
        ).await;
        tracing::info!("色图撤回成功喵!");
    });
    Ok(())
}

async fn setu_send(event: &MessageEvent, setu: SetuType) -> anyhow::Result<bool> {
    return match setu {
        SetuType::Array(a) => {
            match LoliconApiBuilder::get_array(&a).await {
                Ok(setu) => {
                    for data in setu {
                        let chain = MessageChain::new()
                            .text(format!("title: {}\n", data.title))
                            .text(format!("pid: {}\n", data.pid))
                            .text(format!("author: {}\n", data.author))
                            .image(&data.urls.original.replace("i.pixiv.re", "pixiv.rco.ink"), &event)
                            .await
                            .build();
                        match event.reply(chain).await {
                            Ok(s) => {
                                if s.seqs[0] == 0 {
                                    let receipt = event.send_message_to_source(
                                        "这张发送失败喵...".parse_message_chain())
                                        .await?;
                                    delete_msg(event.client(),
                                               receipt,
                                               event.as_group_message().unwrap().inner.group_code).await?;
                                } else {
                                    delete_msg(event.client(),
                                               s,
                                               event.as_group_message().unwrap().inner.group_code).await?;
                                }
                            }
                            Err(_) => {
                                let receipt = event.send_message_to_source(
                                    "这张发送失败喵...".parse_message_chain())
                                    .await?;
                                delete_msg(event.client(),
                                           receipt,
                                           event.as_group_message().unwrap().inner.group_code).await?;
                            }
                        }
                    }
                    Ok(true)
                }
                Err(err) => {
                    event.send_message_to_source(err.to_msg()).await?;
                    Ok(true)
                }
            }
        }
        SetuType::Single(s) => {
            match LoliconApiBuilder::get(&s).await {
                Ok(setu) => {
                    let chain = MessageChain::new()
                        .text(format!("title: {}\n", setu.title))
                        .text(format!("pid: {}\n", setu.pid))
                        .text(format!("author: {}\n", setu.author))
                        .image(&setu.urls.original.replace("i.pixiv.re", "pixiv.rco.ink"), &event)
                        .await
                        .build();
                    match event.reply(chain).await {
                        Ok(s) => {
                            if s.seqs[0] == 0 {
                                let receipt = event.send_message_to_source(
                                    "色图发送失败喵...".parse_message_chain())
                                    .await?;
                                delete_msg(event.client(),
                                           receipt,
                                           event.as_group_message().unwrap().inner.group_code).await?;
                            } else {
                                delete_msg(event.client(),
                                           s,
                                           event.as_group_message().unwrap().inner.group_code).await?;
                            }
                        }
                        Err(_) => {
                            let receipt = event.send_message_to_source(
                                "色图发送失败喵...".parse_message_chain())
                                .await?;
                            delete_msg(event.client(),
                                       receipt,
                                       event.as_group_message().unwrap().inner.group_code).await?;
                        }
                    }
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

fn setu_builder(data: Vec<String>, is_r18: bool) -> SetuType {
    let mut builder = LoliconApiBuilder::new();
    if is_r18 {
        if !data.is_empty() {
            match data.first().unwrap().parse::<i8>() {
                Ok(n) => {
                    println!("{}", data.len());
                    if data.len() > 1 {
                        let mut data = data;
                        data.remove(0);
                        SetuType::Array(builder.tag(data).num(n).r18().build())
                    } else {
                        if n <= 20 {
                            SetuType::Array(builder.num(n).r18().build())
                        } else {
                            SetuType::Single(builder.r18().build())
                        }
                    }
                }
                Err(_) => {
                    SetuType::Single(builder.tag(data).r18().build())
                }
            }
        } else {
            SetuType::Single(builder.r18().build())
        }
    } else {
        if !data.is_empty() {
            match data.first().unwrap().parse::<i8>() {
                Ok(n) => {
                    if data.len() > 1 {
                        let mut data = data;
                        data.remove(0);
                        SetuType::Array(builder.tag(data).num(n).no_r18().build())
                    } else {
                        if n <= 20 {
                            SetuType::Array(builder.num(n).no_r18().build())
                        } else {
                            SetuType::Single(builder.no_r18().build())
                        }
                    }
                }
                Err(_) => {
                    SetuType::Single(builder.tag(data).no_r18().build())
                }
            }
        } else {
            SetuType::Single(builder.no_r18().build())
        }
    }
}