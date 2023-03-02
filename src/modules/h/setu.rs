use std::num::ParseIntError;
use std::str::FromStr;
use chrono::{Local, NaiveDateTime};
use crate::api::lolicon::{get_lolicon, get_lolicon_list, get_lolicon_list_tag, get_lolicon_tag};
use crate::msg_util::{ForwardNodeTrait, MessageChain,forward_message};
use crate::utils::Reg;
use proc_qq::{module, MessageChainParseTrait, MessageContentTrait, MessageEvent, MessageSendToSourceTrait, Module, event, MessageRecallTrait, ClientTrait, MessageChainPointTrait};
use proc_qq::re_exports::ricq::RQResult;
use rbatis::dark_std::err;
use crate::BotError;

static ID: &'static str = "setu";
static NAME: &'static str = "涩图";

pub struct SetuHelp {
    pub mod_name:String,
    pub help_text:Vec<String>,
}
impl Default for SetuHelp{
    fn default() -> Self {
        SetuHelp{
            mod_name: "色图".to_string(),
            help_text: vec![
                "setu_help",
                "----------------------------------------------------------------",
                "/色图",
                "子指令: /色图  <tag>  <tag>",
                "             /色图#num",
                "             /色图sep#<num>",
                "             /色图sep#<num>  <tag>  <tag>",
                "----------------------------------------------------------------",
                "<tag>  是色图的标签  中间使用空格间隔 ",
                " 例: /色图 萝莉 白丝",
                "<num> 是色图数量 最大速20 ,因为api一次性最大只能获取20条数据 ",
                " 例: /色图#5 <tag>...",
                "多张色图默认发送合并转发,如果加上sep 代表一张一张发送",
                " 例: /色图sep#5 <tag>...",
            ].iter().map(|str| str.to_string()).collect::<Vec<_>>(),
        }
    }
}
pub(crate) fn module() -> Module {
    module!(ID,
        NAME,
        setu,
        setu_tag,
        setu_list,
        setu_list_tag,
    )
}

