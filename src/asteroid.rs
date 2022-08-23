use crate::circle::Circle;
use fast_math::atan2;
use macroquad::prelude::*;
use std::f32::consts::PI;

/**
 *  Asteroid
 *  
 *  This file contains the logic for the games asteroids such as movement, collisions and drawing.
 *  Asteroid are generated with a random number of sides, sizes and positions. However, their hit
 *  detection uses a simple square collision for now.
 */

const VELOCITY_DECAY: f32 = 0.99999;
// MAX_VELOCITY: f32 = 200.0;
const ASTEROID_MIN_SIZE: f32 = 100.0;
const ASTEROID_MAX_SIZE: f32 = 400.0;
const MIN_EDGES: u8 = 12;
const MAX_EDGES: u8 = 18;
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
    let offset = y as f32 * ASTEROID_MAX_SIZE;
    let size = rand::gen_range(ASTEROID_MIN_SIZE, ASTEROID_MAX_SIZE);
    let start_x = rand::gen_range(-size, screen_width());
    let start_y = rand::gen_range(size, screen_height()) + offset;
    Asteroid {
        color: Color::from_rgba(110u8, 67u8, 24u8, 255u8),
        spin: rand::gen_range(SPIN_MIN, SPIN_MAX),
        side: rand::gen_range(MIN_EDGES, MAX_EDGES),
        velocity: rand::gen_range(-10f32, 10f32),
        texture: get_textures(size),
        radius: size * 0.5f32,
        size: size,
        x: start_x,
        y: -start_y,
        id: y,
    }
}

pub struct Asteroid {
    pub side: u8,
    pub size: f32,
    pub spin: f32,
    pub color: Color,
    pub velocity: f32,
    pub radius: f32,
    pub id: u32,
    pub x: f32,
    pub y: f32,
    texture: Vec<Rect>,
}

impl Asteroid {
    pub fn new(y: u32) -> Self {
        let params = get_random_asteroid(y);
        return params;
    }

    pub fn hit(&mut self, speed: f32) {
        self.velocity += speed * get_frame_time();
        //self.velocity -= speed / self.size * 2.;
    }

    pub fn reset(&mut self) {
        let next = get_random_asteroid(self.id);
        self.side = next.side;
        self.size = next.size;
        self.spin = next.spin;
        self.color = next.color;
        // self.velocity = next.velocity;
        self.texture = next.texture;
        self.x = next.x;
        self.y = next.y;
        self.radius = next.radius;
    }

    pub fn update(&mut self, speed: &f32, delta: &f32) {
        self.y += speed * delta;
        self.x += self.velocity * delta;
        if self.y > screen_height() + self.size {
            self.y = -rand::gen_range(self.size, screen_height());
        }
        if self.x > screen_width() {
            self.x = -self.size;
        }
        if self.x + self.size < 0f32 {
            self.x = screen_width() + self.size;
        }
        if self.velocity < -1f32 || self.velocity > 1f32 {
            self.velocity *= VELOCITY_DECAY;
        }
    }

    pub fn get_circle(&self) -> Circle {
        Circle {
            x: self.x + self.radius,
            y: self.y + self.radius,
            radius: self.radius,
        }
    }

    pub fn collide(&self, asteroid: Circle) -> Option<Vec2> {
        let current = self.get_circle();

        let diameter: f32 = current.radius + asteroid.radius;

        let dx: f32 = (current.x + current.radius * 2.) - (asteroid.x + asteroid.radius);
        let dy: f32 = (current.x + current.radius * 2.) - (asteroid.x + asteroid.radius);
        let distance: f32 = (dx * dx + dy * dy).sqrt();
        match distance < diameter {
            true => {
                // println!(
                //     "{:?}",
                //     (
                //         (current.x, asteroid.y),
                //         (current.y, asteroid.y),
                //         (current.radius, asteroid.radius)
                //     )
                // );
                let angle = atan2(dy, dx);
                let move_by = diameter - distance;
                let x = move_by * angle.cos();
                let y = move_by * angle.sin();
                let vec = Vec2::new(x, y);
                Some(vec)
            }
            false => None,
        }
    }

    pub fn draw(&self) {
        //  draw_rectangle(self.x, self.y, self.size, self.size, self.color);
        draw_poly(
            self.x + self.radius,
            self.y + self.radius,
            self.side,
            self.radius,
            self.spin,
            self.color,
        );
        for texture in self.texture.iter() {
            let radius = texture.w * 0.2f32;
            let x = self.x + texture.x;
            draw_poly(
                x + radius,
                texture.y + self.y + radius,
                8u8,
                radius,
                0f32,
                Color::from_rgba(88u8, 54u8, 19u8, 255u8),
            );
        }
    }
}
