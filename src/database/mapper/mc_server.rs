use rbatis::{crud, impl_select};
use crate::database::table::McServer;
crud!(McServer {});

impl_select!(McServer{select_server_by_name(name:&str) -> Option => "`where name = #{name}`"});
impl_select!(McServer{select_server_all_by_group_id(group_id:i64)  => "`where group_id = #{group_id}`"});