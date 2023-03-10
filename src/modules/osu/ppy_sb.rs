use proc_qq::{event, MessageContentTrait, MessageEvent, Module, module};
use crate::utils::Reg;

static ID: &'static str = "osu_sb";
static NAME: &'static str = "sb服数据查询";


#[event]
async fn ppy_sb_user_info(event:&MessageEvent) -> anyhow::Result<bool> {
    let content = event.message_content();
    if Reg::ex(content.as_str(), &["info"], Some(&[Reg::All])) {}
    Ok(false)
}

pub(crate) fn module() -> Module {
    module!(
        ID,
        NAME,
        ppy_sb_user_info,
    )
}
