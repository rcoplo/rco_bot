use rbatis::{crud, impl_delete, impl_select};
use crate::database::table::BiliPush;
crud!(BiliPush{});

impl_select!(BiliPush{select_bili_push_by_uid(uid:i64,group_id:i64) -> Option => "`where uid = #{uid} and group_id = #{group_id}`"});
impl_select!(BiliPush{select_bili_push_by_room_id(room_id:i64,group_id:i64) -> Option => "`where room_id = #{room_id} and group_id = #{group_id}`"});
impl_delete!(BiliPush{delect_bili_push_by_uid_group_id(uid:i64,group_id:i64) => "`where uid = #{uid} and group_id = #{group_id}`"});