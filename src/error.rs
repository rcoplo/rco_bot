use std::{fmt, io};
use std::error::Error as StdError;
use std::fmt::Display;
use proc_qq::MessageChainParseTrait;
use proc_qq::re_exports::ricq_core::msg::MessageChain;


#[derive(Debug)]
pub enum BotError {
    MsgChain(MessageChain)
}

pub type BotResult<T> = Result<T, BotError>;


impl Display for BotError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BotError::MsgChain(error) => write!(f, "{}", error),
        }
    }
}

impl BotError {
    pub fn to_msg(&self) -> MessageChain {
        match self {
            BotError::MsgChain(err) => err.to_string().parse_message_chain()
        }
    }
}
impl StdError for BotError {}

impl From<io::Error> for BotError {
    #[inline]
    fn from(err: io::Error) -> Self {
        BotError::from(err.to_string())
    }
}

impl From<&str> for BotError {
    fn from(arg: &str) -> Self {
        return BotError::MsgChain(arg.to_string().parse_message_chain());
    }
}

impl From<String> for BotError {
    fn from(arg: String) -> Self {
        return BotError::MsgChain(arg.parse_message_chain());
    }
}

impl From<&dyn std::error::Error> for BotError {
    fn from(arg: &dyn std::error::Error) -> Self {
        return BotError::MsgChain(arg.to_string().parse_message_chain());
    }
}

impl From<BotError> for std::io::Error {
    fn from(arg: BotError) -> Self {
        arg.into()
    }
}

impl From<rbatis::Error> for BotError {
    fn from(arg: rbatis::Error) -> Self {
        BotError::MsgChain(arg.to_string().parse_message_chain())
    }
}

impl From<reqwest::Error> for BotError {
    fn from(arg: reqwest::Error) -> Self {
        BotError::MsgChain(arg.to_string().parse_message_chain())
    }
}

impl From<serde_json::Error> for BotError {
    fn from(arg: serde_json::Error) -> Self {
        BotError::MsgChain(arg.to_string().parse_message_chain())
    }
}

impl From<og_image_writer::Error> for BotError {
    fn from(arg: og_image_writer::Error) -> Self {
        BotError::MsgChain(arg.to_string().parse_message_chain())
    }
}

impl From<etternaonline_api::Error> for BotError {
    fn from(arg: etternaonline_api::Error) -> Self {
        BotError::MsgChain(arg.to_string().parse_message_chain())
    }
}

impl From<image::error::ImageError> for BotError {
    fn from(arg: image::error::ImageError) -> Self {
        BotError::MsgChain(arg.to_string().parse_message_chain())
    }
}

impl From<rosu_v2::error::ApiError> for BotError {
    fn from(arg: rosu_v2::error::ApiError) -> Self {
        BotError::MsgChain(arg.to_string().parse_message_chain())
    }
}

impl From<rosu_v2::error::OsuError> for BotError {
    fn from(arg: rosu_v2::error::OsuError) -> Self {
        BotError::MsgChain(arg.to_string().parse_message_chain())
    }
}

impl From<rosu_v2::error::ParsingError> for BotError {
    fn from(arg: rosu_v2::error::ParsingError) -> Self {
        BotError::MsgChain(arg.to_string().parse_message_chain())
    }
}

impl From<proc_qq::re_exports::ricq_core::RQError> for BotError {
    fn from(arg:proc_qq::re_exports::ricq_core::RQError) -> Self {
        BotError::MsgChain(arg.to_string().parse_message_chain())
    }
}

impl From<thirtyfour::error::WebDriverError> for BotError {
    fn from(arg: thirtyfour::error::WebDriverError) -> Self {
        BotError::MsgChain(arg.to_string().parse_message_chain())
    }
}

impl Clone for BotError {
    fn clone(&self) -> Self {
        BotError::from(self.to_string())
    }

    fn clone_from(&mut self, source: &Self) {
        *self = Self::from(source.to_string());
    }
}
