mod particle;
mod events;
mod sim;

use macroquad::prelude::*;
use crate::sim::Sim;

#[macroquad::main("Sim")]
async fn main() {
    let world = vec2(8000.0, 6400.0);
    let mut sim = Sim::new(world, 10.0, 5.0, 5000);

    loop {
        let mut dt = get_frame_time();
        if dt > 0.1 {
            dt = 0.1;
        }
        clear_background(BLACK);

        // world –> screen, recomputed each frame so resizing works
        let scale = (screen_width() / sim.world.x).min(screen_height() / sim.world.y);
        let offset = vec2(
            (screen_width() - sim.world.x * scale) * 0.5,
            (screen_height() - sim.world.y * scale) * 0.5,
        );
        let to_screen = |p: Vec2| p * scale + offset;

        draw_rectangle_lines(offset.x, offset.y, sim.world.x * scale, sim.world.y * scale, 1.0, DARKGRAY);

        for p in &sim.particles {
            let s = to_screen(p.pos);
            draw_circle(s.x, s.y, p.radius * scale, BEIGE);
        }

        sim.advance(dt);

        next_frame().await;
    }
}
