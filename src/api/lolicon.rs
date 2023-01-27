use std::collections::HashMap;
use std::iter::Map;
use reqwest::header::HeaderMap;
use serde_json::{json, Value};

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

async fn get(url: &str, data:Value) -> Option<Value>{
   let client = reqwest::Client::new();
    // 组装header
    let mut headers = HeaderMap::new();
    headers.insert("Content-Type", "application/json".parse().unwrap());

     let resp = client.post(URL).headers(headers).json(&data).send().await.ok()?;
    let result:Value = serde_json::from_str(resp.text().await.unwrap().as_str()).unwrap();
     Some(result)
}

pub async fn get_lolicon_list_r18(num:i8)-> Option<Vec<Setu>>{
   let json =  json!(
       {
            "r18": 1,
            "num": num,
        }
    );

    let data = get(URL, json).await;
    match data {
        None => None,
        Some(data) => {
            to_setu_list(data)
        }
    }

}

pub async fn get_lolicon_list(num:i64)-> Option<Vec<Setu>>{
    let json =  json!(
       {
            "r18": 0,
            "num": num,
        }
    );

    let data = get(URL, json).await;
    match data {
        None => None,
        Some(data) => {
            to_setu_list(data)
        }
    }
}
pub async fn get_lolicon_list_tag(num:i64,tag:Vec<String>)-> Option<Vec<Setu>>{
    let value = Value::from(tag);
    let json =  json!(
       {
            "r18": 0,
            "num": num,
            "tag": value,
        }
    );

    let data = get(URL, json).await;
    match data {
        None => None,
        Some(data) => {
            to_setu_list(data)
        }
    }
}

pub async fn get_lolicon_r18_tag(tag:Vec<String>)-> Option<Setu>{
    let value = Value::from(tag);
    let json =  json!(
       {
            "r18": 1,
            "tag": value,
        }
    );

    let data = get(URL, json).await;
    match data {
        None => None,
        Some(data) => {
            to_setu(data)
        }
    }
}
pub async fn get_lolicon_r18()-> Option<Setu>{
    let json =  json!(
       {
            "r18": 1,
        }
    );
    let data = get(URL, json).await;
    match data {
        None => None,
        Some(data) => {
            to_setu(data)
        }
    }
}
pub async fn get_lolicon()-> Option<Setu>{
    let json =  json!(
       {
            "r18": 0,
        }
    );
    let data = get(URL, json).await;
    match data {
        None => None,
        Some(data) => {
            to_setu(data)
        }
    }
}

pub async fn get_lolicon_tag(tag:Vec<String>)-> Option<Setu>{
    let value = Value::from(tag);
    let json =  json!(
       {
            "r18": 0,
            "tag": value,
        }
    );
    let data = get(URL, json).await;
    match data {
        None => None,
        Some(data) => {
            to_setu(data)
        }
    }
}

fn to_setu(data:Value)-> Option<Setu> {
    let setu = serde_json::from_value::<Setu>(data["data"][0].clone()).unwrap();
    Some(setu)

}
fn to_setu_list(data:Value) -> Option<Vec<Setu>> {
    let mut vec = vec![];
    let data = data["data"].as_array().unwrap().clone();
    for v in data {
        let lolicon = serde_json::from_value::<Setu>(v).unwrap();
        vec.push(lolicon);
    }
    Some(vec)
}