
use og_image_writer::{style};
use og_image_writer::writer::OGImageWriter;
use crate::{BotError, BotResult, resource_path, resource_tmp_path};
use crate::utils::image::{file_to_image, MSYHBD};


pub fn long1_emoji_make_image(text: &str) -> BotResult<Vec<u8>> {
    let mut text = text.replace(".n", "\n");
    let vec = text.split(".n").collect::<Vec<_>>();
    let mut num = 0;
    for str in vec {
        let mut text_num = 0.;
        str.chars().for_each(|c| {
            if c.len_utf8() == 3 {
                text_num = text_num + 0.1;
            } else if c.len_utf8() < 3 {
                text_num = text_num + 0.05;
            }
        });
        if text_num >= 1. {
            num = num + (text_num as u32 * 84) as u32
        }
        num = num + 80
    }
    if num < 200 {
        num = 200;
    }
    let mut writer = OGImageWriter::new(
        style::WindowStyle {
            width: 675,
            height: 409 + num,
            background_color: Some(style::Rgba([255, 255, 255, 255])),
            ..style::WindowStyle::default()
        })?;

    writer.set_img(
        resource_path!("image","emoji" => "long1.png").unwrap_or_default().as_str(),
        675,
        609,
        style::Style {
            position: style::Position::Absolute,
            margin: style::Margin(0, 0, 0, 0),
            ..style::Style::default()
        })?;

    let mut content = OGImageWriter::new(
        style::WindowStyle {
            width: 673,
            height: num,
            align_items: style::AlignItems::Center,
            justify_content: style::JustifyContent::Center,
            ..style::WindowStyle::default()
        })?;
    content.set_text(
        text.as_str(),
        style::Style {
            color: style::Rgba([0, 0, 0, 255]),
            line_height: 2.,
            font_size: 80.,
            text_align: style::TextAlign::Center,
            white_space: style::WhiteSpace::PreLine,
            word_break: style::WordBreak::BreakAll,
            ..style::Style::default()
        },
        Some(Vec::from(MSYHBD)),
    )?;
    writer.set_container(
        &mut content,
        style::Style {
            margin: style::Margin(428, 0, 0, 0),
            position: style::Position::Absolute,
            ..style::Style::default()
        })?;

    match resource_tmp_path!("emoji" => "long1.png") {
        None => { Err(BotError::from("获取路径失败喵...")) }
        Some(path) => {
            writer.generate(path.as_str().as_ref())?;
            let vec = file_to_image(path)?;
            Ok(vec)
        }
    }
}