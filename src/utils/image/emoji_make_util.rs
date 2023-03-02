use std::str::FromStr;
use og_image_writer::{style};
use og_image_writer::img::ImageInputFormat;
use og_image_writer::writer::OGImageWriter;
use crate::{BotError, BotResult};
use crate::utils::file_util::{file_tmp_random_image_path, get_resources_path};
use crate::utils::image::{file_to_image, MSYHBD};


pub fn long1_emoji_make_image(text: &str) -> BotResult<Vec<u8>> {
    if text.chars().count() > 22 {
        return Err(BotError::from("字符最多22个喵!"));
    }

    let mut writer = OGImageWriter::new(
        style::WindowStyle {
            width: 675,
            height: 609,
            background_color: Some(style::Rgba([255, 255, 255, 255])),
            ..style::WindowStyle::default()
        })?;

    writer.set_img(
        format!("{}/↑_long.png", get_resources_path(vec!["image", "emoji"])).as_str(),
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
            height: 200,
            background_color: Some(style::Rgba([255, 255, 255, 255])),
            align_items: style::AlignItems::Center,
            justify_content: style::JustifyContent::Center,
            ..style::WindowStyle::default()
        })?;
    content.set_text(
        text,
        style::Style {
            color: style::Rgba([0, 0, 0, 255]),
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
            line_height: 3.,
            margin: style::Margin(428, 0, 0, 0),
            position: style::Position::Absolute,
            ..style::Style::default()
        })?;
    let string = file_tmp_random_image_path("long1", "png", &[]);
    writer.generate(string.as_str().as_ref())?;
    let vec = file_to_image(string)?;
    Ok(vec)
}