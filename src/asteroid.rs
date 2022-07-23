use macroquad::prelude::*;

const ASTEROID_SIZE: f32 = 100f32;

pub struct Asteroid {
    pub rect: Rect,
    pub side: u8,
    pub size: f32,
    pub spin: f32,
    pub color1: Color,
    pub color2: Color,
    pub velocity: f32,
}

impl Asteroid {
    pub fn new(y: u32) -> Self {
        let size = rand::gen_range(50f32, 150f32);
        let start_x = rand::gen_range(0f32, screen_width() - size);
        let start_y = y as f32 * size * -2f32;

        Self {
            rect: Rect::new(start_x, start_y, size, size),
            side: rand::gen_range(8, 14),
            size: size,
            spin: 0f32,
            color1: Color::from_rgba(110u8, 67u8, 24u8, 255u8),
            color2: Color::from_rgba(110u8, 67u8, 24u8, 255u8),
            velocity: 0f32,
        }
    }

    pub fn hit(&mut self, speed: f32) {
        self.velocity -= speed / self.size;
    }

    pub fn update(&mut self, speed: &f32, delta: &f32) {
        self.rect.y += speed * delta;
        self.rect.x += self.velocity * delta;
        if self.rect.y > screen_height() + self.size {
            self.rect.x = rand::gen_range(0f32, screen_width());
            self.rect.y = -self.size * 2f32;
            self.velocity = 0f32;
        }
        if self.rect.x + 200f32 > screen_width() {
            self.rect.x = -self.size;
        }
        if self.rect.x + self.size + 200f32 < 0f32 {
            self.rect.x = screen_width();
        }
    }

    pub fn draw(&self) {
        draw_rectangle(
            self.rect.x,
            self.rect.y,
            self.rect.w,
            self.rect.h,
            self.color1,
        );
        draw_poly(
            self.rect.x + (self.size / 2f32),
            self.rect.y + (self.size / 2f32),
            self.side,
            self.size / 1.5f32,
            self.rect.y * get_frame_time() * self.spin,
            self.color2,
        );
        //draw_rectangle(self.rect.x, self.rect.y, self.rect.w, self.rect.h, BROWN);
    }
}
