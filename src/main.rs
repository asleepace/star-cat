mod asteroid;
mod game;
mod player;
mod powerup;
mod stars;

#[macroquad::main("star-cat")]
async fn main() {
    game::main().await
}
