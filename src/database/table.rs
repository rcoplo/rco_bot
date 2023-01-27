use serde_json::Value;

pub const BILI_PUSH:&str = r##"
create table bili_push
(
    id                  INTEGER not null
        constraint table_name_pk
            primary key autoincrement,
    room_id             INTEGER not null,
    uid                 INTEGER not null,
    uname               TEXT,
    group_id            TEXT,
    live_status         INTEGER,
    latest_video_time   INTEGER,
    latest_dynamic_time INTEGER,
    live_push           INTEGER default 1 not null,
    video_push          INTEGER default 1 not null,
    dynamic_push        INTEGER default 1 not null
);
"##;


#[derive(Debug , Clone,Default,serde::Deserialize,serde::Serialize)]
pub struct BiliPush{
    pub id:i32,
    pub room_id:i64,
    pub uid:i64,
    pub uname:String,
    pub group_id:String,
    pub live_status:i32,
    pub latest_video_time:i64,
    pub latest_dynamic_time:i64,
    pub live_push:i8,
    pub video_push:i8,
    pub dynamic_push:i8,
}