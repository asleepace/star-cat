use macroquad::prelude::*;

const PLAYER_SIZE: f32 = 50f32;
const ACCELERATION: f32 = 1.1f32;
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
    image: Texture2D,
}

impl Player {
    pub fn new(texture: Texture2D) -> Self {
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
            image: texture,
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
        // check for user input and update speed and velocity
        let _ = match (is_key_down(KeyCode::Right), is_key_down(KeyCode::Left)) {
            (true, false) => {
                self.speed *= 1f32 + ACCELERATION;
                self.velocity = 1f32;
            }
            (false, true) => {
                self.speed *= 1f32 + ACCELERATION;
                self.velocity = -1f32;
            }
            _ => {
                // allows speed decay
                self.speed /= DECAY;
            }
        };

        // allow player to slow down

        if self.did_lose() {
            return self.draw();
        }

        // check speed boundries
        self.speed = max(self.speed, -MAX_SPEED);
        self.speed = min(self.speed, MAX_SPEED);

        // move the players x coordinate
        self.rect.x += self.speed * delta * self.velocity;

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
            self.half,
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
