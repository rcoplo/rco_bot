use etternaonline_api::v2::Session;
use og_image_writer::font_context::FontContext;
use once_cell::sync::Lazy;
use crate::{BotResult, CONTEXT};
use crate::utils::image::ett::ett_user_info_image::EttUserInfoImage;
use crate::utils::image::MSYHBD;

pub mod ett_user_info_image;


pub static ETT_CLIENT:Lazy<BotResult<Session>> = Lazy::new(||{
    let ett_config = CONTEXT.config.ett.clone();
    let cooldown = match ett_config.cooldown {
        None => 0,
        Some(time) => {
            time * 1000
        }
    };
    let timeout = match ett_config.timeout {
        None => None,
        Some(time) => {
            Some(std::time::Duration::from_millis((time * 1000) as u64))
        }
    };

    let session = Session::new_from_login(
        ett_config.uin.to_string(),
        ett_config.pwd.to_string(),
        "4406B28A97B326DA5346A9885B0C9DEE8D66F89B562CF5E337AC04C17EB95C40".to_string(),
        std::time::Duration::from_millis(cooldown as u64),
        timeout
    )?;
    Ok(session)
});
