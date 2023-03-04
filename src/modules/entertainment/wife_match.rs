static ID: &'static str = "wife_match";
static NAME: &'static str = "wife_match";


pub struct WifeMatchHelp {
    pub mod_name: String,
    pub help_text: Vec<String>,
}

impl Default for WifeMatchHelp {
    fn default() -> Self {
        WifeMatchHelp {
            mod_name: "wife".to_string(),
            help_text: vec![
                "wife_match help",
                "指令:",
                "       找老婆",
                "       分手",
                "       上床",
            ].iter().map(|str| str.to_string()).collect::<Vec<_>>(),
        }
    }
}