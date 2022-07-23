mod asteroid;
mod player;
use asteroid::Asteroid;
use macroquad::prelude::*;
use player::Player;

#[macroquad::main("star-cat")]
async fn main() {
    // instantiate the player
    let mut player = Player::new();
    let mut asteroids = Vec::<Asteroid>::new();

    for i in 0..3 {
        asteroids.push(Asteroid::new(i));
    }

    // run the game loop
    loop {
        clear_background(BLACK);
        player.update(get_frame_time());

        for asteroid in asteroids.iter_mut() {
            asteroid.update(get_frame_time());
            player.collision(&asteroid.rect);
        }

        next_frame().await
    }
}
