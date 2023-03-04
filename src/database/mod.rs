use rbatis::Rbatis;
use crate::RcoBotConfig;

pub mod table;
pub mod implement;
mod mapper;


pub fn init_rbatis(config: &RcoBotConfig) -> Rbatis {
    let rbatis = Rbatis::new();
    if rbatis.is_debug_mode() == false && config.debug.eq(&true) {
        panic!(
            r#"已使用release模式运行，但是仍使用debug模式！请修改 botconfig.yml 中debug配置项为  debug: false"#
        );
    }
    return rbatis;
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct GroupVec {
    group_id: Vec<i64>,
}