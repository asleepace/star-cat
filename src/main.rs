mod asteroid;
mod player;
mod star;
use asteroid::Asteroid;
use macroquad::prelude::*;
use player::Player;
use star::Star;

#[macroquad::main("star-cat")]
async fn main() {
    let font = load_ttf_font("res/VT323-Regular.ttf").await.unwrap();
    // instantiate the player
    let mut player = Player::new();
    let mut asteroids = Vec::<Asteroid>::new();
    let mut star = Star::new();
    let mut speed = 200f32;
    let mut prev_score = 0f32;
    let mut next_score = 0f32;
    let mut tick = 0f32;

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
            next_score += 100f32;
            player.powerup();
            star.powerup();
        }

        for asteroid in asteroids.iter_mut() {
            asteroid.update(speed, get_frame_time());
            if player.collision(&asteroid.rect, true) {
                next_score += 1f32;
            }
        }

        if prev_score < next_score {
            prev_score += 1f32;
        }

        tick += 1f32;
        if tick > 10f32 {
            tick = 0f32;
            next_score += 1f32;
        }

        let score_text = format!("SCORE: {}", prev_score);
        let score_text_dim = measure_text(&score_text, Some(font), 40u16, 1.0);
        draw_text_ex(
            &score_text,
            screen_width() * 0.5f32 - score_text_dim.width * 0.5f32,
            40.0,
            TextParams {
                font,
                font_size: 40u16,
                color: WHITE,
                ..Default::default()
            },
        );

        next_frame().await
    }
}
