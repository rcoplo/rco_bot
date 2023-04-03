use proc_qq::{event, MessageEvent, Module, module, MessageSendToSourceTrait, MessageChainParseTrait};

pub(crate) fn module() -> Module {
    module! {
    "help",
    "帮助",
    help,
}
}

#[event(bot_command = "/help {name}")]
async fn help(event: &MessageEvent, name: Option<String>) -> anyhow::Result<bool> {
    event.send_message_to_source("暂未完成喵...".parse_message_chain()).await?;
    Ok(true)
}
