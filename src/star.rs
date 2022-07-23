use macroquad::prelude::*;

const STAR_SIZE: f32 = 10f32;

pub struct Star {
    pub rect: Rect,
}

impl Star {
    pub fn new() -> Self {
        let start_x = rand::gen_range(0f32, screen_width() - STAR_SIZE);
        let start_y = -STAR_SIZE;
        Self {
            rect: Rect::new(start_x, start_y, STAR_SIZE, STAR_SIZE),
        }
    }
    pub fn update(&mut self, speed: f32, delta: f32) {
        self.rect.y += speed * delta;
        if self.rect.bottom() > screen_height() {
            self.powerup();
        } else {
            self.draw();
        }
    }
    pub fn powerup(&mut self) {
        self.rect.x = rand::gen_range(0f32, screen_width());
        self.rect.y = -self.rect.h;
        self.draw();
    }
    fn draw(&self) {
        draw_circle(self.rect.x, self.rect.y, 8f32, YELLOW);
        //draw_rectangle(self.rect.x, self.rect.y, self.rect.w, self.rect.h, YELLOW)
    }
}
