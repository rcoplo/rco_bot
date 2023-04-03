#![feature(iter_intersperse)]
#![allow(unused_variables)] //允许未使用的变量
#![allow(unused_must_use)]
#![feature(const_trait_impl)]
pub mod modules;
pub mod database;
mod config;
mod utils;
mod api;
mod error;
pub mod basic_modules;

use once_cell::sync::Lazy;
use proc_qq::re_exports::tracing;
use rbatis::{Error, log, Rbatis};
use rbatis::executor::RbatisRef;
use rbatis::table_sync::{SqliteTableSync, TableSync};
use rbdc_sqlite::driver::SqliteDriver;
use rbs::to_value;
pub use config::*;
pub use config::*;
pub use utils::{
    msg_util
};
pub use error::{
    BotResult, BotError
};

use crate::database::implement::bili_push_impl::BiliPushImpl;
use crate::database::implement::ett_user_impl::EttUserImpl;
use crate::database::implement::mc_server_impl::McServerImpl;
use crate::database::implement::osu_sb_impl::OsuSbImpl;
use crate::database::implement::sign_impl::SignImpl;
use crate::database::table::{BiliPush, EttUser, McServer, OsuSb, Sign};

extern crate rbatis;

pub static CONTEXT: Lazy<BotConText> = Lazy::new(||{BotConText::default()});


#[macro_export]
macro_rules! pool {
    () => {
        &mut $crate::CONTEXT.rbatis.clone()
    };
}

pub struct BotConText {
    pub config: RcoBotConfig,
    pub rbatis: Rbatis,
    pub bili_push: BiliPushImpl,
    pub osu_sb: OsuSbImpl,
    pub sign: SignImpl,
    pub ett: EttUserImpl,
    pub mc_server: McServerImpl,
}

impl Default for BotConText{
    fn default() -> Self {
        let config = RcoBotConfig::default();
        Self {
            rbatis: database::init_rbatis(&config),
            config,
            bili_push: BiliPushImpl {},
            osu_sb: OsuSbImpl {},
            sign: SignImpl {},
            ett: EttUserImpl {},
            mc_server: McServerImpl {},
        }
    }
}

impl BotConText {
    pub async fn init_pool(&self) {
        let path = resource_path!("data" =>"bot.db").unwrap_or_default();
        tracing::debug!("{}", &path);
        self.rbatis.init(SqliteDriver {}, path.as_str()).unwrap();
        let mut s = SqliteTableSync::default();
        s.sql_id = " PRIMARY KEY AUTOINCREMENT NOT NULL ".to_string();
        // bili_push
        s.sync(self.rbatis.acquire().await.unwrap(), to_value!(BiliPush{
                id:Some(0),
                ..Default::default()
        }), "bili_push")
            .await
            .unwrap();
        // osu_sb
        s.sync(self.rbatis.acquire().await.unwrap(), to_value!(OsuSb{
             id:Some(0),
                ..Default::default()
        }), "osu_sb")
            .await
            .unwrap();
        // Sign
        s.sync(self.rbatis.acquire().await.unwrap(),
               to_value!(Sign{
                id:Some(0),
                ..Default::default()
            }), "sign")
            .await
            .unwrap();
        // EttUser
        s.sync(self.rbatis.acquire().await.unwrap(), to_value!(EttUser{
                id:Some(0),
                ..Default::default()
        }), "ett_user")
            .await
            .unwrap();
        // mc_server
        s.sync(self.rbatis.acquire().await.unwrap(), to_value!(McServer{
                id:Some(0),
                ..Default::default()
        }), "mc_server")
            .await
            .unwrap();
    }
}