use proc_qq::re_exports::{serde, serde_json};
use proc_qq::re_exports::serde_json::{json, Map, Value};
use crate::error::{BotError, BotResult};

use crate::utils::http_util::http_post_json;

static LOLICOM_API_URI: &'static str = "https://api.lolicon.app/setu/v2";

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct LoliconApiStruct {
    pub pid: i64,
    pub p: i64,
    pub uid: i64,
    pub title: String,
    pub author: String,
    pub r18: bool,
    pub width: i32,
    pub height: i32,
    pub tags: Vec<String>,
    pub ext: String,
    #[serde(rename = "aiType")]
    pub ai_type: i8,
    #[serde(rename = "uploadDate")]
    pub upload_date: i64,
    pub urls: ImageUrls,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ImageUrls {
    pub original: String,
}

#[derive(Debug, Clone)]
pub struct LoliconApiBuilder {
    inner: Map<String, Value>,
}
macro_rules! lolicon_api_builder_fn {
    ($fname: ident , $api_key:literal => $ty: ty) => {
        pub fn $fname(&mut self, $fname: $ty) -> &mut Self {
            self.inner.insert($api_key.to_string(), Value::from($fname));
            self
        }
    };

    ($fname: ident, $api_key:literal = $value:literal) => {
        pub fn $fname(&mut self) -> &mut Self {
            self.inner.insert($api_key.to_string(), Value::from($value));
            self
        }
    };
}

impl LoliconApiBuilder {
    pub fn new() -> Self {
        Self { inner: Map::new() }
    }
    lolicon_api_builder_fn!(r18, "r18" = 1);
    lolicon_api_builder_fn!(no_r18, "r18" = 0);
    lolicon_api_builder_fn!(exclude_ai, "excludeAI" = false);

    lolicon_api_builder_fn!(tag, "tag" => Vec<String>);
    //有 num() 就使用 get_array()
    lolicon_api_builder_fn!(num, "num" => i8);
    lolicon_api_builder_fn!(uid, "uid" => i32);
    lolicon_api_builder_fn!(keyword, "keyword" => &str);

    pub fn build(&self) -> LoliconApiBuilder {
        self.clone()
    }

    pub fn build_value(&self) -> Value {
        Value::from(self.inner.clone())
    }

    pub async fn get(&self) -> BotResult<LoliconApiStruct> {
        let value = self.build_value();
        let result = http_post_json(LOLICOM_API_URI, &value).await?;
        let result = serde_json::from_str::<Value>(result.as_str())?;
        to_setu(result)
    }

    pub async fn get_array(&self) -> BotResult<Vec<LoliconApiStruct>> {
        let value = self.build_value();
        let result = http_post_json(LOLICOM_API_URI, &value).await?;
        let result = serde_json::from_str::<Value>(result.as_str())?;
        to_setu_array(result)
    }
}

fn to_setu(data: Value) -> BotResult<LoliconApiStruct> {
    match serde_json::from_value::<LoliconApiStruct>(data["data"][0].clone()) {
        Ok(mut setu) => {
            setu.urls.original = setu.urls.original.replace("i.pixiv.re", "pixiv.rco.ink");
            Ok(setu)
        }
        Err(_) => Err(BotError::from("没有这种色图喵...")),
    }
}

fn to_setu_array(data: Value) -> BotResult<Vec<LoliconApiStruct>> {
    let mut vec = vec![];
    let data = data["data"].as_array().unwrap().clone();
    for v in data {
        match serde_json::from_value::<LoliconApiStruct>(v) {
            Ok(mut lolicon) => {
                lolicon.urls.original =
                    lolicon.urls.original.replace("i.pixiv.re", "pixiv.rco.ink");
                vec.push(lolicon);
            }
            Err(_) => {
                return Err(BotError::from("没有这种色图喵..."));
            }
        };
    }
    Ok(vec)
}

#[test]
fn build() {
    // let mut builder = LoliconApiBuilder::new();
    // builder.tag(vec!["1"]);
    // builder.r18();
    // builder.num(4);
    // builder.uid(2314145);
    // builder.keyword("sdoafhs");
    // builder.exclude_ai();
    let binding = LoliconApiBuilder::new()
        .tag(vec!["1".to_string()])
        .r18()
        .num(4)
        .uid(2314145)
        .keyword("sdoafhs")
        .exclude_ai()
        .build_value();
    let json = json!({
        "tag":["1"],
        "r18":1,
        "num":4,
        "uid":2314145,
        "keyword":"sdoafhs",
        "excludeAI":false,
    });
    assert_eq!(binding, json);
}
