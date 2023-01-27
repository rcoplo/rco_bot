use std::collections::LinkedList;
use proc_qq::re_exports::async_trait::async_trait;
use proc_qq::re_exports::ricq::msg::MessageChain;
use proc_qq::re_exports::ricq::RQResult;
use proc_qq::re_exports::ricq_core::msg::elem::At;
use proc_qq::{FriendMessageEvent, GroupMessageEvent, MessageChainAppendTrait, MessageChainParseTrait, MessageEvent, MessageSendToSourceTrait, TextEleParseTrait};
use proc_qq::re_exports::bytes;
use proc_qq::re_exports::image::EncodableLayout;
use regex::RegexSet;
use crate::CONTEXT;


mod local;
pub mod msg_util;
pub mod http_util;
pub mod chrome_util;

pub enum Reg{
    All = 0, //全部前缀
    Sharp = 1, //#
    Dollar = 2 ,//$
    And  = 3, //&
    Em = 4, //英文 !
    EmC = 5, //中文 ！
    Qm = 6, //英文 ?
    QmC = 7, //中文 ？
    Sd = 8, //英文 ～
    SdC = 9, //中文 ~
    Fs = 10, // /
}
impl Reg {
    
    pub fn is_super_admin(user_id:&i64) -> bool {
        let exp = RegexSet::new(&CONTEXT.config.super_admin).unwrap();
        exp.is_match(format!("{}",user_id).as_str())
    }

    pub fn is_bot_name(name:&String) -> bool {
        let exp = RegexSet::new(&CONTEXT.config.bot_name).unwrap();
        exp.is_match(format!("{}",name).as_str())
    }

    pub fn ex(content:&String,command:&[&str],prefix:Option<&[Reg]>) -> bool{
        match prefix {
            None => {
                let exp = RegexSet::new(command).unwrap();
                exp.is_match(content.as_str())
            },
            Some(prefix) => {
                //把前缀添加到命令前面
                let mut vec = Reg::literal_conversion(command,prefix);
                let exp = RegexSet::new(&vec).unwrap();
                exp.is_match(content.as_str())
            }
        }

    }
    //好想优化一下啊,但是不会
    fn literal_conversion(command:&[&str],prefix:&[Reg]) -> Vec<String> {
        let mut vec = vec![];
        for x in command.iter() {
            for reg in prefix.iter() {
                match reg {
                    Reg::All => {
                        vec.push(format!("#{}",x));
                        vec.push(format!("${}",x));
                        vec.push(format!("&{}",x));
                        vec.push(format!("!{}",x));
                        vec.push(format!("！{}",x));
                        vec.push(format!(r"\?{}",x));
                        vec.push(format!("？{}",x));
                        vec.push(format!("～{}",x));
                        vec.push(format!("~{}",x));
                        vec.push(format!("/{}",x));
                    }
                    Reg::Sharp => {
                        vec.push(format!("#{}",x));
                    }
                    Reg::Dollar => {
                        vec.push(format!("${}",x));
                    }
                    Reg::And => {
                        vec.push(format!("&{}",x));
                    }
                    Reg::Em => {
                        vec.push(format!("!{}",x));
                    }
                    Reg::EmC => {
                        vec.push(format!("！{}",x));
                    }
                    Reg::Qm => {
                        vec.push(format!(r"\?{}",x));
                    }
                    Reg::QmC => {
                        vec.push(format!("？{}",x));
                    }
                    Reg::Sd => {
                        vec.push(format!("～{}",x));
                    }
                    Reg::SdC => {
                        vec.push(format!("~{}",x));
                    }
                    Reg::Fs => {
                        vec.push(format!("/{}",x));
                    }
                }
            }
        }

        vec
    }
}



#[async_trait]
pub(crate) trait CanReply {
    async fn make_reply_chain(&self) -> MessageChain;
    async fn reply_text(&self, text: &str) -> RQResult<()>;
    async fn reply_raw_text(&self, text: &str) -> RQResult<()>;
}

#[async_trait]
impl CanReply for GroupMessageEvent {
    async fn make_reply_chain(&self) -> MessageChain {
        let mut at = At::new(self.inner.from_uin);
        at.display = format!("@{}", self.inner.group_card);
        MessageChain::default()
            .append(at)
            .append("\n\n".parse_text())
    }

    async fn reply_text(&self, text: &str) -> RQResult<()> {
        self.send_message_to_source(self.make_reply_chain().await.append(text.parse_text()))
            .await?;
        RQResult::Ok(())
    }

    async fn reply_raw_text(&self, text: &str) -> RQResult<()> {
        self.send_message_to_source(text.parse_message_chain())
            .await?;
        RQResult::Ok(())
    }
}

#[async_trait]
impl CanReply for MessageEvent {
    async fn make_reply_chain(&self) -> MessageChain {
        match self {
            MessageEvent::GroupMessage(group_message) => group_message.make_reply_chain().await,
            _ => MessageChain::default(),
        }
    }
    async fn reply_text(&self, text: &str) -> RQResult<()> {
        self.send_message_to_source(self.make_reply_chain().await.append(text.parse_text()))
            .await?;
        RQResult::Ok(())
    }

    async fn reply_raw_text(&self, text: &str) -> RQResult<()> {
        self.send_message_to_source(text.parse_message_chain())
            .await?;
        RQResult::Ok(())
    }
}

