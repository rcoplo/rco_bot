
use proc_qq::{ClientTrait, event, MessageChainAppendTrait, MessageChainParseTrait, MessageContentTrait, MessageEvent, MessageSendToSourceTrait, Module, module, TextEleParseTrait};
use proc_qq::re_exports::ricq_core::msg::elem::Text;
use crate::msg_util::MessageChain;
use crate::utils::{CanReply, Reg};
use crate::utils::msg_util::text;

static ID: &'static str = "help";
static NAME: &'static str = "帮助";

#[event]
async fn group_help(msg :&MessageEvent) -> anyhow::Result<bool>{
    let content = msg.message_content();
    if msg.is_group_message() {
        if Reg::ex(&content,&["help"],Some(&[Reg::All])) {
            msg.send_message_to_source(text("暂未完成")).await?;
            return Ok(true);
        }
    }
    Ok(false)
}

pub(crate) fn help_module() -> Module {
    module!(
        ID,
        NAME,
        group_help,
    )
}