use std::io::Cursor;

use crate::lib::generate_gg_namecard;
use anyhow::Result;
use clap::Parser;
use image::ImageReader;
use puddle_farm_api_client_openapi_client::apis::default_api::player_id_get;
use puddle_farm_api_client_openapi_client::apis::{
    configuration::Configuration, default_api::avatar_player_id_get,
};
use puddle_farm_api_client_openapi_client::models::PlayerResponse;
use tokio::runtime::{self, Runtime};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    id: i64,
}

mod lib;
fn main() -> Result<()> {
    let rt = runtime::Builder::new_current_thread()
        .enable_all()
        .build()?;
    let id = Cli::parse().id;
    let c = Configuration::default();
    let p = rt.block_on(async { player_id_get(&c, id).await })?;
    let res = rt.block_on(async { avatar_player_id_get(&c, id).await.unwrap().bytes().await })?;
    let a = ImageReader::new(Cursor::new(res))
        .with_guessed_format()?
        .decode()?
        .to_rgba8();
    let img = generate_gg_namecard(p, a)?;
    img.save("./ggnamecard.png")?;
    Ok(())
}
