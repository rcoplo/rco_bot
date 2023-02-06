#![allow(unused_variables)] //允许未使用的变量
#![allow(unused_must_use)]

pub mod modules;
pub mod database;
mod config;
mod utils;
mod api;
mod error;

use once_cell::sync::Lazy;
use rbatis::{Error, log, Rbatis};
use rbatis::executor::RbatisRef;
use rbatis::table_sync::{SqliteTableSync, TableSync};
use rbdc_sqlite::driver::SqliteDriver;
use rbs::to_value;
use tracing_subscriber::util::SubscriberInitExt;
pub use config::*;
pub use config::*;
pub use utils::{
    msg_util,chrome_util
};
pub use error::{
    BotResult, BotError
};
use crate::database::implement::bili_push_impl::BiliPushImpl;
use crate::database::implement::osu_sb_impl::OsuSbImpl;
use crate::database::implement::sign_impl::SignImpl;
use crate::database::table::{BiliPush, OsuSb, Sign};
use crate::utils::file_util::get_resources_path;

extern crate rbatis;


pub static CONTEXT: Lazy<BotConText> = Lazy::new(||{BotConText::default()});


#[macro_export]
macro_rules! pool {
    () => {
        &mut $crate::CONTEXT.rbatis.clone()
    };
}

pub struct BotConText{
    pub config: RcoBotConfig,
    pub rbatis:Rbatis,
    pub bili_push:BiliPushImpl,
    pub osu_sb:OsuSbImpl,
    pub sign:SignImpl,
}

impl Default for BotConText{
    fn default() -> Self {
        let config = RcoBotConfig::default();

        Self{
            rbatis: database::init_rbatis(&config),
            config,
            bili_push: BiliPushImpl {},
            osu_sb: OsuSbImpl {},
            sign: SignImpl {},
        }
    }
}

impl BotConText {
    pub async fn init_pool(&self) {
        let path = get_resources_path(vec!["data", "bot.db"]);
        tracing::debug!("{}", &path);
        self.rbatis.init(SqliteDriver {}, path.as_str()).unwrap();
        // let mut s = SqliteTableSync::default();
        // s.sql_id = " PRIMARY KEY AUTOINCREMENT NOT NULL ".to_string();
        // // bili_push
        // s.sync(self.rbatis.acquire().await.unwrap(), to_value!(BiliPush::default()), "bili_push")
        //     .await
        //     .unwrap();
        // // osu_sb
        // s.sync(self.rbatis.acquire().await.unwrap(), to_value!(OsuSb::default()), "osu_sb")
        //     .await
        //     .unwrap();
        // // Sign
        // s.sync(self.rbatis.acquire().await.unwrap(), to_value!(Sign::default()), "sign")
        //     .await
        //     .unwrap();
    }
}