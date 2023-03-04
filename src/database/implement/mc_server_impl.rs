use rbatis::rbdc::Error;
use serde_json::Value;
use crate::{BotError, BotResult, pool};
use crate::database::GroupVec;
use crate::database::table::McServer;

pub struct McServerImpl {}


impl McServerImpl {
    pub async fn new(&self, name: &str, url: &str, group_id: i64) -> BotResult<()> {
        match self.select_server_by_name_group_id(name, group_id).await {
            None => {
                McServer::insert(pool!(), &McServer {
                    id: None,
                    group_id,
                    name: name.to_string(),
                    url: url.to_string(),
                }).await?;
                Ok(())
            }
            Some(_) => {
                Err(BotError::from("本群已有该服务器的简称喵..."))
            }
        }
    }

    pub async fn select_server_by_name_group_id(&self, name: &str, group_id: i64) -> Option<McServer> {
        let mc_server = McServer::select_server_by_name(pool!(), name, group_id).await.ok()?;
        mc_server
    }

    pub async fn select_server_all_by_group_id(&self, group_id: i64) -> Option<Vec<McServer>> {
        let mc_server = McServer::select_by_column(pool!(), "group_id", group_id).await.ok();
        mc_server
    }

    pub async fn update_name_by_name_group_id(&self, name: &str, group_id: i64, new_name: &str) -> BotResult<()> {
        match self.select_server_by_name_group_id(name, group_id).await {
            None => {
                Err(BotError::from("本群并没有绑定这个服务器简称喵..."))
            }
            Some(mc_server) => {
                McServer::update_by_column(pool!(), &McServer {
                    name: new_name.to_string(),
                    ..mc_server
                }, "id").await?;
                Ok(())
            }
        }
    }

    pub async fn update_url_by_name_group_id(&self, name: &str, group_id: i64, new_url: &str) -> BotResult<()> {
        match self.select_server_by_name_group_id(name, group_id).await {
            None => {
                Err(BotError::from("本群并没有绑定这个服务器简称喵..."))
            }
            Some(mc_server) => {
                McServer::update_by_column(pool!(), &McServer {
                    url: new_url.to_string(),
                    ..mc_server
                }, "id").await?;
                Ok(())
            }
        }
    }

    pub async fn delete_server_by_name_group_id(&self, name: &str, group_id: i64) -> BotResult<()> {
        match self.select_server_by_name_group_id(name, group_id).await {
            None => {
                Err(BotError::from("本群并没有绑定这个服务器简称喵..."))
            }
            Some(mc_server) => {
                McServer::delete_by_column(pool!(), "name", mc_server.name).await?;
                Ok(())
            }
        }
    }
}