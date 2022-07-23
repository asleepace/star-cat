mod asteroid;
mod player;
mod star;
use asteroid::Asteroid;
use macroquad::prelude::*;
use player::Player;
use star::Star;

#[macroquad::main("star-cat")]
async fn main() {
    // instantiate the player
    let mut player = Player::new();
    let mut asteroids = Vec::<Asteroid>::new();
    let mut star = Star::new();
    let mut speed = 200f32;

    for i in 0..3 {
        asteroids.push(Asteroid::new(i));
    }

    // run the game loop
    loop {
        clear_background(BLACK);
        player.update(get_frame_time());
        star.update(speed, get_frame_time());

        if player.collision(&star.rect, false) {
            speed += 20f32 * get_frame_time();
            player.powerup();
            star.powerup();
        }

        for asteroid in asteroids.iter_mut() {
            asteroid.update(speed, get_frame_time());
            player.collision(&asteroid.rect, true);
        }

        next_frame().await
    }
}
