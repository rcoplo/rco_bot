use rbatis::{crud, impl_select};
use crate::database::table::McServer;
crud!(McServer {});

impl_select!(McServer{select_server_by_name(name:&str,group_id:i64) -> Option => "`where name = #{name} and group_id = #{group_id}`"});