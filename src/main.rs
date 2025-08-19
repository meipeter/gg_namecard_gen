#[allow(unused, unused_variables)]
use ab_glyph::{FontArc, PxScale};
use anyhow::{Ok, Result};
use image::{Rgb, RgbImage};
use imageproc::drawing::{draw_filled_rect_mut, draw_text_mut, text_size};
use imageproc::rect::Rect;
use puddle_farm_api_client_openapi_client::models::PlayerResponse;
use std::fmt::format;

fn main() -> Result<()> {
    let red = Rgb([55, 55, 37]);
    let gray = Rgb([38, 42, 44]);
    let p: PlayerResponse = serde_json::from_str(include_str!("../240608152606560723.json"))?;
    let font = FontArc::try_from_slice(include_bytes!("../unifont-16.0.04.otf"))?;
    let mut chara_string_vec = Vec::new();
    //这一段是拆分小字列表
    for i in p
        .clone()
        .ratings
        .ok_or(anyhow::Error::msg("empty player response"))?
    {
        chara_string_vec.push(format!(
            "{} {} ±{}",
            i.character.unwrap_or("".to_string()),
            i.rating.unwrap_or(0.0),
            i.deviation.unwrap_or(0.0)
        ));
        chara_string_vec.push(format!("({} games)", i.match_count.unwrap_or(0)));
        chara_string_vec.push("  ".to_string());
    }
    // println!("{:?}", chara_string_vec);

    //这一段是大字字符串
    let the_fist_chara = p.ratings.unwrap_or_default()[0].clone();
    let the_fist_chara = &format!(
        "{} Rating: {:.0}±{:.0} ({} games)",
        the_fist_chara.character.unwrap_or("".to_string()),
        the_fist_chara.rating.unwrap_or(0.0),
        the_fist_chara.deviation.unwrap_or(0.0),
        the_fist_chara.match_count.unwrap_or(0),
    );
    let fist_lint_wideth = text_size(PxScale::from(30.0), &font, the_fist_chara).0 + 10;
    let mut img = RgbImage::from_pixel(fist_lint_wideth, 512, gray);
    let mut x = 0;
    let mut y = 50;
    //画大字
    draw_text_mut(
        &mut img,
        Rgb([255, 255, 255]),
        x + 10,
        y,
        PxScale::from(30.0),
        &font,
        &the_fist_chara,
    );
    y = y + 40;

    //画charator小字
    draw_text_mut(
        &mut img,
        Rgb([255, 255, 255]),
        x + 30,
        y,
        PxScale::from(20.0),
        &font,
        "Characters:",
    );
    y = y + 40;

    //画小字
    for line in chara_string_vec {
        draw_text_mut(
            &mut img,
            Rgb([255, 255, 255]),
            x + 50,
            y,
            PxScale::from(20.0),
            &font,
            &line,
        );
        y = y + 20;
    }

    img.save("postcard.png")?;
    Ok(())
}
