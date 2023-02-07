use std::path::Path;
use etternaonline_api::v2::UserDetails;
use og_image_writer::img::ImageInputFormat;
use og_image_writer::style;
use og_image_writer::writer::OGImageWriter;
use crate::BotResult;
use crate::utils::file_util::file_tmp_random_image_path;
use crate::utils::http_util::http_get_image;
use crate::utils::image::{file_to_image, MSYHBD};


pub struct EttUserInfoImage{
    user_data:UserDetails,
    image_writer:OGImageWriter,
}


impl EttUserInfoImage {
    pub fn new(user_data:UserDetails) -> Self{
        Self{
            user_data,
            image_writer:OGImageWriter::new(style::WindowStyle {
                width: 1400,
                height: 880,
                background_color: Some(style::Rgba([75, 0, 75, 255])),
                align_items: style::AlignItems::Start,
                justify_content: style::JustifyContent::Start,
                ..style::WindowStyle::default()
            }).unwrap()
        }
    }

    pub fn ok(&mut self,avatars:&Vec<u8>) -> anyhow::Result<Vec<u8>>{
        //设置 username country_code
        if self.user_data.player_rating == 0.0{
            return Err(anyhow::Error::msg("rating为0喵... 获取数据没有意义喵..."))
        }
        self.image_writer.set_text(
            format!("{}  -  {}",self.user_data.username,self.user_data.country_code).as_str(),
            style::Style{
                margin: style::Margin(40,0,0,80),
                position:style::Position::Absolute,
                color:style::Rgba([255,255,255,255]),
                font_size:100.,
                ..style::Style::default()
            },
            Some(Vec::from(MSYHBD)),
        )?;
        //设置 头像
        let image_f = match self.user_data.avatar_url.split(".").collect::<Vec<_>>()[1] {
            "png" => ImageInputFormat::Png,
            _ => ImageInputFormat::Jpeg,
        };
        self.image_writer.set_img_with_data(
            avatars.as_slice(),
            300,300,
            image_f,
            style::Style {
                margin: style::Margin(80,0,0,1040),
                position:style::Position::Absolute,
                border_radius: style::BorderRadius(20,20,20,20),
                ..style::Style::default()
            },
        )?;

        //设置 player_rating
        self.image_writer.set_text(
            format!("{:.2}",self.user_data.player_rating).as_str(),
            style::Style{
                margin: style::Margin(420,0,0,1100),
                position:style::Position::Absolute,
                color:style::Rgba([255,0,0,255]),
                font_size:86.,
                ..style::Style::default()
            },
            Some(Vec::from(MSYHBD)),
        )?;
        self.image_writer.set_text(
            "overall",
            style::Style{
                margin: style::Margin(390,0,0,1130),
                position:style::Position::Absolute,
                color:style::Rgba([0,0,0,255]),
                font_size:40.,
                ..style::Style::default()
            },
            Some(Vec::from(MSYHBD)),
        )?;
        let vec = vec![
            "Stream",
            "JumpStream",
            "HandStream",
            "stamina",
            "JackSpeed",
            "ChordJack",
            "Technical",
        ];
        for (i,str) in vec.iter().enumerate() {
            self.image_writer.set_text(
                str,
                style::Style{
                    margin: style::Margin((220 + (i * 78)) as i32, 0, 0, 120),
                    position:style::Position::Absolute,
                    color:style::Rgba([255,255,255,255]),
                    font_size:50.,
                    ..style::Style::default()
                },
                Some(Vec::from(MSYHBD)),
            )?;
        }

        self.set_rating()?;
        self.set_rating_background()?;
        // 设置一个获取数据时间
        let time = chrono::Local::now();

        self.image_writer.set_text(
            format!("update time: {}", time).as_str(),
            style::Style{
                margin: style::Margin(838, 0, 0, 40),
                position:style::Position::Absolute,
                color:style::Rgba([0,0,0,255]),
                font_size:32.,
                ..style::Style::default()
            },
            Some(Vec::from(MSYHBD)),
        )?;
        // 生成图片
        let string = file_tmp_random_image_path("EttUserInfoImage", "png", &[]);
        self.image_writer.generate(Path::new(&string))?;
        let vec = file_to_image(string)?;
        Ok(vec)

    }
    /// 设置 rating
    fn set_rating(&mut self) ->  anyhow::Result<()>{
        let vec= vec![
            (self.user_data.rating.stream,style::Rgba([125,107,145, 255])),
            (self.user_data.rating.jumpstream,style::Rgba([132,129,219, 255])),
            (self.user_data.rating.handstream,style::Rgba([153,95,163, 255])),
            (self.user_data.rating.stamina,style::Rgba([242,181,250, 255])),
            (self.user_data.rating.jackspeed,style::Rgba([108,150,157, 255])),
            (self.user_data.rating.chordjack,style::Rgba([165,248,211, 255])),
            (self.user_data.rating.technical,style::Rgba([176,206,194, 255])),
        ];
        for (i,(rating,color)) in vec.iter().enumerate() {
            let mut rating_w = (*rating as i32 * 12);
            tracing::debug!("{}", rating_w);
            if rating_w < 50 {
                rating_w = rating_w + 50;
            }
            tracing::debug!("{}", rating);
            tracing::debug!("{}", rating_w);
            let mut stream = OGImageWriter::new(style::WindowStyle {
                width: rating_w as u32,
                height: 50,
                background_color: Some(*color),
                align_items: style::AlignItems::Center,
                justify_content: style::JustifyContent::Center,

                ..style::WindowStyle::default()
            })?;
            stream.set_text(
                format!("{:.2}",rating).as_str(),
                style::Style{
                    color:style::Rgba([0,0,0,255]),
                    font_size:30.,
                    text_align: style::TextAlign::Center,
                    word_break:style::WordBreak::Normal,
                    ..style::Style::default()
                },
                Some(Vec::from(MSYHBD)),
            )?;
            self.image_writer.set_container(
                &mut stream,
                style::Style{
                    margin: style::Margin((220 + (i * 78)) as i32,0,0,360),
                    position:style::Position::Absolute,
                    border_radius: style::BorderRadius(0, 10, 10, 0),
                    ..style::Style::default()
                },
            )?;
        }

        Ok(())
    }
    fn set_rating_background(&mut self,) -> anyhow::Result<()>{
        let mut rating_background = OGImageWriter::new(style::WindowStyle {
            width: 920,
            height: 600,
            background_color: Some(style::Rgba([255,255,255, 50])),
            align_items: style::AlignItems::Center,
            justify_content: style::JustifyContent::Center,
            ..style::WindowStyle::default()
        })?;

        self.image_writer.set_container(
            &mut rating_background,
            style::Style{
                margin: style::Margin(180,0,0,80),
                position:style::Position::Absolute,
                border_radius: style::BorderRadius(20, 20, 20, 20),
                ..style::Style::default()
            },
        )?;
        Ok(())
    }
}