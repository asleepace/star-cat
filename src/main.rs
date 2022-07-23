mod asteroid;
mod background_star;
mod player;
mod star;
use std::f32::consts::PI;

use asteroid::Asteroid;
use background_star::BackgroundStar;
use macroquad::prelude::*;
use macroquad_particles::{self as particles, AtlasConfig, BlendMode, Emitter, EmitterConfig};
use player::Player;
use star::Star;

fn smoke() -> particles::EmitterConfig {
    particles::EmitterConfig {
        lifetime: 1.2,
        amount: 20,
        initial_direction: vec2((270f32).sin(), (270f32).cos()),
        initial_direction_spread: 0.8f32,
        size: 2f32,
        ..Default::default()
    }
}

fn powerup() -> particles::EmitterConfig {
    particles::EmitterConfig {
        lifetime: 20.2,
        amount: 20,
        initial_direction: vec2((270f32).sin(), (270f32).cos()),
        initial_direction_spread: 0.8f32,
        size: 12f32,
        ..Default::default()
    }
}

#[macroquad::main("star-cat")]
async fn main() {
    let font = load_ttf_font("res/VT323-Regular.ttf").await.unwrap();
    let cat: Texture2D = load_texture("res/cat_graphic.png").await.unwrap();
    let smoke_texture = Image::gen_image_color(2u16, 2u16, BROWN);
    let texture = Texture2D::from_image(&smoke_texture);
    let powerup_image = Image::gen_image_color(12u16, 12u16, YELLOW);
    let powerup_texture = Texture2D::from_image(&powerup_image);

    // instantiate the player
    let mut player = Player::new(cat);
    let mut asteroids = Vec::<Asteroid>::new();
    let mut background = Vec::<BackgroundStar>::new();
    let mut star = Star::new();
    let mut speed = 200f32;
    let mut prev_score = 0f32;
    let mut next_score = 0f32;
    let mut tick = 0f32;

    let mut flying_emitter_local = Emitter::new(EmitterConfig {
        local_coords: true,
        texture: Some(texture),
        ..smoke()
    });

    let mut powerup_emitter_local = Emitter::new(EmitterConfig {
        local_coords: true,
        texture: Some(powerup_texture),
        ..powerup()
    });

    for i in 0..5 {
        asteroids.push(Asteroid::new(i));
    }

    for i in 0..100 {
        background.push(BackgroundStar::new());
    }

    // run the game loop
    loop {
        // clear background and paint background stars first
        clear_background(BLACK);
        for bg_star in background.iter_mut() {
            bg_star.update(speed, get_frame_time());
        }

        // update the player and powerup start
        player.update(get_frame_time());
        star.update(speed, get_frame_time());

        // check if the player touched the powerup
        if player.collision(&star.rect, false) {
            speed += 20f32 * get_frame_time();
            powerup_emitter_local.draw(vec2(star.rect.x, star.rect.y));
            next_score += 100f32;
            player.powerup();
            star.powerup();
        }

        // check if player collided with an asteroid
        for asteroid in asteroids.iter_mut() {
            asteroid.update(speed, get_frame_time());
            if player.collision(&asteroid.rect, true) {
                next_score += 1f32;
                match player.rect.intersect(asteroid.rect) {
                    Some(rect) => flying_emitter_local.draw(vec2(rect.x, rect.y)),
                    None => {}
                }
            }
        }

        // incrament difference between scores
        if prev_score < next_score {
            prev_score += 1f32;
        }

        // every 10 ticks update the score by one
        tick += 1f32;
        if tick > 10f32 {
            tick = 0f32;
            next_score += 1f32;
        }

        // output the score on the scren
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
