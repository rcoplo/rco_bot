use std::fmt::Debug;
use std::io::Read;
use proc_qq::{FriendMessageEvent, GroupMessageEvent, GroupTempMessageEvent, MessageChainAppendTrait, MessageChainParseTrait, MessageChainPointTrait, MessageContentTrait, MessageEvent, MessageSendToSourceTrait, TextEleParseTrait, UploadImage};

use proc_qq::re_exports::ricq::{Client, RQResult};
use proc_qq::re_exports::ricq::structs::GroupMemberPermission;
use proc_qq::re_exports::ricq_core::msg::{MessageChain as ricqMessageChain, MessageChainBuilder};
use proc_qq::re_exports::ricq_core::msg::elem::{At, Text, Face, Reply};
use proc_qq::re_exports::ricq_core::RQError;
use proc_qq::re_exports::ricq_core::structs::{ForwardMessage, MessageNode, MessageReceipt};
use proc_qq::re_exports::{tokio, tracing};
use proc_qq::re_exports::async_trait::async_trait;
use rbatis::dark_std::err;


use tokio::io::AsyncReadExt;

use crate::{BotError, BotResult, CONTEXT};


#[derive(Debug, Clone)]
pub struct MessageChain {
    inner: ricqMessageChain,
}

impl MessageChain {
    pub fn new() -> Self {
        Self {
            inner: MessageChainBuilder::new().build(),
        }
    }

    pub fn text<T: AsRef<str>>(&mut self, text: T) -> &mut MessageChain {
        self.inner.push(Text::new(text.as_ref().to_string()));
        self
    }

    pub async fn image(&mut self, image: &str, event: &MessageEvent) -> &mut MessageChain {
        tracing::debug!("image: {}", image);
        match match image.split("://").collect::<Vec<_>>().first() {
            None => {
                Err(BotError::from(format!("上传图片失败喵... \n 没有图片地址前辍喵...")))
            }
            Some(v) => {
                match *v {
                    "http" | "https" => {
                        match crate::utils::http_util::http_get_image(image).await {
                            Ok(b) => {
                                event.upload_image_to_source(b).await.map_err(|err| { BotError::from(format!("上传图片失败喵... \nRQError: {}", err)) })
                            }
                            Err(err) => {
                                Err(err)
                            }
                        }
                    }
                    "bytes" => {
                        let bytes = image.replace("bytes://", "").bytes().collect::<Vec<_>>();
                        event.upload_image_to_source(bytes).await.map_err(|err| { BotError::from(format!("上传图片失败喵... \nRQError: {}", err)) })
                    }
                    "file" => {
                        let file = image.replace("file://", "");
                        let mut f = tokio::fs::File::open(file).await.map_err(|err| { BotError::from(format!("上传图片失败喵... \nError: {}", err)) }).unwrap();
                        let mut b = vec![];
                        f.read_to_end(&mut b).await.map_err(|err| { BotError::from(format!("上传图片失败喵... \nError: {}", err)) });
                        event.upload_image_to_source(b).await.map_err(|err| { BotError::from(format!("上传图片失败喵... \nRQError: {}", err)) })
                    }
                    _ => Err(BotError::from("上传图片失败喵... \n 给予图片地址前辍不匹配喵..."))
                }
            }
        } {
            Ok(image) => {
                self.inner.push(image);
            }
            Err(err) => {
                self.inner.push(Text::new(err.to_string()));
            }
        }
        self
    }
    pub async fn image_bytes(&mut self, data: Vec<u8>, event: &MessageEvent) -> &mut MessageChain {
        let upload_res = event.upload_image_to_source(data).await;
        match upload_res {
            Ok(image) => {
                self.inner.push(image);
            },
            Err(err) => {
                self.inner.push(Text::new(format!("上传图片失败喵... \nRQError: {}", err)));
            },
        }
        self
    }

    pub fn at(&mut self, user_id: i64) -> &mut MessageChain {
        self.inner.push(At::new(user_id));
        self
    }

    pub fn face(&mut self, id: i32) -> &mut MessageChain {
        self.inner.push(Face::new(id));
        self
    }

    pub fn build(&self) -> ricqMessageChain {
        self.inner.clone()
    }
}

