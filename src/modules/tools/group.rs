use proc_qq::{event, JoinGroupRequestEvent, MessageChainParseTrait, Module, module};

use crate::msg_util::MessageChain;


pub(crate) fn module() -> Module {
    module!(
        "group",
        "group",
        group_tool_join_group,
    )
}

#[event]
async fn group_tool_join_group(event: &JoinGroupRequestEvent) -> anyhow::Result<bool> {
    match event.accept().await {
        Ok(_) => {
            let chain = MessageChain::new()
                .at(event.inner.req_uin)
                .text(" 欢迎大佬入群喵~~")
                .build();
            event.client.send_group_message(event.inner.group_code, chain).await?;
            Ok(true)
        }
        Err(err) => {
            event.client.send_group_message(event.inner.group_code, err.to_string().parse_message_chain()).await?;
            Ok(true)
        }
    }
}

