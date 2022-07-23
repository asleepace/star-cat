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

const GAME_SPEED: f32 = 200.0;

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
        lifetime: 1.2,
        amount: 20,
        initial_direction: vec2((270f32).sin(), (270f32).cos()),
        initial_direction_spread: 0.8f32,
        size: 2f32,
        ..Default::default()
    }
}

fn display_text(font: &Font, text: &str, size: u16, x: f32, y: f32) {
    let text_dim = measure_text(&text, Some(*font), size, 1.0);
    let x_pos = screen_width() * 0.5f32 - text_dim.width * 0.5f32 - x;
    let y_pos = text_dim.height * 0.5f32 + y;
    draw_text_ex(
        &text,
        x_pos,
        y_pos,
        TextParams {
            font: *font,
            font_size: size,
            color: WHITE,
            ..Default::default()
        },
    );
}

enum GameState {
    New,
    Play,
    Reset,
    GameOver,
}

#[macroquad::main("star-cat")]
async fn main() {
    // initialize and load graphical assets
    let font = load_ttf_font("res/VT323-Regular.ttf").await.unwrap();
    let cat: Texture2D = load_texture("res/cat_graphic.png").await.unwrap();
    let smoke_texture = Image::gen_image_color(2u16, 2u16, BROWN);
    let texture = Texture2D::from_image(&smoke_texture);

    // instantiate the player
    let mut player = Player::new(cat);
    let mut asteroids = Vec::<Asteroid>::new();
    let mut background = Vec::<BackgroundStar>::new();
    let mut star = Star::new();
    let mut speed = GAME_SPEED;
    let mut prev_score = 0f32;
    let mut next_score = 0f32;
    let mut tick = 0f32;
    let mut game_state: GameState = GameState::New;

    let mut flying_emitter_local = Emitter::new(EmitterConfig {
        local_coords: true,
        texture: Some(texture),
        ..smoke()
    });

    // generate the background
    for _ in 0..100 {
        background.push(BackgroundStar::new());
    }

    // run the game loop
    loop {
        let frame_time = get_frame_time();

        // clear background and paint background stars first
        clear_background(BLACK);
        for bg_star in background.iter_mut() {
            bg_star.update(speed, get_frame_time());
        }

        // handle various game states
        match game_state {
            GameState::New => {
                if is_key_pressed(KeyCode::Space) {
                    game_state = GameState::Reset;
                }
                let y_pos = screen_height() * 0.5f32;
                display_text(&font, "Star Cat", 80, 0f32, y_pos);
                display_text(&font, "PRESS SPACE TO START", 30, 0f32, y_pos + 40f32);
                next_frame().await;
                continue;
            }
            GameState::GameOver => {
                if is_key_pressed(KeyCode::Space) {
                    game_state = GameState::Reset;
                    print!("resting");
                }
                let y = screen_height() * 0.5f32;
                display_text(&font, "GAME OVER", 80, 0f32, y);
                display_text(
                    &font,
                    &format!("SCORE: {}", &next_score),
                    30,
                    0f32,
                    y - 40f32,
                );
                display_text(&font, "PRESS SPACE TO RESTART", 30, 0f32, -60f32);
                speed = 0.0;
                //continue;
            }
            GameState::Reset => {
                game_state = GameState::Play;
                speed = GAME_SPEED;
                prev_score = 0f32;
                next_score = 0f32;
                player.reset();
                asteroids.clear();
                for i in 0..5 {
                    asteroids.push(Asteroid::new(i));
                }
            }
            GameState::Play => {
                // check win and loss conditions
                match player.did_win() || player.did_lose() {
                    true => game_state = GameState::GameOver,
                    _ => {}
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
            }
            _ => {}
        };

        // update the player and powerup start
        player.update(get_frame_time());
        star.update(speed, get_frame_time());

        // check if the player touched the powerup
        if player.collision(&star.rect, false) {
            speed += 50f32 * get_frame_time();
            next_score += 100f32;
            player.powerup();
            star.powerup();
        }

        // check if player collided with an asteroid
        for asteroid in asteroids.iter_mut() {
            asteroid.update(speed, get_frame_time());
            if player.collision(&asteroid.rect, true) {
                //next_score += 1f32;
                match player.rect.intersect(asteroid.rect) {
                    Some(rect) => flying_emitter_local.draw(vec2(rect.x, rect.y)),
                    None => {}
                }
            }
        }

        match game_state {
            GameState::Play => {
                // output the score on the scren
                let score_text = format!("SCORE: {}", prev_score);
                display_text(&font, &score_text, 40, 0.0, 40.0);
            }
            _ => {}
        }

        // wait for the next frame
        next_frame().await
    }
}
