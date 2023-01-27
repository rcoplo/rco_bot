

pub async fn http_get(url:&String) -> Result<String,reqwest::Error>{
    let data = reqwest::get(url)
        .await?
        .text()
        .await?;
    Ok(data)
}

pub async fn http_post_json(url:&String,json:&serde_json::Value) -> Result<String,reqwest::Error>{
    let client = reqwest::Client::new();
    let data = client
        .post(url)
        .json(json)
        .send()
        .await?
        .text()
        .await?;
    Ok(data)
}
