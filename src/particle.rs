use macroquad::math::Vec2;
use crate::events::Axis;

#[derive(Clone)]
pub struct Particle {
    pub radius: f32,
    pub mass: f32,
    pub vel: Vec2,
    pub pos: Vec2,
    pub collision_count: u32,
}

impl Default for Particle {
    fn default() -> Self { Particle { radius: 1.0, mass: 1.0, vel: Vec2::ZERO, pos: Vec2::ZERO, collision_count: 0  } }
}

impl Particle {
    pub fn time_to_wall(&self, world: Vec2) -> Option<(f64, Axis)> {
        let t_x = if self.vel.x > 0.0 {
            ((world.x - self.pos.x - self.radius) / self.vel.x) as f64
        } else if self.vel.x < 0.0 {
            ((self.pos.x - self.radius) / -self.vel.x) as f64
        } else {
            f64::INFINITY
        };

        let t_y = if self.vel.y > 0.0 {
            ((world.y - self.pos.y - self.radius) / self.vel.y) as f64
        } else if self.vel.y < 0.0 {
            ((self.pos.y - self.radius) / -self.vel.y) as f64
        } else {
            f64::INFINITY
        };

        // FIX: only "no event" when both are infinite. A finite tie (exact corner
        // hit) used to return None and let the particle escape the box.
        if t_x.is_infinite() && t_y.is_infinite() {
            None
        } else if t_x <= t_y {
            Some((t_x, Axis::X))
        } else {
            Some((t_y, Axis::Y))
        }
    }
}
