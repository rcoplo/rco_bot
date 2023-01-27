
use proc_qq::re_exports::ricq::version::ANDROID_WATCH;
use proc_qq::{result, run_client, ClientBuilder, DeviceSource, Authentication, EventResult, ShowQR, CustomUinPassword};
use std::sync::Arc;
use tracing::Level;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use RcoBot::{CONTEXT, modules};


#[tokio::main]
async fn main() -> anyhow::Result<()> {
    init_tracing_subscriber();
    CONTEXT.init_pool().await;
    let config = CONTEXT.config.clone();
    let client = ClientBuilder::new()
        .device(DeviceSource::JsonFile("device.json".to_owned()))
        .version(&ANDROID_WATCH)
        .priority_session("session.token")
        .authentication(Authentication::UinPassword(config.account.uin, config.account.pwd))
        .show_slider_pop_menu_if_possible()
        .modules(modules::all_modules())
        .result_handlers(vec![on_result {}.into()])
        .build()
        .await
        .unwrap();
    // 可以做一些定时任务, rq_client在一开始可能没有登录好
    let client = Arc::new(client);
    let copy = client.clone();
    tokio::spawn(async move {
        tracing::info!("{}", copy.rq_client.start_time);
    });
    run_client(client).await?;
    Ok(())
}

fn init_tracing_subscriber() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::fmt::layer()
                .with_target(true)
                .without_time(),
        )
        .with(
            tracing_subscriber::filter::Targets::new()
                .with_target("ricq", Level::DEBUG)
                .with_target("proc_qq", Level::DEBUG)
                .with_target("RcoBot", Level::DEBUG)
                .with_target("rbatis", Level::DEBUG)
        )
        .init();

}

#[result]
pub async fn on_result(result: &EventResult) -> anyhow::Result<bool> {
    match result {
        EventResult::Process(info) => {
            tracing::info!("{} : {} : 处理了一条消息", info.module_id, info.handle_name);
        }
        EventResult::Exception(info, err) => {
            tracing::info!(
                "{} : {} : 遇到了错误 : {}",
                info.module_id,
                info.handle_name,
                err
            );
        }
    }
    Ok(false)
}