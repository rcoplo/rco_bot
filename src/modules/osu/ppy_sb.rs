use proc_qq::{event, MessageContentTrait, MessageEvent, Module, module};
use proc_qq::re_exports::anyhow;


static ID: &'static str = "osu_sb";
static NAME: &'static str = "sb服数据查询";


#[event]
async fn ppy_sb_user_info(event:&MessageEvent) -> anyhow::Result<bool> {
    let content = event.message_content();

    Ok(false)
}

pub(crate) fn module() -> Module {
    module!(
        ID,
        NAME,
        ppy_sb_user_info,
    )
}