#[event]
async fn setu(event: &MessageEvent) -> anyhow::Result<bool> {
    if Reg::ex(event.message_content().as_str(), &["色图 $", "涩图 $"], Some(&[Reg::All])) {
        let setu = get_lolicon().await;
        match setu {
            Err(err) => {
                event.send_message_to_source(err.to_msg())
                    .await?;
            }
            Ok(data) => {
                tracing::debug!("Setu = {:?}", &data);
                let chain = MessageChain::new()
                    .reply(&event)
                    .text(format!("title: {}\n", data.title))
                    .text(format!("pid: {}\n", data.pid))
                    .text(format!("author: {}\n", data.author))
                    .image(&data.urls.original, &event)
                    .await
                    .ok();
                return match event.send_message_to_source(chain).await {
                    Ok(data) => {
                        if data.seqs[0] == 0 {
                            event.send_message_to_source(
                                "色图发送失败喵...".parse_message_chain())
                                .await?;
                        }
                        Ok(true)
                    }
                    Err(err) => {
                        event.send_message_to_source(err.to_string().parse_message_chain())
                            .await?;
                        Ok(true)
                    }
                }
            }
        }
    }
    Ok(false)
}
#[event]
async fn setu_tag(event: &MessageEvent)-> anyhow::Result<bool> {
    let content = event.message_content();
    let (bool, mut msg_array) = Reg::ex_msg(content.as_str(), &["色图[\\s]+(.*)", "涩图[\\s]+(.*)"], Some(&[Reg::All]));
    if bool {
        msg_array.remove(0);
        let setu = get_lolicon_tag(msg_array).await;
        match setu {
            Err(err) => {
                event.send_message_to_source(err.to_msg())
                    .await?;
            }
            Ok(data) => {
                tracing::debug!("Setu = {:?}", &data);
                let chain = MessageChain::new()
                    .reply(&event)
                    .text(format!("title: {}\n", data.title))
                    .text(format!("pid: {}\n", data.pid))
                    .text(format!("author: {}\n", data.author))
                    .image(&data.urls.original, &event)
                    .await
                    .ok();

                return match event.send_message_to_source(chain).await {
                    Ok(data) => {
                        if data.seqs[0] == 0 {
                            event.send_message_to_source(
                                "色图发送失败喵...".parse_message_chain())
                                .await?;
                        }
                        Ok(true)
                    }
                    Err(err) => {
                        event.send_message_to_source(err.to_string().parse_message_chain())
                            .await?;
                        Ok(true)
                    }
                }
            }
        }
    }
    Ok(false)
}
#[event]
async fn setu_list(event: &MessageEvent)-> anyhow::Result<bool> {
    let content = event.message_content();
    let (bool, mut msg_array) = Reg::ex_msg(content.as_str(), &["色图#([1-9]*|10|20) $", "涩图#([1-9]*|10|20) $"], Some(&[Reg::All]));
    if bool {
        let vec = msg_array[0].split("#").collect::<Vec<_>>();
        let num = match vec[1].parse::<i8>(){
            Ok(data) => Ok(data),
            Err(err) => {
                Err(BotError::from(format!("{} , 参数是否不是整数?",err)))
            }
        };

        let setu =  match num {
            Ok(data) =>  get_lolicon_list(data).await,
            Err(err) => Err(err)
        };
        match setu {
            Err(err) => {
                event.send_message_to_source(err.to_msg())
                    .await?;
            }
            Ok(data) => {
                let mut chain = MessageChain::new();
                let mut vec = vec![];
                for setu in data {
                    vec.push( MessageChain::new()
                        .text(format!("title: {}\n", setu.title))
                        .text(format!("pid: {}\n", setu.pid))
                        .text(format!("author: {}\n", setu.author))
                        .image(&setu.urls.original, &event)
                        .await
                        .ok());
                }
                let forward = forward_message((&event.bot_uin().await, "天天看色图,超市你"), vec);
                return match event.send_forward_msg(forward).await {
                    Ok(data) => {
                        if data.seqs[0] == 0 {
                            event.send_message_to_source(
                                "色图发送失败喵...".parse_message_chain())
                                .await?;
                        }
                        Ok(true)
                    }
                    Err(err) => {
                        event.send_message_to_source(err.to_string().parse_message_chain())
                            .await?;
                        Ok(true)
                    }
                }
            }
        }
    }
    let (bool, mut msg_array) = Reg::ex_msg(content.as_str(), &["色图(sep)#([1-9]*|10|20)$", "涩图(sep)#([1-9]*|10|20)$"], Some(&[Reg::All]));
    if bool {
        let vec = msg_array[0].split("#").collect::<Vec<_>>();
        let num = match vec[1].parse::<i8>(){
            Ok(data) => Ok(data),
            Err(err) => {
                Err(BotError::from(format!("{} , 参数是否不是整数?",err)))
            }
        };

        let setu =  match num {
            Ok(data) =>  get_lolicon_list(data).await,
            Err(err) => Err(err)
        };
        match setu {
            Err(err) => {
                event.send_message_to_source(err.to_msg())
                    .await?;
            }
            Ok(data) => {
                for setu in data {
                    let mut chain = MessageChain::new()
                        .text(format!("title: {}\n", setu.title))
                        .text(format!("pid: {}\n", setu.pid))
                        .text(format!("author: {}\n", setu.author))
                        .image(&setu.urls.original, &event)
                        .await
                        .ok();
                    match event.send_message_to_source(chain).await {
                        Ok(data) => {
                            if data.seqs[0] == 0 {
                                event.send_message_to_source(
                                    "色图发送失败喵...".parse_message_chain())
                                    .await?;
                            }
                        }
                        Err(err) => {
                            event.send_message_to_source(err.to_string().parse_message_chain())
                                .await?;
                        }
                    };
                }
                return Ok(true);
            }
        }
    }
    Ok(false)
}
#[event]
async fn setu_list_tag(event: &MessageEvent)-> anyhow::Result<bool> {
    let content = event.message_content();
    let (bool, mut msg_array) = Reg::ex_msg(content.as_str(), &["色图#([1-9]*|10|20)[\\s+](.*)", "涩图#([1-9]*|10|20)[\\s+](.*)"], Some(&[Reg::All]));
    if bool {
        let vec = msg_array[0].split("#").collect::<Vec<_>>();
        let num = match vec[1].parse::<i8>(){
            Ok(data) => Ok(data),
            Err(err) => {
                Err(BotError::from(format!("{} , 参数是否不是整数?",err)))
            }
        };
        msg_array.remove(0);
        let setu =  match num {
            Ok(data) =>  get_lolicon_list_tag(data,msg_array).await,
            Err(err) => Err(err)
        };
        match setu {
            Err(err) => {
                event.send_message_to_source(err.to_msg())
                    .await?;
            }
            Ok(data) => {
                let mut chain = MessageChain::new();
                let mut vec = vec![];
                    for setu in data {
                        vec.push( MessageChain::new()
                            .text(format!("title: {}\n", setu.title))
                            .text(format!("pid: {}\n", setu.pid))
                            .text(format!("author: {}\n", setu.author))
                            .image(&setu.urls.original, &event)
                            .await
                            .ok());
                    }
                let forward = forward_message((&event.bot_uin().await, "天天看色图,超市你"), vec);
                return match event.send_forward_msg(forward).await {
                    Ok(data) => {
                        if data.seqs[0] == 0 {
                            event.send_message_to_source(
                                "色图发送失败喵...".parse_message_chain())
                                .await?;
                        }
                        Ok(true)
                    }
                    Err(err) => {
                        event.send_message_to_source(err.to_string().parse_message_chain())
                            .await?;
                        Ok(true)
                    }
                }
            }
        }
    }
    let (bool, mut msg_array) = Reg::ex_msg(content.as_str(), &["色图(sep)#([1-9]*|10|20)[\\s+](.*)", "涩图(sep)#([1-9]*|10|20)[\\s+](.*)"], Some(&[Reg::All]));
    if bool {
        let vec = msg_array[0].split("#").collect::<Vec<_>>();
        let num = match vec[1].parse::<i8>(){
            Ok(data) => Ok(data),
            Err(err) => {
                Err(BotError::from(format!("{} , 参数是否不是整数?",err)))
            }
        };
        msg_array.remove(0);
        let setu =  match num {
            Ok(data) =>  get_lolicon_list_tag(data,msg_array).await,
            Err(err) => Err(err)
        };
        match setu {
            Err(err) => {
                event.send_message_to_source(err.to_msg())
                    .await?;
            }
            Ok(data) => {
                for setu in data {
                    let mut chain = MessageChain::new()
                        .text(format!("title: {}\n", setu.title))
                        .text(format!("pid: {}\n", setu.pid))
                        .text(format!("author: {}\n", setu.author))
                        .image(&setu.urls.original, &event)
                        .await
                        .ok();
                    match event.send_message_to_source(chain).await {
                        Ok(data) => {
                            if data.seqs[0] == 0 {
                                event.send_message_to_source(
                                    "色图发送失败喵...".parse_message_chain())
                                    .await?;
                            }
                        }
                        Err(err) => {
                            event.send_message_to_source(err.to_string().parse_message_chain())
                                .await?;
                        }
                    };
                }
                return Ok(true);
            }
        }
    }
    Ok(false)
}