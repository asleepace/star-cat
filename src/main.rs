mod player;
use macroquad::prelude::*;
use player::Player;

#[macroquad::main("star-cat")]
async fn main() {
    // instantiate the player
    let mut player = Player::new();

    // run the game loop
    loop {
        clear_background(BLACK);

        player.update(get_frame_time());

        next_frame().await
    }
}
