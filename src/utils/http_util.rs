use proc_qq::re_exports::{serde_json, tokio};
use proc_qq::re_exports::anyhow::__private::kind::TraitKind;
use reqwest::header::HeaderMap;
use crate::{BotError, BotResult};

pub async fn http_get(url: &str) -> BotResult<String> {
    let data = reqwest::get(url)
        .await?
        .text()
        .await;
    match data {
        Ok(data) => Ok(data),
        Err(err) => Err(BotError::from(err))
    }
}

pub async fn http_get_image(url: &str) -> BotResult<Vec<u8>> {
    let bytes = match reqwest::get(url).await {
        Ok(res) => {
            res
        }
        Err(err) => {
            return Err(BotError::from(format!("获取图片失败,响应码: {:?}\n image url: {}", err.status(), url)));
        }
    };
    match tokio::time::timeout(std::time::Duration::from_secs(60), bytes.bytes()).await {
        Ok(bytes) => {
            match bytes {
                Ok(b) => {
                    Ok(b.to_vec())
                }
                Err(err) => {
                    return return Err(BotError::from(format!("获取图片失败,响应码: {:?}\n image url: {}", err.status(), url)));
                }
            }
        },
        Err(_) => {
            Err(BotError::from("获取图片超时喵..."))
        }
    }
}

pub async fn http_post_json(url: &str, json: &serde_json::Value) -> BotResult<String> {
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
