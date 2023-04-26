use chrono::{Datelike, NaiveDateTime};
use etternaonline_api::v2::UserDetails;
use og_image_writer::img::ImageInputFormat;
use og_image_writer::style;
use og_image_writer::writer::OGImageWriter;
use proc_qq::re_exports::{anyhow, serde_json};
use crate::image::{file_to_image, MSYHBD};
use crate::resource_path;


pub struct EttUserInfoImage {
    user_data: UserDetails,
    info_history_record: Option<String>,
    time: NaiveDateTime,
    image_writer: OGImageWriter,
}


impl EttUserInfoImage {
    pub fn new(user_data: UserDetails, info_history_record: Option<String>, time: NaiveDateTime) -> Self {
        Self {
            user_data,
            info_history_record,
            time,
            image_writer: OGImageWriter::new(style::WindowStyle {
                width: 1400,
                height: 880,
                background_color: Some(style::Rgba([75, 0, 75, 255])),
                align_items: style::AlignItems::Start,
                justify_content: style::JustifyContent::Start,
                ..style::WindowStyle::default()
            }).unwrap(),
        }
    }

    pub fn build(&mut self, avatars: &Vec<u8>) -> anyhow::Result<Vec<u8>> {
        //设置 username country_code
        if self.user_data.player_rating == 0.0 {
            return Err(anyhow::Error::msg("rating为0喵... 获取数据没有意义喵..."))
        }
        self.image_writer.set_text(
            format!("{}  -  {}", self.user_data.username, self.user_data.country_code).as_str(),
            style::Style {
                margin: style::Margin(40, 0, 0, 80),
                position: style::Position::Absolute,
                color: style::Rgba([255, 255, 255, 255]),
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
            300, 300,
            image_f,
            style::Style {
                margin: style::Margin(80, 0, 0, 1040),
                position: style::Position::Absolute,
                border_radius: style::BorderRadius(20, 20, 20, 20),
                ..style::Style::default()
            },
        )?;
        let v = serde_json::from_str::<serde_json::Value>(self.info_history_record.clone().unwrap_or_default().as_str()).unwrap_or_default();
        if self.info_history_record.is_some() {

            //设置 player_rating
            let mut rating_ = 0.00;
            if self.user_data.player_rating > (v["overall"].as_f64().unwrap_or(0.00) as f32) {
                rating_ = self.user_data.player_rating - v["overall"].as_f64().unwrap_or(0.00) as f32
            } else if self.user_data.player_rating == (v["overall"].as_f64().unwrap_or(0.00) as f32) {}
            self.image_writer.set_text(
                format!("(+{rating_:.2})").as_str(),
                style::Style {
                    margin: style::Margin(500, 0, 0, 1100),
                    position: style::Position::Absolute,
                    color: style::Rgba([255, 0, 0, 255]),
                    font_size: 60.,
                    ..style::Style::default()
                },
                Some(Vec::from(MSYHBD)),
            )?;
        }

        self.image_writer.set_text(
            format!("{:.2}", self.user_data.player_rating).as_str(),
            style::Style {
                margin: style::Margin(430, 0, 0, 1100),
                position: style::Position::Absolute,
                color: style::Rgba([255, 0, 0, 255]),
                font_size: 80.,
                ..style::Style::default()
            },
            Some(Vec::from(MSYHBD)),
        )?;

        self.image_writer.set_text(
            "overall",
            style::Style {
                margin: style::Margin(390, 0, 0, 1130),
                position: style::Position::Absolute,
                color: style::Rgba([0, 0, 0, 255]),
                font_size: 40.,
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
                    position: style::Position::Absolute,
                    color: style::Rgba([255, 255, 255, 255]),
                    font_size: 50.,
                    ..style::Style::default()
                },
                Some(Vec::from(MSYHBD)),
            )?;
        }


        self.set_rating(v)?;
        self.set_rating_background()?;
        // 设置一个获取数据时间
        let time = chrono::Local::now();
        let time_ = time.day() - self.time.day();
        self.image_writer.set_text(
            format!("update time: {}  与{time_}天之前做对比", time).as_str(),
            style::Style {
                margin: style::Margin(838, 0, 0, 40),
                position: style::Position::Absolute,
                color: style::Rgba([0, 0, 0, 255]),
                font_size: 32.,
                ..style::Style::default()
            },
            Some(Vec::from(MSYHBD)),
        )?;
        // 生成图片
        let path = resource_path!("tmp","EttUserInfoImage.png");
        self.image_writer.generate(path.as_str().as_ref())?;
        let vec = file_to_image(path)?;
        Ok(vec)
    }
    /// 设置 rating
    fn set_rating(&mut self, v: serde_json::Value) -> anyhow::Result<()> {
        let vec = vec![
            (self.user_data.rating.stream,
             v["stream"].as_f64(),
             style::Rgba([125, 107, 145, 255])),
            (self.user_data.rating.jumpstream,
             v["jumpstream"].as_f64(),
             style::Rgba([132, 129, 219, 255])),
            (self.user_data.rating.handstream,
             v["handstream"].as_f64(),
             style::Rgba([153, 95, 163, 255])),
            (self.user_data.rating.stamina,
             v["stamina"].as_f64(),
             style::Rgba([242, 181, 250, 255])),
            (self.user_data.rating.jackspeed,
             v["jackspeed"].as_f64(),
             style::Rgba([108, 150, 157, 255])),
            (self.user_data.rating.chordjack,
             v["chordjack"].as_f64(),
             style::Rgba([165, 248, 211, 255])),
            (self.user_data.rating.technical,
             v["technical"].as_f64(),
             style::Rgba([176, 206, 194, 255])),
        ];
        for (i, (rating, history_rating, color)) in vec.iter().enumerate() {
            let mut rating_w = (*rating as i32 * 12);
            let mut rating_ = 0.00;
            if rating > &(history_rating.unwrap_or(0.00) as f32) {
                rating_ = rating - history_rating.unwrap_or(0.00) as f32
            } else if rating == &(history_rating.unwrap_or(0.00) as f32) {}

            if rating_w < 70 {
                rating_w = rating_w + 70;
            }

            let mut stream = OGImageWriter::new(style::WindowStyle {
                width: rating_w as u32,
                height: 50,
                background_color: Some(*color),
                align_items: style::AlignItems::Center,
                justify_content: style::JustifyContent::Center,

                ..style::WindowStyle::default()
            })?;
            stream.set_text(
                format!("{rating:.2}", ).as_str(),
                style::Style {
                    color: style::Rgba([0, 0, 0, 255]),
                    font_size: 30.,
                    text_align: style::TextAlign::Center,
                    word_break: style::WordBreak::Normal,
                    ..style::Style::default()
                },
                Some(Vec::from(MSYHBD)),
            )?;
            if self.info_history_record.is_some() {
                stream.set_text(
                    format!(" (+{rating_:.2})", ).as_str(),
                    style::Style {
                        color: style::Rgba([0, 0, 0, 255]),
                        font_size: 25.,
                        text_align: style::TextAlign::End,
                        word_break: style::WordBreak::Normal,
                        ..style::Style::default()
                    },
                    Some(Vec::from(MSYHBD)),
                )?;
            }

            self.image_writer.set_container(
                &mut stream,
                style::Style {
                    margin: style::Margin((220 + (i * 78)) as i32, 0, 0, 360),
                    position: style::Position::Absolute,
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