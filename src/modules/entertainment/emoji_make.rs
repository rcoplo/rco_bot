use proc_qq::{event, MessageEvent, MessageSendToSourceTrait, module, Module};
use crate::BotError;
use crate::msg_util::{CanReply, MessageChain};
use crate::utils::image::emoji_make_util::long1_emoji_make_image;


pub(crate) fn module() -> Module {
    module!("emoji",
        "emoji",
        emoji_make_long
    )
}

#[event(bot_command = "/emoji {image_type} {string}")]
pub async fn emoji_make_long(
    event: &MessageEvent,
    image_type: Option<String>,
    string: Option<String>,
) -> anyhow::Result<bool> {
    if let Some(image_type) = image_type {
        if let Some(string) = string {
            let req = match image_type.as_str() {
                "long1" => Ok(long1_emoji_make_image(string.as_str())?),
                _ => Err(BotError::from("没有这个emoji合成喵...")),
            };
            return match req {
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
            return Ok(true);
        }
    } else {
        event.reply(MessageChain::new()
            .text("可用 {emoji}: \n")
            .text(">    long1\n")
            .text("指令: /emoji {emoji} {string}")
            .build())
            .await?;
        return Ok(true);
    }
}