pub fn forward_message(node_config:(&i64,&str),message_node:Vec<ricqMessageChain>) -> Vec<ForwardMessage> {
    let mut vec = vec![];
    let time = chrono::Local::now().timestamp() as i32;
    for msg in message_node {
        vec.push( ForwardMessage::from(MessageNode{
            sender_id: *node_config.0,
            sender_name: node_config.1.to_string(),
            elements: msg,
            time,
        }));
    }
    vec
}
#[async_trait]
pub(crate) trait CanReply {
    async fn make_at_chain(&self) -> ricqMessageChain;
    async fn make_reply_chain(&self) -> ricqMessageChain;
    async fn at_text(&self, text: &str) -> RQResult<MessageReceipt>;
    async fn reply_text(&self, text: &str) -> RQResult<MessageReceipt>;
    async fn at<S: Into<ricqMessageChain> + Send + Sync>(&self, message: S) -> RQResult<MessageReceipt>;
    async fn reply<S: Into<ricqMessageChain> + Send + Sync>(&self, message: S) -> RQResult<MessageReceipt>;
    async fn send(&self, text: &str) -> RQResult<MessageReceipt>;
}

#[async_trait]
pub(crate) trait ForwardNodeTrait {
    async fn send_forward_msg(&self,msgs: Vec<ForwardMessage>) -> RQResult<MessageReceipt>;
}

#[async_trait]
impl ForwardNodeTrait for GroupMessageEvent{
    async fn send_forward_msg(&self,msgs: Vec<ForwardMessage>) -> RQResult<MessageReceipt> {
        self.client.send_group_forward_message(self.inner.group_code,msgs).await
    }
}

#[async_trait]
impl ForwardNodeTrait for MessageEvent{
    async fn send_forward_msg(&self,msgs: Vec<ForwardMessage>) -> RQResult<MessageReceipt> {
        match self {
            MessageEvent::GroupMessage(group_message) => group_message.send_forward_msg(msgs).await,
            _ => Err(RQError::Other(
                "The forward message does not support other message types".to_owned(),
            )),
        }
    }
}

#[async_trait]
impl CanReply for GroupMessageEvent {
    async fn make_at_chain(&self) -> ricqMessageChain {
        let mut at = At::new(self.inner.from_uin);
        at.display = format!("@{}", self.inner.group_card);
       ricqMessageChain::default().append(at)
           .append(" ".parse_text())
    }

    async fn make_reply_chain(&self) -> ricqMessageChain {
        let reply = Reply {
            reply_seq: self.inner.seqs[0],
            sender: self.inner.from_uin,
            time: self.inner.time,
            elements: self.inner.elements.clone(),
        };
        let mut chain = ricqMessageChain::default();
        chain.with_reply(reply);
        chain
    }

    async fn at_text(&self, text: &str) -> RQResult<MessageReceipt> {
        self.send_message_to_source(self.make_at_chain().await.append(text.parse_text()))
            .await
    }

    async fn reply_text(&self, text: &str) -> RQResult<MessageReceipt> {
        self.send_message_to_source(
            self
                .make_reply_chain().await
                .append(text.parse_text())).await
    }

    async fn at<S: Into<ricqMessageChain> + Send + Sync>(&self, message: S) -> RQResult<MessageReceipt> {
        let mut chain = self.make_at_chain().await;
        chain.push(message.into().0);
        self.send_message_to_source(
            chain).await
    }

    async fn reply<S: Into<ricqMessageChain> + Send + Sync>(&self, message: S) -> RQResult<MessageReceipt> {
        let mut chain = self.make_reply_chain().await;
        chain.push(message.into().0);
        self.send_message_to_source(
            chain).await
    }

    async fn send(&self, text: &str) -> RQResult<MessageReceipt> {
        self.send_message_to_source(
            text.parse_message_chain()).await
    }
}
#[async_trait]
impl CanReply for FriendMessageEvent {
    async fn make_at_chain(&self) -> ricqMessageChain {
        let mut at = At::new(self.inner.target);
        at.display = format!("@{}", self.inner.from_uin);
        ricqMessageChain::default().append(at)
            .append(" ".parse_text())
    }

    async fn make_reply_chain(&self) -> ricqMessageChain {
        let reply =Reply {
            reply_seq: self.inner.seqs[0],
            sender: self.inner.from_uin,
            time: self.inner.time,
            elements: self.inner.elements.clone(),
        };
        let mut chain = ricqMessageChain::default();
        chain.with_reply(reply);
        chain
    }

