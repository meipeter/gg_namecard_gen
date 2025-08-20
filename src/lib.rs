use ab_glyph::{FontArc, PxScale};
use anyhow::{Error, Ok, Result};

use image::imageops::{self, FilterType};
use image::{Rgb, RgbImage, Rgba, RgbaImage, imageops::resize};
use imageproc::{
    drawing::{draw_filled_ellipse_mut, draw_filled_rect_mut, draw_text_mut, text_size},
    rect::Rect,
};

use puddle_farm_api_client_openapi_client::models::{PlayerResponse, TopRating, top_rating};
use std::fmt::format;
fn draw_rounded_rect_mut(img: &mut RgbaImage, rect: Rect, r: i32, color: Rgba<u8>) {
    let (x, y) = (rect.left(), rect.top());
    let (w, h) = (rect.width() as i32, rect.height() as i32);

    /* ---------- 1. 先画 4 个圆角圆 ---------- */
    // 左上角
    draw_filled_ellipse_mut(img, (x + r, y + r), r, r, color);
    // 右上角
    draw_filled_ellipse_mut(img, (x + w - r - 1, y + r), r, r, color);
    // 左下角
    draw_filled_ellipse_mut(img, (x + r, y + h - r - 1), r, r, color);
    // 右下角
    draw_filled_ellipse_mut(img, (x + w - r - 1, y + h - r - 1), r, r, color);

    /* ---------- 2. 再画中间矩形 ---------- */
    let inner = Rect::at(x + r, y).of_size((w - 2 * r) as u32, h as u32);
    draw_filled_rect_mut(img, inner, color);

    let inner = Rect::at(x, y + r).of_size(w as u32, (h - 2 * r) as u32);
    draw_filled_rect_mut(img, inner, color);
}
pub fn generate_gg_namecard(p: PlayerResponse, avatar: RgbaImage) -> Result<RgbaImage> {
    let red = Rgba([155, 55, 37, 255]);
    let gray = Rgba([38, 42, 44, 255]);

    let font = FontArc::try_from_slice(include_bytes!("../unifont-16.0.04.otf"))?;
    let mut chara_string_vec = Vec::new();
    //这一段是拆分小字列表
    for i in p
        .clone()
        .ratings
        .ok_or(anyhow::Error::msg("empty player response"))?
    {
        chara_string_vec.push(format!(
            "{} {:.0} ±{:.0}",
            i.character.unwrap_or("".to_string()),
            i.rating.unwrap_or(0.0),
            i.deviation.unwrap_or(0.0)
        ));
        chara_string_vec.push(format!("({} games)", i.match_count.unwrap_or(0)));
        chara_string_vec.push("  ".to_string());
    }
    // println!("{:?}", chara_string_vec);

    //这一段是大字字符串
    let the_fist_chara = p.clone().ratings.unwrap_or_default()[0].clone();
    let the_fist_chara = &format!(
        "{} Rating: {:.0}±{:.0} ({} games)",
        the_fist_chara.character.unwrap_or("".to_string()),
        the_fist_chara.rating.unwrap_or(0.0),
        the_fist_chara.deviation.unwrap_or(0.0),
        the_fist_chara.match_count.unwrap_or(0),
    );
    let mut if_rank = false;
    //大字后面的排名
    let chara_rank = p.clone().ratings.unwrap_or_default()[0].clone();
    let mut rank_string = String::new();
    if let Some(rank) = chara_rank.top_char {
        rank_string = format!(" #{} {} ", rank, chara_rank.character.unwrap_or_default());
        if rank != 0 {
            if_rank = true;
        }
    }
    let rank_wideth = text_size(PxScale::from(20.0), &font, &rank_string).0 + 20;
    //Top rating 字符串
    let mut if_top_rating = false;
    let top_rate = p.clone().ratings.unwrap_or_default()[0].clone();
    let mut top_rate_string = String::new();
    if let Some(top_rate) = top_rate.top_rating {
        top_rate_string = format!(
            "Top Rating: {:.0} ±{:.0} ({})",
            top_rate.value.unwrap_or(0.0),
            top_rate.deviation.unwrap_or(0.0),
            top_rate.timestamp.unwrap_or(String::new())
        );
        if top_rate.value != Some(0.0) {
            if_top_rating = true;
        }
    };

    let mut if_top_defeat = false;
    let top_defeat = p.clone().ratings.unwrap_or_default()[0].clone();
    let mut top_defeat_string = String::new();
    if let Some(top_defeat) = top_defeat.top_defeated {
        top_defeat_string = format!(
            "Top Defeated: {}({}) {:.0} ±{:.0} ({})",
            top_defeat.name.unwrap_or_default(),
            top_defeat.char_short.unwrap_or_default(),
            top_defeat.value.unwrap_or_default(),
            top_defeat.deviation.unwrap_or_default(),
            top_defeat.timestamp.unwrap_or_default()
        );
        if top_defeat.id != Some(0) {
            if_top_defeat = true;
        }
    }
    let bar_height = 50;
    let top_defeat_line_wideth = text_size(PxScale::from(20.0), &font, &top_defeat_string).0 + 30;
    let top_rate_line_wideth = text_size(PxScale::from(20.0), &font, &top_rate_string).0 + 30;
    let fist_lint_wideth =
        text_size(PxScale::from(30.0), &font, the_fist_chara).0 + 10 + rank_wideth;
    let img_height: u32 = 130u32 + (chara_string_vec.len() as u32 * 20u32) + bar_height + 20;
    let img_wideth = (fist_lint_wideth.max(top_rate_line_wideth)).max(top_defeat_line_wideth);
    let mut img = RgbaImage::from_pixel(img_wideth, img_height, gray);
    //画红头
    draw_filled_rect_mut(
        &mut img,
        Rect::at(0, 0).of_size(img_wideth, bar_height),
        red,
    );
    //画玩家名
    let name = format!("{}", p.name.unwrap_or_default());
    draw_text_mut(
        &mut img,
        Rgba([255, 255, 255, 255]),
        ((img_wideth - text_size(PxScale::from(40.0), &font, &name).0) / 2) as i32,
        5,
        PxScale::from(40.0),
        &font,
        &name,
    );

    let mut x = 0;
    let mut y = bar_height as i32 + 20;
    //画大字
    draw_text_mut(
        &mut img,
        Rgba([255, 255, 255, 255]),
        x + 10,
        y,
        PxScale::from(30.0),
        &font,
        &the_fist_chara,
    );
    //画排名
    if if_rank {
        draw_rounded_rect_mut(
            &mut img,
            Rect::at(
                text_size(PxScale::from(30.0), &font, &the_fist_chara).0 as i32 + 20,
                y,
            )
            .of_size(
                text_size(PxScale::from(20.0), &font, &rank_string).0 as u32,
                30,
            ),
            8,
            red,
        );
        draw_text_mut(
            &mut img,
            Rgba([255, 255, 255, 255]),
            x + text_size(PxScale::from(30.0), &font, &the_fist_chara).0 as i32 + 20,
            y + 5,
            PxScale::from(20.0),
            &font,
            &rank_string,
        );
    }
    y = y + 40;
    //画top_rating
    if if_top_rating {
        draw_text_mut(
            &mut img,
            Rgba([255, 255, 255, 255]),
            x + 30,
            y,
            PxScale::from(20.0),
            &font,
            &top_rate_string,
        );
        y = y + 20;
    }

    //画topdefeat
    if if_top_defeat {
        draw_text_mut(
            &mut img,
            Rgba([255, 255, 255, 255]),
            x + 30,
            y,
            PxScale::from(20.0),
            &font,
            &top_defeat_string,
        );
        y = y + 30;
    }
    let img_y = y;
    //画charator小字
    draw_text_mut(
        &mut img,
        Rgba([255, 255, 255, 255]),
        x + 30,
        y,
        PxScale::from(20.0),
        &font,
        "Characters:",
    );
    y = y + 30;

    //画小字
    for line in chara_string_vec {
        draw_text_mut(
            &mut img,
            Rgba([255, 255, 255, 255]),
            x + 50,
            y,
            PxScale::from(20.0),
            &font,
            &line,
        );
        y = y + 20;
    }

    //画头像
    let sidelen = ((img_wideth - 10) / 2).min(img_height - img_y as u32 - 10);

    let avatar = resize(&avatar, sidelen, sidelen, FilterType::Nearest);

    imageops::overlay(
        &mut img,
        &avatar,
        ((img_wideth - 10) / 2) as i64,
        img_y as i64,
    );
    Ok(img)
}
