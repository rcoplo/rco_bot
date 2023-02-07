use reqwest::header::HeaderMap;
use tracing::event;
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
pub async fn http_get_image(url:&String) -> BotResult<Vec<u8>>{
    let bytes = reqwest::get(url)
        .await.unwrap().error_for_status();
    match bytes {
        Ok(bytes) => {
            let bytes = bytes.bytes().await?;
            Ok(bytes.to_vec())
        }
        Err(err) => {
            Err(BotError::from(err))
        }
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