    async fn at_text(&self, text: &str) -> RQResult<MessageReceipt> {
        self.send_message_to_source(self.make_at_chain().await.append(text.parse_text()))
            .await
    }

    async fn reply_text(&self, text: &str) -> RQResult<MessageReceipt> {
        self.send_message_to_source(
            self
                .make_reply_chain().await
                .append(text.parse_text())).await
    }
    async fn at<S: Into<ricqMessageChain> + Send + Sync>(&self, message: S) -> RQResult<MessageReceipt> {
        let mut chain = self.make_at_chain().await;
        chain.push(message.into().0);
        self.send_message_to_source(
            chain).await
    }

    async fn reply<S: Into<ricqMessageChain> + Send + Sync>(&self, message: S) -> RQResult<MessageReceipt> {
        let mut chain = self.make_reply_chain().await;
        chain.push(message.into().0);
        self.send_message_to_source(
            chain).await
    }
    async fn send(&self, text: &str) -> RQResult<MessageReceipt> {
        self.send_message_to_source(
            text.parse_message_chain()).await
    }
}

#[async_trait]
impl CanReply for GroupTempMessageEvent {
    async fn make_at_chain(&self) -> ricqMessageChain {
        let mut at = At::new(self.inner.from_uin);
        at.display = format!("@{}", self.inner.from_uin);
        ricqMessageChain::default().append(at)
            .append(" ".parse_text())
    }

    async fn make_reply_chain(&self) -> ricqMessageChain {
        let reply =Reply {
            reply_seq: self.inner.seqs[0],
            sender: self.inner.from_uin,
            time: self.inner.time,
            elements: self.inner.elements.clone(),
        };
        let mut chain = ricqMessageChain::default();
        chain.with_reply(reply);
        chain
    }

    async fn at_text(&self, text: &str) -> RQResult<MessageReceipt> {
        self.send_message_to_source(self.make_at_chain().await.append(text.parse_text()))
            .await
    }

    async fn reply_text(&self, text: &str) -> RQResult<MessageReceipt> {
        self.send_message_to_source(
            self
                .make_reply_chain().await
                .append(text.parse_text())).await
    }
    async fn at<S: Into<ricqMessageChain> + Send + Sync>(&self, message: S) -> RQResult<MessageReceipt> {
        let mut chain = self.make_at_chain().await;
        chain.push(message.into().0);
        self.send_message_to_source(
            chain).await
    }

    async fn reply<S: Into<ricqMessageChain> + Send + Sync>(&self, message: S) -> RQResult<MessageReceipt> {
        let mut chain = self.make_reply_chain().await;
        chain.push(message.into().0);
        self.send_message_to_source(
            chain).await
    }
    async fn send(&self, text: &str) -> RQResult<MessageReceipt> {
        self.send_message_to_source(
            text.parse_message_chain()).await
    }
}


#[async_trait]
impl CanReply for MessageEvent {
    async fn make_at_chain(&self) -> ricqMessageChain {
        match self {
            MessageEvent::GroupMessage(event) => {
                event.make_at_chain().await
            }
            MessageEvent::FriendMessage(event) => {
                event.make_at_chain().await
            }
            MessageEvent::GroupTempMessage(event) => {
                event.make_at_chain().await
            }
        }
    }

    async fn make_reply_chain(&self) -> ricqMessageChain {
        match self {
            MessageEvent::GroupMessage(event) => {
                event.make_reply_chain().await
            }
            MessageEvent::FriendMessage(event) => {
                event.make_reply_chain().await
            }
            MessageEvent::GroupTempMessage(event) => {
                event.make_reply_chain().await
            }
        }
    }

    async fn at_text(&self, text: &str) -> RQResult<MessageReceipt> {
        self.send_message_to_source(self.make_at_chain().await.append(text.parse_text()))
            .await
    }

    async fn reply_text(&self, text: &str) -> RQResult<MessageReceipt> {
        self.send_message_to_source(
            self
                .make_reply_chain().await
                .append(text.parse_text())).await
    }

    async fn at<S: Into<ricqMessageChain> + Send + Sync>(&self, message: S) -> RQResult<MessageReceipt> {
        let mut chain = self.make_at_chain().await;
        chain.push(message.into().0);
        self.send_message_to_source(
            chain).await
    }

