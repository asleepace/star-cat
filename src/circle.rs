pub struct Circle {
    pub x: f32,
    pub y: f32,
    pub radius: f32,
}

impl Circle {
    /** create a new circle from x & y and a radius */
    pub fn new(x: f32, y: f32, radius: f32) -> Self {
        Self { x, y, radius }
    }
    /** check if collided with another circle */
    pub fn hit(&mut self, circle: Circle) -> bool {
        let dx: f32 = (self.x + self.radius) - (circle.x + circle.radius);
        let dy: f32 = (self.y + self.radius) - (circle.y + circle.radius);
        let diameter: f32 = self.radius + circle.radius;
        let distance: f32 = (dx * dx + dy * dy).sqrt();
        let collision = distance < diameter;
        return collision;
    }
}
