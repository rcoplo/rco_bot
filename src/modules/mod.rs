use std::collections::HashMap;
use once_cell::sync::Lazy;
use proc_qq::{event, MessageChainParseTrait, MessageChainPointTrait, MessageContentTrait, MessageEvent, MessageSendToSourceTrait, Module, module};
use std::sync::Arc;
use crate::BotResult;
use crate::modules::entertainment::emoji_make::EmojiMakeHelp;
use crate::modules::entertainment::sign::SignHelp;
use crate::modules::ett::EttHelp;
use crate::modules::h::setu::SetuHelp;
use crate::modules::tools::mc_server_status_get::McServerStatusGetHelp;
use crate::msg_util::{MessageChain, text};
use crate::utils::image::help_image_util::help_module_image;
use crate::utils::Reg;

mod forwarding;
mod h;
mod tools;
mod osu;
mod entertainment;
mod ett;

static MODULES: Lazy<Arc<Vec<Module>>> =
    Lazy::new(|| Arc::new(vec![
        h::setu::module(),
        forwarding::bili::module(),
        entertainment::sign::module(),
        entertainment::emoji_make::module(),
        help_module(),
        ett::ett_user_info::module(),
        tools::mc_server_status_get::module(),
    ]));
pub fn all_modules() -> Arc<Vec<Module>> {
    MODULES.clone()
}

static MODULES_HELP:Lazy<Help> = Lazy::new(||Help::default());

struct Help{
    help:HashMap<String,HelpEnum>
}

enum HelpEnum {
    Setu(SetuHelp),
    Sign(SignHelp),
    Ett(EttHelp),
    Emoji(EmojiMakeHelp),
    McStatus(McServerStatusGetHelp),
}
impl Default for Help {
    fn default() -> Self {
        let mut map = HashMap::new();
        let setu = SetuHelp::default();
        let sign = SignHelp::default();
        let ett = EttHelp::default();
        let emoji = EmojiMakeHelp::default();
        let mc_status = McServerStatusGetHelp::default();
        map.insert(setu.mod_name.clone(), HelpEnum::Setu(setu));
        map.insert(sign.mod_name.clone(), HelpEnum::Sign(sign));
        map.insert(ett.mod_name.clone(), HelpEnum::Ett(ett));
        map.insert(emoji.mod_name.clone(), HelpEnum::Emoji(emoji));
        map.insert(mc_status.mod_name.clone(), HelpEnum::McStatus(mc_status));
        Self {
            help: map,
        }
    }
}
#[event]
async fn help(event: &MessageEvent) -> anyhow::Result<bool> {
    let content = event.message_content();
    if Reg::ex(content.as_str(), &["help $"], Some(&[Reg::All])) {
        event.send_message_to_source(text("暂未完成")).await?;
        return Ok(true);
    } else {
        let (bool, msg_array) = Reg::ex_msg(content.as_str(), &["help[\\s]+(.*)"], Some(&[Reg::All]));
        if bool {
            let help = MODULES_HELP.help.get(msg_array[1].as_str());
            return if help.is_some() {
                let result = match help.unwrap() {
                    HelpEnum::Setu(setu) => {
                        help_module_image(&setu.help_text)
                    }
                    HelpEnum::Sign(sign) => {
                        help_module_image(&sign.help_text)
                    }
                    HelpEnum::Ett(ett) => {
                        help_module_image(&ett.help_text)
                    }
                    HelpEnum::Emoji(emoji) => {
                        help_module_image(&emoji.help_text)
                    }
                    HelpEnum::McStatus(mc_status) => {
                        help_module_image(&mc_status.help_text)
                    }
                };
                match result {
                    Ok(vec) => {
                        let chain = MessageChain::new()
                            .reply(&event)
                            .image_vec(vec, &event)
                            .await
                            .ok();
                        event.send_message_to_source(chain).await?;
                        Ok(true)
                    }
                    Err(err) => {
                        event.send_message_to_source(err.to_msg()).await?;
                        Ok(true)
                    }
                }
            } else {
                event.send_message_to_source("该功能名称不存在喵...".parse_message_chain()).await?;
                Ok(true)
            }

        }
    }
    Ok(false)
}

pub(crate) fn help_module() -> Module {
    module!("help", "帮助模块", help,)
}