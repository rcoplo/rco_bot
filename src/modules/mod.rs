use proc_qq::{Module};


mod forwarding;
mod h;
mod tools;
mod osu;
mod entertainment;
mod ett;

pub use forwarding::bili::BiliPushTask;
pub fn all_modules() -> Vec<Module> {
    vec![
        entertainment::emoji_make::module(),
        entertainment::sign::module(),
        forwarding::bili::module(),
        ett::ett::module(),
        h::setu::module(),
        // basic_modules::help::module(),
        tools::group::module(),
        tools::mc_server_status::module(),
        tools::test_1::module(),
    ]
}


