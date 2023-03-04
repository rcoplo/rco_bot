use rbatis::{crud, impl_select};
use crate::database::table::EttUser;
crud!(EttUser {});

impl_select!(EttUser{select_user_by_user_name(user_name:&String) -> Option => "`where user_name = #{user_name}`"});
impl_select!(EttUser{select_user_by_qq(user_id_qq:i64) -> Option => "`where user_id_qq = #{user_id_qq}`"});