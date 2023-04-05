
pub mod table;
pub mod implement;
mod mapper;


pub fn init_rbatis(config: &crate::RcoBotConfig) -> rbatis::Rbatis {
    let rbatis = rbatis::Rbatis::new();
    return rbatis;
}
