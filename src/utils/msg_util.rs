use std::fmt::Debug;
use std::io::Read;
use async_trait::async_trait;
use proc_qq::{ClientTrait, FriendMessageEvent, GroupMessageEvent, GroupTempMessageEvent, MessageChainAppendTrait, MessageChainParseTrait, MessageChainPointTrait, MessageEvent, MessageSendToSourceTrait, TextEleParseTrait, UploadImage};
use proc_qq::re_exports::bytes::Bytes;
use proc_qq::re_exports::ricq::RQResult;
use proc_qq::re_exports::ricq_core::msg::{MessageChain as ricqMessageChain, MessageChainBuilder, MessageElem};
use proc_qq::re_exports::ricq_core::msg::elem::{At, Text, Face, Reply};
use proc_qq::re_exports::ricq_core::RQError;
use proc_qq::re_exports::ricq_core::structs::{ForwardMessage, ForwardNode, MessageNode, MessageReceipt};
use rbatis::dark_std::err;
use reqwest::Response;
use tokio::io::AsyncReadExt;
use tracing::event;
use tracing::instrument::WithSubscriber;
use crate::utils::http_util::http_get;

#[derive(Debug, Clone)]
pub struct MessageChain{
    inner:ricqMessageChain
}

impl MessageChain {
    pub fn new() -> Self {
        Self {
            inner: MessageChainBuilder::new().build(),
        }
    }

    pub fn text<T: AsRef<str>+ TextEleParseTrait>(&mut self, text: T) -> &mut MessageChain {
        self.inner.push(<T as Into<T>>::into(text).parse_text());
        self
    }

    pub async fn image(&mut self, url: &String, event: &MessageEvent) -> &mut MessageChain {
        tracing::debug!("image_url: {}", url);
        let bytes = reqwest::get(url)
            .await.unwrap().error_for_status();
        match bytes {
            Ok(bytes) => {
                let bytes = bytes.bytes().await.unwrap();
                let upload_res = event.upload_image_to_source(bytes.to_vec()).await;
                match upload_res {
                    Ok(image) => {
                        self.inner.push(image);
                    },
                    Err(err) => {
                        self.inner.push(Text::new(format!("上传图片失败喵... \n RQError: {} ",err)));
                    }
                }
                self
            }
            Err(err) => {
                self.inner.push(Text::new(format!("上传图片失败喵... \n Error: {} ",err)));
                self
            }
        }

    }
    pub async fn image_vec(&mut self, data: Vec<u8>, event: &MessageEvent) -> &mut MessageChain {
        let upload_res = event.upload_image_to_source(data).await;
        match upload_res {
            Ok(image) => {
                self.inner.push(image);
            },
            Err(err) => {
                self.inner.push(format!("上传图片失败喵... \nRQError: {}",err).parse_text());
            },
        };
        self
    }
    pub async fn image_path(&mut self, data: &String, event: &MessageEvent) -> &mut MessageChain {
        let mut f = tokio::fs::File::open(data).await.map_err(RQError::IO).unwrap();
        let mut b = vec![];
        f.read_to_end(&mut b).await.map_err(RQError::IO).unwrap();
        let upload_res = event.upload_image_to_source(b).await;
        match upload_res {
            Ok(image) => {
                self.inner.push(image);
            },
            Err(err) => {
                self.inner.push(format!("上传图片失败喵... \nRQError: {}",err).parse_text())
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

    pub fn reply(&mut self,event:&MessageEvent) -> &mut MessageChain {
        let (reply_seq,sender,time,elements) = match event {
            MessageEvent::GroupMessage(event) => {
                (event.inner.seqs[0],event.inner.from_uin,event.inner.time,event.inner.elements.clone())
            }
            MessageEvent::FriendMessage(event) => {
                (event.inner.seqs[0],event.inner.from_uin,event.inner.time,event.inner.elements.clone())
            }
            MessageEvent::GroupTempMessage(event) => {
                (event.inner.seqs[0],event.inner.from_uin,event.inner.time,event.inner.elements.clone())
            }
        };
        let reply = Reply {
            reply_seq,
            sender,
            time,
            elements
        };
        self.inner.with_reply(reply);
        self
    }


    pub fn ok(&self) -> ricqMessageChain{
        self.inner.clone()
    }
}

pub fn text<T: AsRef<str>+ TextEleParseTrait+ MessageChainParseTrait>( text: T) -> ricqMessageChain {
    <T as Into<T>>::into(text).parse_message_chain()
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
    async fn at_text(&self, text: &str) -> RQResult<()>;
    async fn reply_text(&self, text: &str) -> RQResult<()>;
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

    async fn at_text(&self, text: &str) -> RQResult<()> {
        self.send_message_to_source(self.make_at_chain().await.append(text.parse_text()))
            .await?;
        Ok(())
    }

    async fn reply_text(&self, text: &str) -> RQResult<()> {
        self.send_message_to_source(
            self
                .make_reply_chain().await
                .append(text.parse_text())).await?;
       Ok(())
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

    async fn at_text(&self, text: &str) -> RQResult<()> {
        self.send_message_to_source(self.make_at_chain().await.append(text.parse_text()))
            .await?;
        Ok(())
    }

    async fn reply_text(&self, text: &str) -> RQResult<()> {
        self.send_message_to_source(
            self
                .make_reply_chain().await
                .append(text.parse_text())).await?;
        Ok(())
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

    async fn at_text(&self, text: &str) -> RQResult<()> {
        self.send_message_to_source(self.make_at_chain().await.append(text.parse_text()))
            .await?;
        Ok(())
    }

    async fn reply_text(&self, text: &str) -> RQResult<()> {
        self.send_message_to_source(
            self
                .make_reply_chain().await
                .append(text.parse_text())).await?;
        Ok(())
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

    async fn at_text(&self, text: &str) -> RQResult<()> {
        self.send_message_to_source(self.make_at_chain().await.append(text.parse_text()))
            .await?;
        Ok(())
    }

    async fn reply_text(&self, text: &str) -> RQResult<()> {
        self.send_message_to_source(
            self
                .make_reply_chain().await
                .append(text.parse_text())).await?;
        Ok(())
    }

}

