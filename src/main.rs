use macroquad::prelude::*;

/** game constants */

const PLAYER_SIZE: f32 = 50f32;
const PLAYER_SPEED: f32 = 800f32;

struct Player {
    rect: Rect,
    half: f32,
}

impl Player {
    fn new() -> Self {
        Self {
            rect: Rect::new(
                screen_width() * 0.5f32 - PLAYER_SIZE,
                screen_height() - 100f32,
                PLAYER_SIZE,
                PLAYER_SIZE,
            ),
            half: PLAYER_SIZE * 0.5f32,
        }
    }
    fn update(&mut self, delta: f32) {
        if is_key_down(KeyCode::Right) && self.rect.x + self.half < screen_width() {
            self.rect.x += PLAYER_SPEED * delta;
        }
        if is_key_down(KeyCode::Left) && self.rect.y - self.half > 0f32 {
            self.rect.x -= PLAYER_SPEED * delta
        }
        if self.rect.x < 0f32 {
            self.rect.x = 0f32;
        }
        if self.rect.x > screen_width() - self.rect.w {
            self.rect.x = screen_width() - self.rect.w;
        }
    }
    fn draw(&self) {
        draw_rectangle(self.rect.x, self.rect.y, self.rect.w, self.rect.h, ORANGE)
    }
}

#[macroquad::main("star-cat")]
async fn main() {
    // instantiate the player
    let mut player = Player::new();

    // build camera with following coordinate system:
    // (0., 0)     .... (SCR_W, 0.)
    // (0., SCR_H) .... (SCR_W, SCR_H)
    // set_camera(&Camera2D {
    //     zoom: vec2(1. / SCR_W * 2., -1. / SCR_H * 2.),
    //     target: vec2(SCR_W / 2., SCR_H / 2.),
    //     ..Default::default()
    // });

    loop {
        clear_background(DARKGRAY);

        player.update(get_frame_time());
        player.draw();

        next_frame().await
    }
}
