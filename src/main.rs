mod particle;
mod events;
mod sim;
mod renderer;
mod measurer;

use macroquad::prelude::*;
use crate::particle::Particle;
use crate::sim::Sim;

#[macroquad::main("Sim")]
async fn main() {
    let mut sim = Sim::new(1.88 * 1e-10, 6.634 * 1e-26, 2000, 300.0, 0.01);

    let rms = rms_speed(&sim.particles);

    loop {
        let mut dt = get_frame_time();
        dt = dt * 1e-10;
        clear_background(BLACK);

        // world –> screen, recomputed each frame so resizing works
        let scale = (screen_width() as f64 / sim.world.x).min(screen_height() as f64 / sim.world.y);
        let offset = dvec2(
            (screen_width() as f64 - sim.world.x * scale) * 0.5,
            (screen_height() as f64 - sim.world.y * scale) * 0.5,
        );
        let to_screen = |p: DVec2| p * scale + offset;

        draw_rectangle_lines(offset.x as f32, offset.y as f32, (sim.world.x * scale) as f32, (sim.world.y * scale) as f32, 1.0, DARKGRAY);

        for p in &sim.particles {
            let s = to_screen(p.pos);
            draw_circle(s.x as f32, s.y as f32, (p.radius * scale) as f32, speed_color(p.vel, rms));
        }

        sim.advance(dt);

        next_frame().await;
    }
}

fn rms_speed(particles: &Vec<Particle>) -> f64 {
    let sum_sq: f64 = particles.iter().map(|p| p.vel.length_squared()).sum();
    (sum_sq / particles.len() as f64).sqrt()
}

fn speed_color(vel: DVec2, rms: f64) -> Color {
    let t = (vel.length() / (2.0 * rms)).clamp(0.0, 1.0) as f32;
    if t < 0.5 {
        Color::new(t * 2.0, 1.0, 0.0, 1.0)         // green -> yellow
    } else {
        Color::new(1.0, 2.0 - t * 2.0, 0.0, 1.0)   // yellow -> red
    }
}