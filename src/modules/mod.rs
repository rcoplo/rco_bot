use std::collections::HashMap;
use once_cell::sync::Lazy;
use proc_qq::{event, MessageChainParseTrait, MessageChainPointTrait, MessageContentTrait, MessageEvent, MessageSendToSourceTrait, Module, module};
use std::sync::Arc;
use crate::BotResult;
use crate::modules::entertainment::sign::SignHelp;
use crate::modules::h::setu::SetuHelp;
use crate::msg_util::{MessageChain, text};
use crate::utils::image::help_image_util::help_module_image;
use crate::utils::Reg;

mod forwarding;
mod h;
mod tools;
mod osu;
mod entertainment;

static MODULES: Lazy<Arc<Vec<Module>>> =
    Lazy::new(|| Arc::new(vec![
        h::setu::module(),
        forwarding::bili::module(),
        entertainment::sign::module(),
        help_module(),
    ]));
pub fn all_modules() -> Arc<Vec<Module>> {
    MODULES.clone()
}

static MODULES_HELP:Lazy<Help> = Lazy::new(||Help::default());

struct Help{
    help:HashMap<String,HelpEnum>
}

enum HelpEnum{
    Setu(SetuHelp),
    Sign(SignHelp),
}
impl Default for Help {
    fn default() -> Self {
        let mut map = HashMap::new();
        let setu = SetuHelp::default();
        let sign = SignHelp::default();
        map.insert(&setu.mod_name,HelpEnum::Setu(setu));
        map.insert(&sign.mod_name,HelpEnum::Sign(sign));
        Self{
            help: map.iter().map(|(k, _)| { *k}).collect::<HashMap<String,HelpEnum>>(),
        }
    }
}
#[event]
async fn help(event: &MessageEvent) -> anyhow::Result<bool> {
    let content = event.message_content();
    if Reg::ex(&content, &["help $"], Some(&[Reg::All])) {
        event.send_message_to_source(text("暂未完成")).await?;
        return Ok(true);
    } else {
        let (bool,msg_array) = Reg::ex_msg(&content, &["help[\\s]+(.*)"], Some(&[Reg::All]));
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