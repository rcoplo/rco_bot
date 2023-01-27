use std::path::Path;
use std::time::Duration;
use thirtyfour::{By, DesiredCapabilities, WebDriver};
use thirtyfour::error::WebDriverResult;
use crate::{BotError, BotResult, CONTEXT};



pub async fn bili_dynamic_screenshot(uid:&i64) -> BotResult<Vec<u8>> {
    let dynamic_url = format!("https://space.bilibili.com/{}/dynamic", uid);
    let caps = DesiredCapabilities::chrome();
    let driver = WebDriver::new(CONTEXT.config.chrome_driver_url.clone().as_str(), caps).await?;
    driver.goto(dynamic_url).await.unwrap();
    tokio::time::sleep(Duration::from_millis(5000)).await;
    let elem = driver.find(By::XPath("/html/body/div[2]/div[4]/div/div/div[1]/div/div[1]/div[1]")).await?;
    let screenshot = elem.screenshot_as_png().await;

    match screenshot {
        Ok(data) => {
            driver.quit().await?;
            Ok(data)
        }
        Err(err) => {
            driver.quit().await?;
            Err(BotError::from(err))
        }
    }
}
pub async fn bili_video_screenshot(uid:&i64) -> BotResult<Vec<u8>> {
    let dynamic_url = format!("https://space.bilibili.com/{}/video", uid);
    let caps = DesiredCapabilities::chrome();
    let driver = WebDriver::new(CONTEXT.config.chrome_driver_url.clone().as_str(), caps).await?;
    driver.goto(dynamic_url).await.unwrap();
    tokio::time::sleep(Duration::from_millis(5000)).await;
    let elem = driver.find(By::XPath("/html/body/div[2]/div[4]/div/div/div[2]/div[1]/div[2]/div/span[2]")).await?;
    elem.click().await?;
    let element = driver.find(By::XPath("/html/body/div[2]/div[4]/div/div/div[2]/div[4]/div/div/ul[1]/li[1]")).await?;
    let screenshot = element.screenshot_as_png().await;
    match screenshot {
        Ok(data) => {
            driver.quit().await?;
            Ok(data)
        }
        Err(err) => {
            driver.quit().await?;
            Err(BotError::from(err))
        }
    }
}



