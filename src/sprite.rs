use macroquad::prelude::*;

pub struct Sprite {
    pub frame: Rect,
    pub speed: Vec2<i32>,
}

impl Sprite {
    /**
     * Create a new sprite with a given frame and speed.
     */
    pub fn new(frame: Rect, speed: Vec2<i32>) -> Self {
        Self { frame, speed }
    }

    /**
     * Update the sprite movement,
     */
    pub fn update(&mut self) {
        self.frame.x_pos += self.speed.x_pos;
        self.frame.y_pos += self.speed.y_pos;
    }

    /**
     * Draw the sprite to the screen.
     */
    pub fn draw() {
        draw_rectangle(
            self.rect.x_pos,
            self.rect.y_pos,
            self.rect.width,
            self.rect.height,
            self.color1,
        );
    }
}
