use std::io::Read;
use once_cell::sync::Lazy;
use proc_qq::re_exports::ricq_core::RQError;
use crate::BotResult;

pub mod help_image_util;
pub mod ett;
pub mod emoji_make_util;


pub static MSYHBD:&[u8] = include_bytes!("../../../resources/font/MSYHBD.TTC");
static LOLITI:&[u8] = include_bytes!("../../../resources/font/萝莉体.ttc");


fn file_to_image(path:String) ->BotResult<Vec<u8>>{
    let mut f = std::fs::File::open(path).map_err(RQError::IO)?;
    let mut b = vec![];
    f.read_to_end(&mut b).map_err(RQError::IO)?;
    Ok(b)
}