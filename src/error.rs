use std::{fmt, io};
use std::error::Error as StdError;
use std::fmt::Display;
use proc_qq::MessageChainParseTrait;
use proc_qq::re_exports::ricq::device::random_string;
use proc_qq::re_exports::ricq_core::msg::MessageChain;
use proc_qq::re_exports::serde_json;
use rbatis::dark_std::err;


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

macro_rules! bot_error {
    ($error:ty) => {
        impl From<$error> for BotError {
            fn from(arg: $error) -> Self {
                BotError::MsgChain(arg.to_string().parse_message_chain())
            }
        }
    };
}
bot_error!(&str);
bot_error!(String);
bot_error!(std::io::Error);
bot_error!(reqwest::Error);
bot_error!(serde_json::Error);
bot_error!(serde_yaml::Error);
bot_error!(regex::Error);
bot_error!(rbs::Error);
bot_error!(rbatis::Error);
bot_error!(rbdc_sqlite::error::SqliteError);
bot_error!(etternaonline_api::Error);
bot_error!(og_image_writer::Error);
bot_error!(proc_qq::re_exports::ricq::RQError);


impl Clone for BotError {
    fn clone(&self) -> Self {
        BotError::from(self.to_string())
    }

    fn clone_from(&mut self, source: &Self) {
        *self = Self::from(source.to_string());
    }
}
