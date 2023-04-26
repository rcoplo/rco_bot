use std::time::Duration;
use proc_qq::{event, MessageChainParseTrait, MessageEvent, MessageSendToSourceTrait, Module, module};
use proc_qq::re_exports::{anyhow, tokio, tracing};


#[event(bot_command = "{time}秒后发送{context}")]
async fn test_1(e: &MessageEvent, time: Option<i8>, context: Option<String>) -> anyhow::Result<bool> {
    tracing::info!("test ok");
    tracing::info!("{:?}   {:?}",&time,&context);
    if let (Some(time), Some(context)) = (time, context) {
        e.send_message_to_source("ok".parse_message_chain()).await?;
        tokio::time::sleep(Duration::from_secs(time as u64)).await;
        e.send_message_to_source(context.parse_message_chain()).await?;
        return Ok(true);
    }
    Ok(false)
}

pub fn module() -> Module {
    module!(
        "test",
        "test",
        test_1
    )
}
