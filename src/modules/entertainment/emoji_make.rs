use proc_qq::{event, MessageEvent, MessageSendToSourceTrait, module, Module};
use proc_qq::re_exports::anyhow;

use crate::BotError;
use crate::image::emoji_make_util::{good_news, long1_emoji_make_image};
use crate::msg_util::{CanReply, MessageChain};

pub(crate) fn module() -> Module {
    module!("emoji",
        "emoji",
        emoji_make_long
    )
}

#[event(bot_command = "/emoji {image_type} {str}")]
pub async fn emoji_make_long(
    event: &MessageEvent,
    image_type: Option<String>,
    str: Option<String>,
) -> anyhow::Result<bool> {
    return if let Some(image_type) = image_type {
        if let Some(string) = str {
            let req = match image_type.as_str() {
                "long1" => long1_emoji_make_image(string.as_str()),
                "bad" => todo!(),
                "good" => good_news(string.as_str()),
                _ => Err(BotError::from("没有这个emoji合成喵...")),
            };
            match req {
                Ok(data) => {
                    event
                        .send_message_to_source(MessageChain::new().image_bytes(data, &event).await.build())
                        .await?;
                    Ok(true)
                }
                Err(err) => {
                    event.send_message_to_source(err.to_msg()).await?;
                    Ok(true)
                }
            }
        } else {
            event.at_text("后面必须要有字符串喵!")
                .await?;
            Ok(true)
        }
    } else {
        event.reply(MessageChain::new()
            .text("可用 {emoji}: \n")
            .text(">    long1\n")
            .text("指令: /emoji {emoji} {string}")
            .build())
            .await?;
        Ok(true)
    }
}
