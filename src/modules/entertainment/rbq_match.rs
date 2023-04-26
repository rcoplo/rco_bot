use proc_qq::{event, MessageEvent, module, Module};
use proc_qq::re_exports::anyhow;


pub(crate) fn module() -> Module {
    module!(
        "wife_match",
        "owo",
        wife_match,
    )
}

#[event]
async fn wife_match(e: &MessageEvent) -> anyhow::Result<bool> {
    todo!()
}