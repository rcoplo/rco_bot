use proc_qq::{ClientTrait, event, GroupLeaveEvent, JoinGroupRequestEvent, MessageChainParseTrait, Module, module};
use proc_qq::re_exports::anyhow;
use proc_qq::re_exports::ricq::structs::GroupMemberPermission;

use crate::msg_util::MessageChain;


pub(crate) fn module() -> Module {
    module!(
        "group",
        "group",
        group_tool_join_group,
        group_tool_leave_group,
    )
}

#[event]
async fn group_tool_join_group(event: &JoinGroupRequestEvent) -> anyhow::Result<bool> {
    let map = event.client.get_group_admin_list(event.inner.group_code).await?;
    for (user_id, p) in map {
        if user_id == event.client.bot_uin().await {
            match p {
                GroupMemberPermission::Member => {}
                _ => {
                    event.accept().await?;
                }
            }
        }
    }
    let chain = MessageChain::new()
        .at(event.inner.req_uin, event.inner.req_nick.as_str())
        .text(" 欢迎大佬入群喵~~")
        .build();
    event.client.send_group_message(event.inner.group_code, chain).await?;
    Ok(true)
}

#[event]
async fn group_tool_leave_group(event: &GroupLeaveEvent) -> anyhow::Result<bool> {
    let info = event.client.get_summary_info(event.inner.member_uin).await?;
    let chain = MessageChain::new()
        .text(format!("{}离开了我们喵...", info.nickname))
        .build();
    event.client.send_group_message(event.inner.group_code, chain).await?;
    Ok(true)
}

