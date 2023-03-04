use std::collections::{HashMap, LinkedList};
use std::fmt::Formatter;
use proc_qq::re_exports::async_trait::async_trait;
use proc_qq::re_exports::ricq::msg::MessageChain;
use proc_qq::re_exports::ricq::RQResult;
use proc_qq::re_exports::ricq_core::msg::elem::{At, Reply};
use proc_qq::{FriendMessageEvent, GroupMessageEvent, MessageChainAppendTrait, MessageChainParseTrait, MessageEvent, MessageSendToSourceTrait, TextEleParseTrait};
use proc_qq::re_exports::bytes;
use proc_qq::re_exports::image::EncodableLayout;
use regex::RegexSet;
use serde::{Deserialize, Deserializer};
use crate::CONTEXT;
use crate::msg_util::text;
use crate::utils::Reg::Sharp;


pub mod msg_util;
pub mod http_util;
pub mod chrome_util;
pub mod file_util;
pub mod image;

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

    pub fn ex(content: &str, command: &[&str], prefix: Option<&[Reg]>) -> bool {
        match prefix {
            None => {
                let exp = RegexSet::new(command).unwrap();
                exp.is_match(content)
            },
            Some(prefix) => {
                //把前缀添加到命令前面

                let mut vec = Reg::command_assembly(command, prefix);
                let exp = RegexSet::new(&vec).unwrap();
                exp.is_match(content)
            }
        }
    }
    pub fn ex_msg(content: &str, command: &[&str], prefix: Option<&[Reg]>) -> (bool, Vec<String>) {
        let mut param = content
            .split_whitespace()
            .filter_map(|str| {
                if str.eq(" ") {
                    None
                } else {
                    Some(str.to_string())
                }
            })
            .collect::<Vec<_>>();
        match prefix {
            None => {
                let exp = RegexSet::new(command).unwrap();
                (exp.is_match(content), param)
            },
            Some(prefix) => {
                //把前缀添加到命令前面
                let mut vec = Reg::command_assembly(command,prefix);
                let exp = RegexSet::new(&vec).unwrap();
                (exp.is_match(content), param)
            }
        }
    }

    //好想优化一下啊,但是不会
    fn command_assembly(command:&[&str],prefix:&[Reg]) -> Vec<String> {
        command
            .iter()
            .zip(prefix.iter())
            .map(|(k,v)|{
                format!("^{}{}", v.to_string(), k)
            })
            .collect::<Vec<_>>()
    }
}

impl std::fmt::Display for Reg{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Reg::All => write!(f,r"([#]|[$]|[&]|[!]|[！]|[\?]|[？]|[～]|[~]|[/])"),
            Reg::Sharp => write!(f,"#"),
            Reg::Dollar => write!(f,"$"),
            Reg::And => write!(f,"&"),
            Reg::Em => write!(f,"!"),
            Reg::EmC => write!(f,"！"),
            Reg::Qm => write!(f,r"\?"),
            Reg::QmC => write!(f,"？"),
            Reg::Sd => write!(f,"～"),
            Reg::SdC => write!(f,"~"),
            Reg::Fs => write!(f,"/"),
        }
    }
}


