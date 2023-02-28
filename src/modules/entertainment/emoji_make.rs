use proc_qq::{event, GroupMessageEvent, MessageContentTrait, MessageEvent, MessageSendToSourceTrait, Module, module};
use rbatis::dark_std::err;
use crate::BotResult;
use crate::msg_util::MessageChain;
use crate::utils::image::emoji_make_util::long1;
use crate::utils::Reg;


static ID: &'static str = "emoji_make";
static NAME: &'static str = "表情制作";

pub struct EmojiMakeHelp {
    pub mod_name: String,
    pub help_text: Vec<String>,
}

impl Default for EmojiMakeHelp {
    fn default() -> Self {
        EmojiMakeHelp {
            mod_name: "签到".to_string(),
            help_text: vec![
                "emoji_make help",
                "code       name      图片本体",
                "long1      ↑       ",
            ].iter().map(|str| str.to_string()).collect::<Vec<_>>(),
        }
    }
}

pub(crate) fn module() -> Module {
    module!(
        ID,
        NAME,
        emoji_make_long,
    )
}

#[event]
pub async fn emoji_make_long(event: &MessageEvent) -> anyhow::Result<bool> {
    let content = event.message_content();
    let (b, msg_array) = Reg::ex_msg(&content, &["emoji[\\s]+(.*)"], Some(&[Reg::All]));
    if b {
        if msg_array[1].eq("long1") {
            let result = long1(&msg_array[2]);
            return match result {
                Ok(data) => {
                    event.send_message_to_source(MessageChain::new().image_vec(data, &event).await.ok()).await?;
                    Ok(true)
                }
                Err(err) => {
                    event.send_message_to_source(err.to_msg()).await?;
                    Ok(true)
                }
            }
        }
    }
    Ok(false)
}