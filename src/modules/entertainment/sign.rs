use chrono::{Datelike, NaiveDateTime, TimeZone};
use proc_qq::{event, GroupMessageEvent, MessageContentTrait, Module, module};
use rand::prelude::SliceRandom;
use rand::Rng;
use crate::{BotResult, CONTEXT};
use crate::database::table::Sign;
use crate::msg_util::{MessageChain,CanReply};
use crate::utils::Reg;

static ID: &'static str = "sign";
static NAME: &'static str = "签到";

pub(crate) fn module() -> Module {
    module!(
        ID,
        NAME,
        sign,
    )
}

pub struct SignHelp{
    pub mod_name:String,
    pub help_text:Vec<String>,
}
impl Default for SignHelp{
    fn default() -> Self {
        SignHelp{
            mod_name: "签到".to_string(),
            help_text: vec![
                "sign help",
                "----------------------------------------------------------------",
                "小白摸 摸小白",
                "----------------------------------------------------------------",
                "可以有很多摸(  例: 小白摸摸摸摸摸摸摸摸摸摸摸",
            ].iter().map(|str| str.to_string()).collect::<Vec<_>>(),
        }
    }
}

#[event]
async fn sign(event:&GroupMessageEvent) -> anyhow::Result<bool> {
    let content = event.message_content();
    if Reg::ex(content.as_str(), &["小白摸+", "摸+小白"], None) {
        let sign = CONTEXT.sign.select_sign(&event.inner.from_uin).await;
        let time = chrono::Local::now().naive_local();
        let mut nums: Vec<i32> = (1..100).collect();
        let rand_num = rand::thread_rng().gen_range(0..100);
        return match sign {
            Ok(data) => {
                if time.day() == data.sign_time.day() {
                    event.at_text("今天你已经签到过了喵...").await?;
                    Ok(true)
                } else {
                    CONTEXT.sign.update_sign_time(&time,&event.inner.from_uin).await?;
                    event.at_text(
                        format!("喵喵~~,签到成功了喵! \n 心情值:{}", rand_num).as_str()
                    ).await?;
                    Ok(true)
                }
            }
            Err(_) => {
                let sign = Sign {
                    sign_time: time,
                    user_id: event.inner.from_uin,
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
    Ok(false)
}
