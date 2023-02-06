use rbatis::{crud, impl_select};
use crate::database::table::OsuSb;
crud!(OsuSb {});

impl_select!(OsuSb{select_user_by_user_name(user_name:&String) -> Option => "`where user_name = #{user_name}`"});
impl_select!(OsuSb{select_user_by_user_id(user_id:&i32) -> Option => "`where user_id = #{user_id}`"});