    async fn reply<S: Into<ricqMessageChain> + Send + Sync>(&self, message: S) -> RQResult<MessageReceipt> {
        let mut chain = self.make_reply_chain().await;
        chain.push(message.into().0);
        self.send_message_to_source(
            chain).await
    }
    async fn send(&self, text: &str) -> RQResult<MessageReceipt> {
        self.send_message_to_source(
            text.parse_message_chain()).await
    }
}

//框架已有命令匹配, 弃用
//
// #[async_trait]
// pub(crate) trait MsgRegexTrait {
//     async fn on_regex(&self,cmd: &[&str]) -> bool;
//     async fn on_regex_msg(&self,cmd: &[&str]) -> (bool,Vec<String>);
//     async fn on_regex_msg_all(&self, cmd: &[&str])  -> (bool,Vec<String>, String) ;
//     async fn on_regex_not_prefix(&self, cmd: &[&str]) -> bool;
//     async fn on_regex_msg_not_prefix(&self, cmd: &[&str]) -> (bool, Vec<String>);
//     async fn on_regex_msg_all_not_prefix(&self, cmd: &[&str]) -> (bool, Vec<String>, String) ;
// }
// async fn _on_regex(content:&str,cmd: &[&str],prefix:Option<()>) -> bool{
//     // 将设置里的前辍和cmd合并 并返回RegexSet
//     let regex_set = match prefix {
//         None => {
//             RegexSet::new(cmd).unwrap()
//         }
//         Some(_) => {
//             RegexSet::new(cmd
//                 .iter()
//                 .zip(CONTEXT.config.bot_config.command_prefix.iter())
//                 .map(|(k, v)| {
//                     // 将指令内的空格全部替换为空格的正则表达式
//                     format!("^{}{}$", v, k.replace(" ","[\\s]+"))
//                 })
//                 .collect::<Vec<_>>()).unwrap()
//         }
//     };
//     regex_set.is_match(content.trim_end_matches(" "))
// }
// async fn _on_regex_msg(content:&str,cmd: &[&str],prefix:Option<()>) -> (bool,Vec<String>){
//     (
//         _on_regex(content,cmd,prefix).await,
//         content
//             .trim_end_matches(" ")
//             .split_whitespace()
//             .map(|x|x.to_string())
//             .collect::<Vec<_>>()
//     )
// }
// async fn _on_regex_msg_all(content:&str,cmd: &[&str],prefix:Option<()>)  -> (bool,Vec<String>, String) {
//     let mut _content = String::new();
//     let (b, mut array) = _on_regex_msg(content, cmd, prefix).await;
//
//     for (i,str) in array.iter().enumerate() {
//         for x in cmd {
//            if i >= x.replace("(.*)","").split_whitespace().count(){
//                _content.push_str(str);
//                _content.push_str(" ");
//            }
//         }
//     }
//     (b,array,_content.trim_end_matches(" ").to_string())
// }
//
// #[async_trait]
// impl MsgRegexTrait for MessageEvent {
//     async fn on_regex(&self,cmd: &[&str]) -> bool {
//         match self {
//             MessageEvent::GroupMessage(event) =>  event.on_regex(cmd).await,
//             MessageEvent::FriendMessage(event) => event.on_regex(cmd).await,
//             MessageEvent::GroupTempMessage(event) => event.on_regex(cmd).await,
//         }
//     }
//     /// 返回 bool和以空格间隔的数组
//     async fn on_regex_msg(&self, cmd: &[&str]) -> (bool, Vec<String>) {
//         match self {
//             MessageEvent::GroupMessage(event) =>  event.on_regex_msg(cmd).await,
//             MessageEvent::FriendMessage(event) => event.on_regex_msg(cmd).await,
//             MessageEvent::GroupTempMessage(event) => event.on_regex_msg(cmd).await,
//         }
//     }
//
//     /// 返回 bool 和 以空格间隔的数组 和 指令后面的字符串
//     async fn on_regex_msg_all(&self, cmd: &[&str]) -> (bool,Vec<String>, String) {
//         match self {
//             MessageEvent::GroupMessage(event) =>  event.on_regex_msg_all(cmd).await,
//             MessageEvent::FriendMessage(event) => event.on_regex_msg_all(cmd).await,
//             MessageEvent::GroupTempMessage(event) => event.on_regex_msg_all(cmd).await,
//         }
//     }
//
//     async fn on_regex_not_prefix(&self, cmd: &[&str]) -> bool {
//         match self {
//             MessageEvent::GroupMessage(event) =>  event.on_regex(cmd).await,
//             MessageEvent::FriendMessage(event) => event.on_regex(cmd).await,
//             MessageEvent::GroupTempMessage(event) => event.on_regex(cmd).await,
//         }
//     }
//
//     async fn on_regex_msg_not_prefix(&self, cmd: &[&str]) -> (bool, Vec<String>) {
//         match self {
//             MessageEvent::GroupMessage(event) =>  event.on_regex_msg_not_prefix(cmd).await,
//             MessageEvent::FriendMessage(event) => event.on_regex_msg_not_prefix(cmd).await,
//             MessageEvent::GroupTempMessage(event) => event.on_regex_msg_not_prefix(cmd).await,
//         }
//     }
//
//     async fn on_regex_msg_all_not_prefix(&self, cmd: &[&str]) -> (bool, Vec<String>, String) {
//         match self {
//             MessageEvent::GroupMessage(event) =>  event.on_regex_msg_all_not_prefix(cmd).await,
//             MessageEvent::FriendMessage(event) => event.on_regex_msg_all_not_prefix(cmd).await,
//             MessageEvent::GroupTempMessage(event) => event.on_regex_msg_all_not_prefix(cmd).await,
//         }
//     }
// }
// #[async_trait]
// impl MsgRegexTrait for GroupMessageEvent {
//     async fn on_regex(&self, cmd: &[&str]) -> bool {
//         _on_regex(self.message_content().as_str(),cmd,Some(())).await
//     }
//
//     async fn on_regex_msg(&self, cmd: &[&str]) -> (bool, Vec<String>) {
//         _on_regex_msg(self.message_content().as_str(),cmd,Some(())).await
//     }
//
//     async fn on_regex_msg_all(&self, cmd: &[&str]) -> (bool, Vec<String>, String) {
//         _on_regex_msg_all(self.message_content().as_str(),cmd,Some(())).await
//     }
//
//     async fn on_regex_not_prefix(&self, cmd: &[&str]) -> bool {
//         _on_regex(self.message_content().as_str(),cmd,None).await
//     }
//
//     async fn on_regex_msg_not_prefix(&self, cmd: &[&str]) -> (bool, Vec<String>) {
//         _on_regex_msg(self.message_content().as_str(),cmd,None).await
//     }
//
//     async fn on_regex_msg_all_not_prefix(&self, cmd: &[&str]) -> (bool, Vec<String>, String) {
//         _on_regex_msg_all(self.message_content().as_str(),cmd,None).await
//     }
// }
// #[async_trait]
// impl MsgRegexTrait for FriendMessageEvent {
//     async fn on_regex(&self, cmd: &[&str]) -> bool {
//         _on_regex(self.message_content().as_str(),cmd,Some(())).await
//     }
//
//     async fn on_regex_msg(&self, cmd: &[&str]) -> (bool, Vec<String>) {
//         _on_regex_msg(self.message_content().as_str(),cmd,Some(())).await
//     }
//
//     async fn on_regex_msg_all(&self, cmd: &[&str]) -> (bool, Vec<String>, String) {
//         _on_regex_msg_all(self.message_content().as_str(),cmd,Some(())).await
//     }
//
//     async fn on_regex_not_prefix(&self, cmd: &[&str]) -> bool {
//         _on_regex(self.message_content().as_str(),cmd,None).await
//     }
//
//     async fn on_regex_msg_not_prefix(&self, cmd: &[&str]) -> (bool, Vec<String>) {
//         _on_regex_msg(self.message_content().as_str(),cmd,None).await
//     }
//
//     async fn on_regex_msg_all_not_prefix(&self, cmd: &[&str]) -> (bool, Vec<String>, String) {
//         _on_regex_msg_all(self.message_content().as_str(),cmd,None).await
//     }
// }
// #[async_trait]
// impl MsgRegexTrait for GroupTempMessageEvent {
//     async fn on_regex(&self, cmd: &[&str]) -> bool {
//         _on_regex(self.message_content().as_str(),cmd,Some(())).await
//     }
//
//     async fn on_regex_msg(&self, cmd: &[&str]) -> (bool, Vec<String>) {
//         _on_regex_msg(self.message_content().as_str(),cmd,Some(())).await
//     }
//
//     async fn on_regex_msg_all(&self, cmd: &[&str]) -> (bool, Vec<String>, String) {
//         _on_regex_msg_all(self.message_content().as_str(),cmd,Some(())).await
//     }
//
//     async fn on_regex_not_prefix(&self, cmd: &[&str]) -> bool {
//         _on_regex(self.message_content().as_str(),cmd,None).await
//     }
//
//     async fn on_regex_msg_not_prefix(&self, cmd: &[&str]) -> (bool, Vec<String>) {
//         _on_regex_msg(self.message_content().as_str(),cmd,None).await
//     }
//
//     async fn on_regex_msg_all_not_prefix(&self, cmd: &[&str]) -> (bool, Vec<String>, String) {
//         _on_regex_msg_all(self.message_content().as_str(),cmd,None).await
//     }
// }

