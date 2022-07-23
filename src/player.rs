use macroquad::prelude::*;

const PLAYER_SIZE: f32 = 50f32;
const PLAYER_SPEED: f32 = 800f32;

pub struct Player {
    rect: Rect,
    half: f32,
}

impl Player {
    pub fn new() -> Self {
        Self {
            rect: Rect::new(
                screen_width() * 0.5f32 - PLAYER_SIZE,
                screen_height() - 100f32,
                PLAYER_SIZE,
                PLAYER_SIZE,
            ),
            half: PLAYER_SIZE * 0.5f32,
        }
    }
    pub fn update(&mut self, delta: f32) {
        if is_key_down(KeyCode::Right) && self.rect.x + self.half < screen_width() {
            self.rect.x += PLAYER_SPEED * delta;
        }
        if is_key_down(KeyCode::Left) && self.rect.y - self.half > 0f32 {
            self.rect.x -= PLAYER_SPEED * delta
        }
        if self.rect.x < 0f32 {
            self.rect.x = 0f32;
        }
        if self.rect.x > screen_width() - self.rect.w {
            self.rect.x = screen_width() - self.rect.w;
        }
        self.draw();
    }
    pub fn collision(&mut self, rect: &Rect) -> bool {
        // early exit
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
                self.rect.y -= to_signum.y * intersection.h;
            }
            false => {
                // bounce on x
                self.rect.x -= to_signum.x * intersection.w;
            }
        }
        self.draw();
        true
    }
    fn draw(&self) {
        draw_rectangle(self.rect.x, self.rect.y, self.rect.w, self.rect.h, ORANGE)
    }
}
