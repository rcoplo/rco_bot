use proc_qq::{event, Module, module};


static ID: &'static str = "Group";
static NAME: &'static str = "Group";

pub struct GroupHelp {
    pub mod_name: String,
    pub help_text: Vec<String>,
}

impl Default for GroupHelp {
    fn default() -> Self {
        GroupHelp {
            mod_name: "Group".to_string(),
            help_text: vec![
                "group_help",
                "----------------------------------------------------------------",
            ].iter().map(|str| str.to_string()).collect::<Vec<_>>(),
        }
    }
}

pub(crate) fn module() -> Module {
    module!(
        ID,
        NAME,
    )
}