#[async_trait]
pub(crate) trait OrderPermissionTrait {
    async fn is_admin(&self) -> bool;
    async fn is_super_admin(&self) -> bool;
}

#[async_trait]
impl OrderPermissionTrait for MessageEvent {
    async fn is_admin(&self) -> bool {
        match self {
            MessageEvent::GroupMessage(event) => event.is_admin().await,
            MessageEvent::GroupTempMessage(event) => event.is_admin().await,
            _ => false
        }
    }

    async fn is_super_admin(&self) -> bool {
        match self {
            MessageEvent::GroupMessage(event) => event.is_super_admin().await,
            MessageEvent::FriendMessage(event) => event.is_super_admin().await,
            MessageEvent::GroupTempMessage(event) => event.is_super_admin().await,
        }
    }
}

#[async_trait]
impl OrderPermissionTrait for GroupMessageEvent {
    async fn is_admin(&self) -> bool {
        let mut is = false;
        for x in CONTEXT.config.bot_config.super_admin.iter() {
            if self.inner.from_uin == x.parse::<i64>().unwrap_or_default() {
                return true;
            }
        }
        self.client
            .get_group_admin_list(self.inner.group_code)
            .await
            .unwrap()
            .iter().map(|(user_id, member)| {
            if self.inner.from_uin == *user_id {
                return match member {
                    GroupMemberPermission::Owner => is = true,
                    GroupMemberPermission::Administrator => is = true,
                    GroupMemberPermission::Member => is = false,
                }
            }
        });
        is
    }

