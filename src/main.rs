use proc_qq::re_exports::ricq::version::ANDROID_WATCH;
use proc_qq::{
    result, run_client, Authentication, ClientBuilder, DeviceSource, EventResult, FileSessionStore,
    SessionStore,
};
use rco_bot::{modules, CONTEXT};
use std::sync::Arc;
use proc_qq::re_exports::{anyhow, tokio, tracing};
use proc_qq::re_exports::tracing::Level;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;


#[tokio::main]
async fn main() -> anyhow::Result<()> {
    init_tracing_subscriber();
    CONTEXT.init_pool().await;

    let config = CONTEXT.config.bot_config.clone();
    let authentication = match config.login_type.as_str() {
        "uin_pwd" => Authentication::UinPassword(config.account_uin, config.account_pwd),
        "qr_code" => Authentication::QRCode,
        _ => Authentication::Abandon,
    };
    let client = ClientBuilder::new()
        .device(DeviceSource::JsonFile(
            "./resources/data/device.json".to_owned(),
        ))
        .version(&ANDROID_WATCH)
        .session_store(FileSessionStore::boxed("./resources/data/session.token"))
        .authentication(authentication)
        .show_slider_pop_menu_if_possible()
        .modules(Arc::new(modules::all_modules()))
        .result_handlers(vec![on_result {}.into()])
        .build()
        .await
        .unwrap();
    // 可以做一些定时任务, rq_client在一开始可能没有登录好
    let client = Arc::new(client);
    let copy = client.clone();
    tokio::spawn(async move {
        tracing::info!(
            "{:?}",
            chrono::NaiveDateTime::from_timestamp_millis(copy.rq_client.start_time.into())
        );
    });
    run_client(client).await?;
    Ok(())
}

fn init_tracing_subscriber() {
    let level = match CONTEXT.config.debug {
        true => Level::DEBUG,
        false => Level::INFO,
    };
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::fmt::layer()
                .with_target(true)
                .without_time(),
        )
        .with(
            tracing_subscriber::filter::Targets::new()
                .with_target("ricq", level)
                .with_target("proc_qq", level)
                .with_target("rco_bot", level)
                .with_target("rbatis", level)
                .with_target("tracing", level)
                .with_target("hyper", level),
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
