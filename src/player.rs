use crate::asteroid::Asteroid;
use crate::Circle;
use fast_math::atan2;
use macroquad::prelude::*;

const PLAYER_SIZE: f32 = 50f32;
const ACCELERATION: f32 = 20.0f32;
const MAX_ACELERATION: f32 = 400.0f32;
const DECAY: f32 = 1.08f32;
const MAX_SPEED: f32 = 800f32;

fn min(a: f32, b: f32) -> f32 {
    if a < b {
        a
    } else {
        b
    }
}

fn max(a: f32, b: f32) -> f32 {
    if a > b {
        a
    } else {
        b
    }
}

pub struct Player {
    pub rect: Rect,
    half: f32,
    speed: f32,
    velocity: f32,
    radius: f32,
    did_collide: f32,
    acceleration: f32,
    // image: Texture2D,
}

impl Player {
    pub fn new(_texture: Texture2D) -> Self {
        Self {
            rect: Rect::new(
                screen_width() * 0.5f32 - PLAYER_SIZE,
                screen_height() - 100f32,
                PLAYER_SIZE,
                PLAYER_SIZE,
            ),
            half: PLAYER_SIZE * 0.5f32,
            speed: 200f32,
            velocity: 0f32,
            radius: PLAYER_SIZE * 0.5f32,
            did_collide: 0.,
            acceleration: 0.,
            // image: texture,
        }
    }

    pub fn reset(&mut self) {
        self.rect.x = screen_width() * 0.5f32 - PLAYER_SIZE;
        self.rect.y = screen_height() - 100f32;
        self.speed = 200f32;
        self.velocity = 0f32;
        self.draw();
    }

    pub fn update(&mut self, delta: &f32) {
        if self.did_collide > 0. {
            self.did_collide -= 1.;
        }
        // check for user input and update speed and velocity
        if self.did_collide <= 0. {
            let _ = match (is_key_down(KeyCode::Right), is_key_down(KeyCode::Left)) {
                (true, false) => {
                    self.acceleration += ACCELERATION;
                    // self.speed *= self.acceleration;
                    // self.velocity = 1f32;
                }
                (false, true) => {
                    self.acceleration -= ACCELERATION;
                    // self.speed *= 1f32 + ACCELERATION;
                    // self.velocity = -1f32;
                }
                _ => {
                    // if self.acceleration < 0. {
                    //     self.acceleration += ACCELERATION;
                    // }
                    // if self.acceleration > 0. {
                    //     self.acceleration -= ACCELERATION;
                    // }
                    // // allows speed decay
                    // self.speed /= DECAY;
                }
            };
        }

        if self.acceleration > MAX_ACELERATION {
            self.acceleration = MAX_ACELERATION;
        }
        if self.acceleration < -MAX_ACELERATION {
            self.acceleration = -MAX_ACELERATION;
        }

        // self.speed += self.acceleration;

        // allow player to slow down

        if self.did_lose() {
            return self.draw();
        }

        // check speed boundries
        self.speed = max(self.speed, -MAX_SPEED);
        self.speed = min(self.speed, MAX_SPEED);

        // move the players x coordinate
        self.rect.x += self.acceleration * delta; // * self.velocity;

        // check screen boundries and draw
        self.rect.x = min(self.rect.x, screen_width() - self.rect.w);
        self.rect.x = max(self.rect.x, 0f32);
        self.draw();
    }

    pub fn collision(&mut self, rect: &Rect, draw: bool) -> bool {
        let intersection = match self.rect.intersect(*rect) {
            Some(intersection) => intersection,
            None => return false,
        };
        let a_center = self.rect.point() + self.rect.size() * 0.5f32;
        let b_center = rect.point() + rect.size() * 0.5f32;
        let to = b_center - a_center;
        let to_signum = to.signum();
        match intersection.w > intersection.h {
            true => {
                // bounce on y
                self.rect.y -= to_signum.y * intersection.h * 2f32;
            }
            false => {
                // bounce on x
                self.rect.x -= to_signum.x * intersection.w;
            }
        }
        if draw {
            self.draw();
        }
        true
    }

    /** detect collisions with asteroids based on radii */
    pub fn collide(&mut self, asteroid: &Asteroid) -> bool {
        let dx: f32 = (self.rect.x + self.radius) - (asteroid.x + asteroid.radius);
        let dy: f32 = (self.rect.y + self.radius) - (asteroid.y + asteroid.radius);
        let diameter: f32 = self.radius + asteroid.radius;
        let distance: f32 = (dx * dx + dy * dy).sqrt();
        let collision = distance < diameter;

        if collision {
            let angle = atan2(dy, dx);
            let move_by = diameter - distance;
            self.rect.x += angle.cos() * move_by;
            self.rect.y += angle.sin() * move_by;
            self.did_collide = 3.;
            self.velocity *= -1.;
        }

        return distance < diameter;
    }

    pub fn did_lose(&mut self) -> bool {
        self.rect.y > screen_height()
    }

    pub fn did_win(&mut self) -> bool {
        self.rect.y <= 0.0
    }

    pub fn powerup(&mut self) {
        self.rect.y -= self.speed * get_frame_time();
        self.draw();
    }

    pub fn draw(&self) {
        // draw_rectangle(self.rect.x, self.rect.y, self.rect.w, self.rect.h, BLUE);
        // draw_texture(self.image, self.rect.x, self.rect.y, WHITE);

        // draw_texture_ex(
        //     self.image,
        //     self.rect.x - 13f32,
        //     self.rect.y - 6f32,
        //     WHITE,
        //     DrawTextureParams {
        //         dest_size: Some(Vec2::new(75f32, 77.50f32)),
        //         flip_x: false,
        //         flip_y: false,
        //         pivot: None,
        //         source: None,
        //         rotation: 0f32,
        //     },
        // )

        draw_circle(
            self.rect.x + self.half,
            self.rect.y + self.half,
            self.radius,
            ORANGE,
        );
        // draw_circle(
        //     self.rect.x + self.half + 8f32,
        //     self.rect.y + self.half - 10f32,
        //     5f32,
        //     BLACK,
        // );
        // draw_circle(
        //     self.rect.x + self.half - 8f32,
        //     self.rect.y + self.half - 10f32,
        //     5f32,
        //     BLACK,
        // );
    }
}
