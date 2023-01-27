use std::fmt::Debug;
use proc_qq::{MessageChainAppendTrait, MessageChainParseTrait, MessageEvent, MessageSendToSourceTrait, TextEleParseTrait, UploadImage};
use proc_qq::re_exports::ricq::RQResult;
use proc_qq::re_exports::ricq_core::msg::{MessageChain as ricqMessageChain, MessageChainBuilder, MessageElem};
use proc_qq::re_exports::ricq_core::msg::elem::{At, Text, Face, Reply};

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
        let bytes = reqwest::get(url.replace("i.pixiv.re","pixiv.rco.ink"))
            .await.unwrap()
            .error_for_status().unwrap()
            .bytes()
            .await.unwrap()
            .to_vec();
        let upload_res = event.upload_image_to_source(bytes).await;
        match upload_res {
            Ok(image) => self.inner.push(image),
            Err(_) => {self.inner.push(format!("上传图片失败喵...(url:{})",url).parse_text())}
        }
        self
    }
    pub async fn image_or(&mut self, data: &Vec<u8>, event: &MessageEvent) -> &mut MessageChain {
        let upload_res = event.upload_image_to_source(data.clone()).await;
        match upload_res {
            Ok(image) => self.inner.push(image),
            Err(_) => {self.inner.push(format!("上传图片失败喵...").parse_text())}
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

    pub fn reply(&mut self) -> &mut MessageChain {
        self.inner.with_reply(Reply::default());
        self
    }

    pub fn ok(&self) -> ricqMessageChain{
        self.inner.clone()
    }
}

pub fn text<T: AsRef<str>+ TextEleParseTrait+ MessageChainParseTrait>( text: T) -> ricqMessageChain {
    <T as Into<T>>::into(text).parse_message_chain()
}