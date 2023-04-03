use proc_qq::{Module};


mod forwarding;
mod h;
mod tools;
mod osu;
mod entertainment;
mod ett;


pub fn all_modules() -> Vec<Module> {
    vec![
        entertainment::emoji_make::module(),
        entertainment::sign::module(),
        ett::ett::module(),
        h::setu::module(),
        // basic_modules::help::module(),
        tools::group::module(),
        tools::mc_server_status::module(),
    ]
}
#[macro_export]
macro_rules! command_to_vec {
    ($command:expr) => {
        $command.split_whitespace().collect::<Vec<String>>()
    };
}

