use std::f32::consts::PI;

use macroquad::prelude::*;

/**
 *  Asteroid
 *  
 *  This file contains the logic for the games asteroids such as movement, collisions and drawing.
 *  Asteroid are generated with a random number of sides, sizes and positions. However, their hit
 *  detection uses a simple square collision for now.
 */

const VELOCITY_DECAY: f32 = 0.99999;
const MAX_VELOCITY: f32 = 200.0;
const ASTEROID_MIN_SIZE: f32 = 100.0;
const ASTEROID_MAX_SIZE: f32 = 200.0;
const SPIN_MIN: f32 = 0.0;
const SPIN_MAX: f32 = 2.0 * PI;

fn get_textures(size: f32) -> Vec<Rect> {
    let mut texture = Vec::<Rect>::new();
    for _ in 0..10 {
        let w = rand::gen_range(0f32, size);
        let x = rand::gen_range(0f32, size - w * 0.5f32);
        let y = rand::gen_range(0f32, size - w * 0.5f32);
        let h = rand::gen_range(0f32, size);
        texture.push(Rect::new(x, y, w, h));
    }
    return texture;
}

pub fn get_random_asteroid(y: u32) -> Asteroid {
    let size = rand::gen_range(ASTEROID_MIN_SIZE, ASTEROID_MAX_SIZE);
    let start_x = rand::gen_range(-size, screen_width());
    let start_y = rand::gen_range(size, screen_height());
    Asteroid {
        color: Color::from_rgba(110u8, 67u8, 24u8, 255u8),
        rect: Rect::new(start_x, -start_y, size, size),
        spin: rand::gen_range(SPIN_MIN, SPIN_MAX),
        side: rand::gen_range(10, 16),
        velocity: rand::gen_range(-10f32, 10f32),
        texture: get_textures(size),
        size: size,
        id: y,
    }
}

pub struct Asteroid {
    pub rect: Rect,
    pub side: u8,
    pub size: f32,
    pub spin: f32,
    pub color: Color,
    pub velocity: f32,
    pub id: u32,
    texture: Vec<Rect>,
}

impl Asteroid {
    pub fn new(y: u32) -> Self {
        let size = rand::gen_range(ASTEROID_MIN_SIZE, ASTEROID_MAX_SIZE);
        let start_x = rand::gen_range(0f32, screen_width() - size);
        let start_y = y as f32 * size * -2f32;
        let params = get_random_asteroid(y);
        return params;
    }

    pub fn hit(&mut self, speed: f32) {
        self.velocity -= speed / self.size;
    }

    pub fn reset(&mut self) {
        let next = get_random_asteroid(self.id);
        self.rect = next.rect;
        self.side = next.side;
        self.size = next.size;
        self.spin = next.spin;
        self.color = next.color;
        self.velocity = next.velocity;
        self.texture = next.texture;
    }

    pub fn update(&mut self, speed: &f32, delta: &f32) {
        self.rect.y += speed * delta;
        self.rect.x += self.velocity * delta;
        if self.rect.y > screen_height() + self.size {
            self.reset();
        }
        if self.rect.x > screen_width() {
            self.rect.x = -self.size;
        }
        if self.rect.x + self.size < 0f32 {
            self.rect.x = screen_width();
        }
        if self.velocity < -1f32 || self.velocity > 1f32 {
            self.velocity *= VELOCITY_DECAY;
        }
    }

    pub fn draw(&self) {
        draw_rectangle(
            self.rect.x,
            self.rect.y,
            self.rect.w,
            self.rect.h,
            self.color,
        );
        draw_poly(
            self.rect.x + (self.size / 2f32),
            self.rect.y + (self.size / 2f32),
            self.side,
            self.size / 1.5f32,
            self.spin,
            self.color,
        );
        for texture in self.texture.iter() {
            let radius = texture.w * 0.2f32;
            let x = self.rect.x + texture.x;
            draw_poly(
                x + radius,
                texture.y + self.rect.y + radius,
                8u8,
                radius,
                0f32,
                Color::from_rgba(88u8, 54u8, 19u8, 255u8),
            );
        }
        //draw_rectangle(self.rect.x, self.rect.y, self.rect.w, self.rect.h, BROWN);
    }
}
