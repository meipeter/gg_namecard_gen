# gg_namecard_gen

Generate a name card using data from puddle.farm.
![example](./postcard.png)

(namecard of bilibili@ON_SELLING)[not me]
## Example

```rust
use gg_namecard_gen::{generate_gg_namecard, PlayerResponse};
use anyhow::Result;

fn main() -> Result<()> {
    let p: PlayerResponse = serde_json::from_str(include_str!("../240608152606560723.json"))?;
    let a = image::open("./avtar.png")?.into_rgba8();
    let img = generate_gg_namecard(p, a)?;
    img.save("postcard.png")?;
    Ok(())
}
```
