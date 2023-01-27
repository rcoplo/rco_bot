use rbatis::{crud,impl_select};
use crate::database::table::BiliPush;
crud!(BiliPush{});

impl_select!(BiliPush{select_group_id_by_uid(uid:&i64) -> Option => "`where uid = #{uid}`"});