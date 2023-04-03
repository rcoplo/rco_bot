use chrono::{Datelike};
use proc_qq::{ClientTrait, event, GroupMessageEvent, Module, module};
use proc_qq::re_exports::anyhow;
use proc_qq::re_exports::ricq::msg::elem::RQElem;
use rand::Rng;

use crate::{CONTEXT};
use crate::database::table::Sign;

use crate::msg_util::CanReply;


pub(crate) fn module() -> Module {
    module!(
        "sign",
        "签到",
        sign,
        sign_at,
    )
}

#[event(regexp = "(小白摸+)|(摸+小白)")]
async fn sign(event: &GroupMessageEvent) -> anyhow::Result<bool> {
    _sign(event).await?;
    Ok(true)
}

#[event(regexp = "(.*)摸+")]
async fn sign_at(event: &GroupMessageEvent) -> anyhow::Result<bool> {
    for x in event.inner.elements.clone() {
        match x {
            RQElem::At(at) => {
                if at.target == event.client.bot_uin().await {
                    _sign(event).await?;
                    return Ok(true);
                }
            }
            _ => return Ok(false)
        }
    }
    Ok(false)
}

async fn _sign(event: &GroupMessageEvent) -> anyhow::Result<bool> {
    let sign = CONTEXT.sign.select_sign(event.inner.from_uin).await;
    let time = chrono::Local::now().naive_local();
    let rand_num = rand::thread_rng().gen_range(0..100);
    match sign {
        Ok(data) => {
            if time.day() == data.sign_time.day() {
                event.at_text("今天你已经签到过了喵...").await?;
                Ok(true)
            } else {
                CONTEXT.sign.update_sign_time(&time, event.inner.from_uin).await?;
                event.at_text(
                    format!("喵喵~~,签到成功了喵! \n 心情值:{}", rand_num).as_str()
                ).await?;
                Ok(true)
            }
        }
        Err(_) => {
            let i = rand::thread_rng().gen_range(0.0..CONTEXT.config.sign_config.scope);
            let sign = Sign {
                sign_time: time,
                user_id: event.inner.from_uin,
                favorability: i,
                ..Default::default()
            };
            CONTEXT.sign.insert(&sign).await?;
            event.at_text(
                format!("喵喵~~,签到成功了喵! \n 心情值:{}", rand_num).as_str()
            ).await?;
            Ok(true)
        }
    }
}