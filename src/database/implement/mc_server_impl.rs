use crate::{BotError, BotResult, pool};

use crate::database::table::McServer;

pub struct McServerImpl {}

pub enum McServerType {
    JAVA,
    Bedrock,
}

impl McServerType {
    pub fn new(server_type: &str) -> BotResult<McServerType> {
        match server_type {
            "JE" => Ok(McServerType::JAVA),
            "BE" => Ok(McServerType::Bedrock),
            _ => Err(BotError::from(format!("\"{}\" 这个类型不存在喵... 只有:[JE,BE]", server_type)))
        }
    }
}

impl ToString for McServerType {
    fn to_string(&self) -> String {
        match self {
            McServerType::JAVA => "JE".to_string(),
            McServerType::Bedrock => "BE".to_string()
        }
    }
}

impl McServerImpl {
    pub async fn new(&self, name: &str, url: &str, group_id: i64, server_type: BotResult<McServerType>) -> BotResult<()> {
        match self.select_server_by_name_group_id(name, group_id).await {
            None => {
                McServer::insert(pool!(), &McServer {
                    id: None,
                    group_id,
                    name: name.to_string(),
                    url: url.to_string(),
                    server_type: {
                        match server_type {
                            Ok(data) => data.to_string(),
                            Err(err) => return Err(err),
                        }
                    },
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
    pub async fn update_server_type_by_name_group_id(&self, name: &str, group_id: i64, new_server_type: BotResult<McServerType>) -> BotResult<()> {
        match self.select_server_by_name_group_id(name, group_id).await {
            None => {
                Err(BotError::from("本群并没有绑定这个服务器简称喵..."))
            }
            Some(mc_server) => {
                McServer::update_by_column(pool!(), &McServer {
                    server_type: {
                        match new_server_type {
                            Ok(data) => data.to_string(),
                            Err(err) => return Err(err),
                        }
                    },
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