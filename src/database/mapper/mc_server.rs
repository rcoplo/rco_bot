use rbatis::{crud, impl_select};
use crate::database::table::McServer;
crud!(McServer {});

impl_select!(McServer{select_server_by_name(name:&str) -> Option => "`where name = #{name}`"});