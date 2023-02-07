use std::path::Path;
use og_image_writer::{ImageOutputFormat, style, TextArea};
use og_image_writer::writer::OGImageWriter;
use proc_qq::re_exports::bytes::Bytes;
use proc_qq::re_exports::ricq_core::RQError;
use tokio::io::AsyncReadExt;
use crate::BotResult;
use crate::msg_util::text;
use crate::utils::file_util::file_tmp_random_image_path;
use std::io::{BufReader, Cursor, Read};
use image::ImageFormat;
use crate::utils::image::{file_to_image, MSYHBD};

pub fn help_module_image(help:&Vec<String>) -> BotResult<Vec<u8>>{
    let mut writer = OGImageWriter::new(style::WindowStyle {
        width: 1024,
        height: 512,
        background_color: Some(style::Rgba([70, 40, 90, 255])),
        align_items: style::AlignItems::Start,
        justify_content: style::JustifyContent::Start,
        ..style::WindowStyle::default()
    })?;

    for text in help {
        writer.set_text(
            text.as_str(),
            style::Style {
                margin: style::Margin(0, 20, 0, 20),
                line_height: 5.,
                font_size: 35.,
                word_break: style::WordBreak::Normal,
                color: style::Rgba([255, 255, 255, 255]),
                text_align: style::TextAlign::Center,
                ..style::Style::default()
            },
            Some(Vec::from(MSYHBD)),
        );
    }
    let string = file_tmp_random_image_path("help_module_image", "png", &[]);
    writer.generate(string.as_str().as_ref())?;
    let vec = file_to_image(string)?;
    Ok(vec)
}