    async fn is_super_admin(&self) -> bool {
        for x in CONTEXT.config.bot_config.super_admin.iter() {
            if self.inner.from_uin == x.parse::<i64>().unwrap_or_default() {
                return true;
            }
        }
        false
    }
}

#[async_trait]
impl OrderPermissionTrait for FriendMessageEvent {
    async fn is_admin(&self) -> bool {
        false
    }

    async fn is_super_admin(&self) -> bool {
        for x in CONTEXT.config.bot_config.super_admin.iter() {
            if self.inner.from_uin == x.parse::<i64>().unwrap_or_default() {
                return true;
            }
        }
        false
    }
}

#[async_trait]
impl OrderPermissionTrait for GroupTempMessageEvent {
    async fn is_admin(&self) -> bool {
        let mut is = false;
        for x in CONTEXT.config.bot_config.super_admin.iter() {
            if self.inner.from_uin == x.parse::<i64>().unwrap_or_default() {
                return true;
            }
        }
        self.client
            .get_group_admin_list(self.inner.group_code)
            .await
            .unwrap()
            .iter().map(|(user_id, member)| {
            if self.inner.from_uin == *user_id {
                return match member {
                    GroupMemberPermission::Owner => is = true,
                    GroupMemberPermission::Administrator => is = true,
                    GroupMemberPermission::Member => is = false,
                }
            }
        });
        is
    }

    async fn is_super_admin(&self) -> bool {
        for x in CONTEXT.config.bot_config.super_admin.iter() {
            if self.inner.from_uin == x.parse::<i64>().unwrap_or_default() {
                return true;
            }
        }
        false
    }
}
