mod asteroid;
mod games;
mod player;
mod powerup;
mod stars;

use asteroid::Asteroid;
use macroquad::prelude::*;
use macroquad_particles::{self as particles, Emitter, EmitterConfig};
use player::Player;
use powerup::Powerup;
use stars::Star;

const GAME_SPEED: f32 = 200.0;
const ASTEROIDS: u32 = 6;

fn smoke() -> particles::EmitterConfig {
    particles::EmitterConfig {
        lifetime: 0.8,
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

    let font = match load_ttf_font("res/VT323-Regular.ttf").await {
        Ok(font) => font,
        Err(e) => {
            println!("Error loading font: {:?}", e);
            return;
        }
    };

    // let font = load_ttf_font("res/VT323-Regular.ttf").await.unwrap();
    let cat: Texture2D = load_texture("res/cat_graphic.png").await.unwrap();
    let brown = Color::from_rgba(110u8, 67u8, 24u8, 200u8);
    let smoke_texture = Image::gen_image_color(2u16, 2u16, brown);
    let texture = Texture2D::from_image(&smoke_texture);

    // instantiate the player
    let mut player = Player::new(cat);
    let mut asteroids = Vec::<Asteroid>::new();
    let mut background = Vec::<Star>::new();
    let mut powerup = Powerup::new();
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
        background.push(Star::new());
    }

    // run the game loop
    loop {
        let frame_time = get_frame_time();

        // clear background and paint background stars first
        clear_background(BLACK);
        for bg_star in background.iter_mut() {
            bg_star.update(&speed, &frame_time);
        }

        // handle various game states
        let _ = &match game_state {
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
                let message = match player.did_win() {
                    true => "YOU WON!",
                    false => "GAME OVER",
                };
                display_text(&font, message, 80, 0f32, y);
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
                for i in 0..ASTEROIDS {
                    asteroids.push(Asteroid::new(i));
                }
            }
            GameState::Play => {
                // output the score on the screen
                let score_text = format!("SCORE: {}", prev_score);
                display_text(&font, &score_text, 40, 0.0, 40.0);

                // check win and loss conditions
                match player.did_win() || player.did_lose() {
                    true => game_state = GameState::GameOver,
                    false => {},
                }

                // increment difference between scores
                if prev_score < next_score {
                    prev_score += 1f32;
                }

                // every 10 ticks update the score by one
                tick += 1f32;
                if tick > 10f32 {
                    tick = 0f32;
                    next_score += 1f32;
                    player.rect.y -= 1f32 * frame_time;
                }
            }
        };

        // update the player and powerup start
        player.update(&frame_time);
        powerup.update(&speed, &frame_time);
        powerup.draw();

        // check if the player touched the powerup
        if player.collision(&powerup.rect, false) {
            speed += 1f32;
            next_score += 100f32;
            player.powerup();
            powerup.reset();
        }

        // check if player collided with an asteroid
        for asteroid in asteroids.iter_mut() {
            asteroid.update(&speed, &frame_time);
            asteroid.draw();
            if player.collision(&asteroid.rect, true) {
                //next_score += 1f32;
                match player.rect.intersect(asteroid.rect) {
                    Some(rect) => {
                        let _direction = if player.rect.x < asteroid.rect.x {
                            -1.2f32
                        } else {
                            1.2f32
                        };
                        flying_emitter_local.draw(vec2(rect.x, rect.y));
                        asteroid.hit(player.rect.x - asteroid.rect.x);
                    }
                    None => {}
                }
            }
        }

        // wait for the next frame
        next_frame().await
    }
}
