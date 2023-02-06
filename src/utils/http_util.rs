use reqwest::header::HeaderMap;
use crate::{BotError, BotResult};

pub async fn http_get(url:&String) -> BotResult<String>{
    let data = reqwest::get(url)
        .await?
        .text()
        .await;
    match data {
        Ok(data) => Ok(data),
        Err(err) => Err(BotError::from(err))
    }
}

pub async fn http_post_json(url:&String,json:&serde_json::Value) -> BotResult<String>{
    let client = reqwest::Client::new();
    let mut headers = HeaderMap::new();
    headers.insert("Content-Type", "application/json".parse().unwrap());
    let data = client
        .post(url)
        .headers(headers)
        .json(json)
        .send()
        .await?
        .text()
        .await;
    match data {
        Ok(data) => Ok(data),
        Err(err) => Err(BotError::from(err))
    }
}
