use proc_qq::{MessageChainAppendTrait, MessageChainParseTrait, MessageSendToSourceTrait, TextEleParseTrait};
use proc_qq::re_exports::image::EncodableLayout;
use proc_qq::re_exports::serde_json;


pub mod msg_util;
pub mod http_util;

pub trait ToJson<T> {
    fn struct_to_json(_: &T) -> String;
}

impl<T: proc_qq::re_exports::serde::Serialize> ToJson<T> for String {
    fn struct_to_json(data: &T) -> String {
        serde_json::to_string(data).unwrap_or_default()
    }
}

#[macro_export]
macro_rules! resource_path {
    ($($path:expr),*) => {{
        let mut path = String::new();
        path.push_str("./resources");
        $(
        path.push_str(&format!("/{}",$path));
        )*
        path
    }};
}

#[macro_export]
macro_rules! resource_tmp_path {
    ($($path:expr),* => $name:expr) => {{
        let mut path = String::new();
        path.push_str("./resources");
        path.push_str("/tmp");
        $(
        path.push_str(&format!("/{}",$path));
        )*
        path.push_str(&format!("/{}_{}",rbatis::rbdc::uuid::Uuid::new().0,$name));
        path
    }};
}


#[test]
fn test() {}
