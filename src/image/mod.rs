use std::io::Read;
use once_cell::sync::Lazy;
use proc_qq::re_exports::ricq_core::RQError;
use resvg::{tiny_skia, usvg};
use resvg::usvg::{fontdb, Size, TreeParsing, TreeTextToPath};
use crate::BotResult;

pub mod help_image_util;
pub mod ett;
pub mod emoji_make_util;


pub static MSYHBD: &[u8] = include_bytes!("../../resources/font/MSYHBD.TTC");
static LOLITI: &[u8] = include_bytes!("../../resources/font/萝莉体.ttc");


fn file_to_image(path: String) -> BotResult<Vec<u8>> {
    let mut f = std::fs::File::open(path).map_err(RQError::IO)?;
    let mut b = vec![];
    f.read_to_end(&mut b).map_err(RQError::IO)?;
    Ok(b)
}

fn svg_to_png(data: String, path: &str) -> BotResult<()> {
    let mut opt = usvg::Options::default();
    opt.default_size = Size::new(600., 425.).unwrap();
    let tree = {
        let mut tree = usvg::Tree::from_str(&data, &opt).unwrap();
        let mut db = fontdb::Database::new();
        db.load_system_fonts();
        // db.set_sans_serif_family("msyhbd");
        tree.convert_text(&db);
        tree
    };

    let fit_to = resvg::FitTo::Original;
    let size = fit_to.fit_to(tree.size.to_screen_size()).unwrap();
    let mut pixmap = tiny_skia::Pixmap::new(size.width(), size.height()).unwrap();
    resvg::render(
        &tree,
        fit_to,
        tiny_skia::Transform::default(),
        pixmap.as_mut(),
    ).expect("转换png错误!");
    pixmap.save_png(path);
    Ok(())
}