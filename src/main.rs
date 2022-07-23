mod player;
use macroquad::prelude::*;
use player::Player;

const BLOCK_SIZE: f32 = 100f32;
const BLOCK_SPEED: f32 = 100f32;

struct Block {
    rect: Rect,
}

impl Block {
    fn new(y: u32) -> Self {
        let start_x = rand::gen_range(0f32, screen_width() - BLOCK_SIZE);
        let start_y = y as f32 * BLOCK_SIZE;
        Self {
            rect: Rect::new(start_x, start_y, BLOCK_SIZE, BLOCK_SIZE),
        }
    }
    pub fn update(&mut self, delta: f32) {
        self.rect.y += BLOCK_SPEED * delta;
        if (self.rect.y > screen_height() + BLOCK_SPEED) {
            self.rect.x = rand::gen_range(0f32, screen_width());
            self.rect.y = -BLOCK_SIZE;
        }
        self.draw();
    }
    fn draw(&self) {
        draw_rectangle(self.rect.x, self.rect.y, self.rect.w, self.rect.h, BROWN)
    }
}

fn check_collision(rect1: Rect, rect2: Rect) -> bool {
    let x_align = rect1.x + rect1.w >= rect2.x && rect1.x <= rect2.x + rect2.w;
    let y_align = rect1.y + rect1.h >= rect2.y && rect1.y <= rect2.y + rect2.h;
    return x_align && y_align;
}

#[macroquad::main("star-cat")]
async fn main() {
    // instantiate the player
    let mut player = Player::new();
    let mut asteroids = Vec::<Block>::new();

    for i in 0..10 {
        asteroids.push(Block::new(i));
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
