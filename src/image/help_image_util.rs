use og_image_writer::{style, TextArea};
use og_image_writer::writer::OGImageWriter;
use crate::BotResult;


// pub fn help_module_image(module_setting:&ModuleSetting) -> BotResult<Vec<u8>> {
//     let i = module_setting.usage.split("\n").collect::<Vec<_>>().len();
//     //初始化背景
//     let width = format!("[Module Name]: {}            [Author]: {}            [Version]: {}",
//                         module_setting.name,
//                         module_setting.author,
//                         module_setting.version)
//         .len() * 13;
//     let mut writer = OGImageWriter::new(
//         style::WindowStyle {
//             width: width as u32,
//             height: (172 + (35 * i)) as u32,
//             align_items: style::AlignItems::Center,
//             justify_content: style::JustifyContent::Center,
//             background_color:Some(style::Rgba([124,251,255,255])),
//             ..style::WindowStyle::default()
//         })?;
//     writer.set_container(&mut OGImageWriter::new(style::WindowStyle{
//         width: width as u32,
//         height:2,
//         background_color:Some(style::Rgba([0,0,0,255])),
//         ..style::WindowStyle::default()
//     })?,style::Style{
//         margin:style::Margin(102,0,0,0),
//         position:style::Position::Absolute,
//         ..style::Style::default()
//     })?;
//     // 上半部分 开始:
//     let mut module_up = OGImageWriter::new(style::WindowStyle {
//         width: width as u32,
//         height: 102,
//         align_items: style::AlignItems::Center,
//         justify_content: style::JustifyContent::Center,
//         ..style::WindowStyle::default()
//     })?;
//     let mut area_up = TextArea::new();
//     area_up.push_text("[Module Name]: ");
//     area_up.push(format!("{}            ", module_setting.name).as_str(), style::Style{
//         color:style::Rgba([0,0,0,255]),
//         font_size:45.,
//         ..Default::default()
//     }, Some(Vec::from(MSYHBD)))?;
//     area_up.push_text("[Author]: ");
//     area_up.push(format!("{}            ", module_setting.author).as_str(), style::Style{
//         color:style::Rgba([0,0,0,255]),
//         font_size:45.,
//         ..Default::default()
//     }, Some(Vec::from(MSYHBD)))?;
//     area_up.push_text("[Version]: ");
//     area_up.push(format!("{}            ", module_setting.version).as_str(), style::Style{
//         color:style::Rgba([0,0,0,255]),
//         font_size:45.,
//         ..Default::default()
//     }, Some(Vec::from(MSYHBD)))?;
//     module_up.set_textarea(
//         area_up,
//         style::Style {
//             text_align: style::TextAlign::Center,
//             white_space:style::WhiteSpace::PreLine,
//             font_size:30.,
//             ..style::Style::default()
//
//         }, Some(Vec::from(MSYHBD)))?;
//
//     writer.set_container(&mut module_up,style::Style{
//         position:style::Position::Absolute,
//         ..Default::default()
//     })?;
//     // 结束
//     //下半部分
//     let mut module_down = OGImageWriter::new(style::WindowStyle {
//         width: (width - 40) as u32,
//         height: (122 + (35 * i) - 70) as u32,
//         align_items: style::AlignItems::Start,
//         justify_content: style::JustifyContent::Start,
//         // background_color:Some(style::Rgba([0,0,0,255])),
//         ..style::WindowStyle::default()
//     })?;
//     let mut area_down = TextArea::new();
//     area_down.push_text("Des:   ");
//     area_down.push(format!("{}",module_setting.des).as_str(),style::Style{
//         font_size:35.,
//         ..Default::default()
//     },Some(Vec::from(MSYHBD)))?;
//     area_down.push_text("\n\nUsage: ");
//     area_down.push(module_setting.usage,style::Style{
//         white_space:style::WhiteSpace::PreLine,
//         word_break:style::WordBreak::BreakAll,
//         font_size:35.,
//         ..Default::default()
//     },Some(Vec::from(MSYHBD)))?;
//     module_down.set_textarea(area_down,style::Style{
//         margin:style::Margin(0,0,0,0),
//         white_space:style::WhiteSpace::PreLine,
//         font_size:25.,
//         line_height:2.,
//         ..Default::default()
//     },Some(Vec::from(MSYHBD)))?;
//
//     writer.set_container(&mut module_down,style::Style{
//         margin:style::Margin(105,0,0,0),
//         ..Default::default()
//     })?;
//     // 结束
//
//     let string = file_tmp_random_image_path("help", "png", &["image","help"]);
//     writer.generate(string.as_str().as_ref())?;
//     let vec = file_to_image(string)?;
//     Ok(vec)
// }