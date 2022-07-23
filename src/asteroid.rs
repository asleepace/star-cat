use macroquad::prelude::*;

const ASTEROID_SIZE: f32 = 100f32;
const ASTEROID_SPEED: f32 = 100f32;

pub struct Asteroid {
    pub rect: Rect,
}

impl Asteroid {
    pub fn new(y: u32) -> Self {
        let start_x = rand::gen_range(0f32, screen_width() - ASTEROID_SIZE);
        let start_y = y as f32 * ASTEROID_SIZE * 2f32;
        Self {
            rect: Rect::new(start_x, start_y, ASTEROID_SIZE, ASTEROID_SIZE),
        }
    }
    pub fn update(&mut self, delta: f32) {
        self.rect.y += ASTEROID_SPEED * delta;
        if self.rect.y > screen_height() + ASTEROID_SPEED {
            self.rect.x = rand::gen_range(0f32, screen_width());
            self.rect.y = -ASTEROID_SIZE;
        }
        self.draw();
    }
    fn draw(&self) {
        draw_rectangle(self.rect.x, self.rect.y, self.rect.w, self.rect.h, BROWN)
    }
}
