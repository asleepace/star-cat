use wasm_bindgen::prelude::*;

mod asteroid;
mod game;
mod player;
mod powerup;
mod stars;

#[wasm_bindgen(start)]
pub async fn start() {
    game::main().await
}