use std::sync::Arc;
use once_cell::sync::Lazy;
use proc_qq::Module;

mod help;
mod tools;
mod forwarding;
mod h;

static MODULES: Lazy<Arc<Vec<Module>>> = Lazy::new(||{
    Arc::new(vec![
        help::help_module(),
        h::setu::setu_module(),
        forwarding::bili::module(),
])});

pub fn all_modules() -> Arc<Vec<Module>> {
    MODULES.clone()
}
