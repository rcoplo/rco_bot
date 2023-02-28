use std::collections::HashMap;
use std::iter::Map;
use rbatis::dark_std::err;
use reqwest::header::HeaderMap;
use serde_json::{Error, json, Value};
use crate::{BotError, BotResult};
use crate::utils::http_util::http_post_json;

const URL:&str = "https://api.lolicon.app/setu/v2";
#[derive(Debug,Clone,serde::Serialize, serde::Deserialize)]
pub struct Setu {
    pub pid:i64,
    pub p:i64,
    pub uid:i64,
    pub title:String,
    pub author:String,
    pub r18:bool,
    pub width:i32,
    pub height:i32,
    pub tags:Vec<String>,
    pub ext:String,
    #[serde(rename = "aiType")]
    pub ai_type:i8,
    #[serde(rename = "uploadDate")]
    pub upload_date:i64,
    pub urls:ImageUrls,

}
#[derive(Debug,Clone,serde::Serialize, serde::Deserialize)]
pub struct ImageUrls{
    pub original:String,
}

async fn get(url: &str, data:Value) -> BotResult<Value>{
    let result = http_post_json(&url.to_string(), &data).await?;
    let result = serde_json::from_str::<Value>(result.as_str())?;
    Ok(result)
}

pub async fn get_lolicon_list_r18(num:i8)-> BotResult<Vec<Setu>> {
    let json = json!(
       {
            "r18": 1,
            "num": num,
        }
    );
    let setu = get(URL, json).await?;
    to_setu_list(setu)
}

pub async fn get_lolicon_list(num:i8)-> BotResult<Vec<Setu>> {
    let json = json!(
       {
            "r18": 0,
            "num": num,
        }
    );
    let setu = get(URL, json).await?;
    to_setu_list(setu)
}

pub async fn get_lolicon_list_tag(num:i8,tag:Vec<String>)-> BotResult<Vec<Setu>>{
    let value = Value::from(tag);
    tracing::debug!("tag = {:?}",&value);
    let json = json!(
       {
            "r18": 0,
            "num": num,
            "tag": value,
        }
    );

    let setu = get(URL, json).await?;
    to_setu_list(setu)
}

pub async fn get_lolicon_r18_tag(tag:Vec<String>)-> BotResult<Setu> {
    let value = Value::from(tag);
    tracing::debug!("tag = {:?}",&value);
    let json = json!(
       {
            "r18": 1,
            "tag": value,
        }
    );
    let setu = get(URL, json).await?;
    to_setu(setu)
}

pub async fn get_lolicon_r18()-> BotResult<Setu> {
    let json = json!(
       {
            "r18": 1,
        }
    );
    let setu = get(URL, json).await?;
    to_setu(setu)
}

pub async fn get_lolicon()-> BotResult<Setu> {
    let json = json!(
       {
            "r18": 0,
        }
    );
    let setu = get(URL, json).await?;
    to_setu(setu)
}

pub async fn get_lolicon_tag(tag:Vec<String>)-> BotResult<Setu> {
    let value = Value::from(tag);
    tracing::debug!("tag = {:?}",&value);
    let json = json!(
       {
            "r18": 0,
            "tag": value,
        }
    );
    let setu = get(URL, json).await?;
    to_setu(setu)
}

fn to_setu(data:Value)-> BotResult<Setu> {
    match serde_json::from_value::<Setu>(data["data"][0].clone()) {
        Ok(mut setu) => {
            setu.urls.original = setu.urls.original.replace("i.pixiv.re", "pixiv.rco.ink");
            Ok(setu)
        }
        Err(_) => Err(BotError::from("没有这种色图喵..."))
    }
}
fn to_setu_list(data:Value) -> BotResult<Vec<Setu>> {
    let mut vec = vec![];
    let data = data["data"].as_array().unwrap().clone();
    for v in data {
        match serde_json::from_value::<Setu>(v) {
            Ok(mut lolicon) => {
                lolicon.urls.original = lolicon.urls.original.replace("i.pixiv.re", "pixiv.rco.ink");
                vec.push(lolicon);
            }
            Err(_) => {
                return Err(BotError::from("没有这种色图喵..."));
            }
        };
    }
    Ok(vec)
}