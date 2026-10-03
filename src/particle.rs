use macroquad::math::DVec2;
use crate::events::Axis;

#[derive(Clone)]
pub struct Particle {
    pub radius: f64,
    pub mass: f64,
    pub vel: DVec2,
    pub pos: DVec2,
    pub collision_count: u64,
}

impl Default for Particle {
    fn default() -> Self { Particle { radius: 1.0, mass: 1.0, vel: DVec2::ZERO, pos: DVec2::ZERO, collision_count: 0  } }
}

impl Particle {
    pub fn time_to_wall(&self, world: DVec2) -> Option<(f64, Axis)> {
        let t_x = if self.vel.x > 0.0 {
            (world.x - self.pos.x - self.radius) / self.vel.x
        } else if self.vel.x < 0.0 {
            (self.pos.x - self.radius) / -self.vel.x
        } else {
            f64::INFINITY
        };

        let t_y = if self.vel.y > 0.0 {
            (world.y - self.pos.y - self.radius) / self.vel.y
        } else if self.vel.y < 0.0 {
            (self.pos.y - self.radius) / -self.vel.y
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
