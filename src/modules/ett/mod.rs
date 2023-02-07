use og_image_writer::font_context::FontContext;
use crate::modules::entertainment::sign::SignHelp;
use crate::utils::image::MSYHBD;

pub mod ett_user_info;




pub struct EttHelp{
    pub mod_name:String,
    pub help_text:Vec<String>,
}

impl Default for EttHelp{
    fn default() -> Self {
        EttHelp{
            mod_name: "ett".to_string(),
            help_text: vec![
                "ett help",
                "----------------------------------------------------------------",
                "/ett  build  ett用户名        //绑定账号",
                "/ett  untie        //解绑账号",
                "/ett  set  background  <图片url> todo",
                "/ett  info      //查询绑定的用户信息",
                "/ett  info  <用户名> //查询用户信息",
                "----------------------------------------------------------------",
            ].iter().map(|str| str.to_string()).collect::<Vec<_>>(),
        }
    }
}
