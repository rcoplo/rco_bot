use proc_qq::re_exports::ricq::version::ANDROID_WATCH;
use proc_qq::{
    result, run_client, Authentication, ClientBuilder, DeviceSource, EventResult, FileSessionStore,
    SessionStore,
};
use rco_bot::{modules, CONTEXT, BotConText};
use std::sync::{Arc, Mutex, RwLock};
use proc_qq::re_exports::{anyhow, tokio, tracing};
use proc_qq::re_exports::tracing::Level;
use rbatis::Rbatis;
use tracing_subscriber::EnvFilter;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use rco_bot::database::implement::bili_push_impl::BiliPushImpl;
use rco_bot::database::table::BiliPush;
use rco_bot::modules::BiliPushTask;
use rco_bot::scheduler::Scheduler;


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
    //添加定时任务

    let scheduler = Scheduler::new(copy).await;
    scheduler.add(BiliPushTask).await;

    scheduler.start().await;
    run_client(client).await
        .expect("启动时出现错误");
    Ok(())
}

fn init_tracing_subscriber() {
    let timer = tracing_subscriber::fmt::time::OffsetTime::new(
        time::UtcOffset::from_hms(8, 0, 0).unwrap(),
        time::format_description::parse(
            "[year repr:last_two]-[month]-[day] [hour]:[minute]:[second]",
        )
            .unwrap(),
    );
    let env = EnvFilter::from(CONTEXT.config.log.as_str());
    tracing_subscriber::fmt()
        .with_env_filter(env)
        .with_timer(timer)
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
