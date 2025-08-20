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
    let config = Configuration::default();
    let player = rt.block_on(async { player_id_get(&config, id).await })?;
    let respone = rt.block_on(async {
        avatar_player_id_get(&config, id)
            .await
            .unwrap()
            .bytes()
            .await
    })?;
    let avatar = ImageReader::new(Cursor::new(respone))
        .with_guessed_format()?
        .decode()?
        .to_rgba8();
    let img = generate_gg_namecard(player, avatar)?;
    img.save("./ggnamecard.png")?;
    Ok(())
}
