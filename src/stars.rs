use macroquad::prelude::*;

const STAR_SIZE: f32 = 10f32;

fn random_position() -> Rect {
    Rect::new(
        rand::gen_range(0f32, screen_width()),
        -rand::gen_range(0f32, screen_height()),
        1f32,
        2f32,
    )
}

pub struct Star {
    pub rect: Rect,
    color: Color,
    speed: f32,
}

impl Star {
    pub fn new() -> Self {
        let pos = random_position();
        Self {
            rect: Rect::new(pos.x, pos.y, pos.w, pos.h),
            color: Color::new(255f32, 255f32, 255f32, rand::gen_range(0f32, 255f32)),
            speed: rand::gen_range(1f32, 3f32),
        }
    }

    pub fn update(&mut self, speed: &f32, delta: &f32) {
        self.rect.y += self.speed + speed * delta;
        if self.rect.bottom() > screen_height() {
            self.rect = random_position();
            self.color = Color::new(255f32, 255f32, 255f32, rand::gen_range(0f32, 255f32));
            self.speed = rand::gen_range(1f32, 3f32);
        }
        self.draw();
    }

    fn draw(&self) {
        draw_rectangle(
            self.rect.x,
            self.rect.y,
            self.rect.w,
            self.rect.h,
            self.color,
        )
    }
}
