use macroquad::prelude::*;

const POWERUP_SIZE: f32 = 10f32;

fn init() -> Rect {
    let x = rand::gen_range(0f32, screen_width());
    let y = POWERUP_SIZE;
    Rect::new(x, y, POWERUP_SIZE, POWERUP_SIZE)
}

pub struct Powerup {
    pub rect: Rect,
}

impl Powerup {
    pub fn new() -> Self {
        Self { rect: init() }
    }

    pub fn update(&mut self, speed: &f32, delta: &f32) {
        self.rect.y += speed * delta;
        if self.rect.bottom() > screen_height() {
            self.rect = init();
        }
    }

    pub fn reset(&mut self) {
        self.rect = init();
    }

    pub fn draw(&self) {
        draw_circle(self.rect.x, self.rect.y, 8f32, YELLOW);
        //draw_rectangle(self.rect.x, self.rect.y, self.rect.w, self.rect.h, YELLOW)
    }
}
