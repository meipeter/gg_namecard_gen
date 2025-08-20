use crate::lib::generate_gg_namecard;
use anyhow::Result;
use puddle_farm_api_client_openapi_client::models::PlayerResponse;
#[allow(unused, unused_variables)]
mod lib;
fn main() -> Result<()> {
    let p: PlayerResponse = serde_json::from_str(include_str!("../240608152606560723.json"))?;
    let a = image::open("./avtar.png")?.into_rgba8();
    let img = generate_gg_namecard(p, a)?;
    img.save("postcard.png")?;
    Ok(())
}
