use rbatis::{crud, impl_select};
use crate::database::table::Sign;
crud!(Sign {});
impl_select!(Sign{select_sign(user_id:i64) -> Option =>"`where user_id = #{user_id}`"});