use serde::Serialize;
use crate::sim::Sim;

const k_B: f64 = 1.380649 * 1e-23;

pub struct Measurer {
    pub window_length: f64,
    pub window_start: f64,
    pub start_impact_total: f64,
}

#[derive(Serialize)]
pub struct Sample {
    pub elapsed_time: f64,
    pub impulse: f64,
    pub perimeter: f64,
    pub pressure: f64,
    pub total_kinetic_energy: f64,
    pub temperature: f64,
}

impl Measurer {
    pub fn new(window_length: f64, sim_time: f64, impact_total: f64) -> Measurer {
        Measurer { window_length, window_start: sim_time, start_impact_total: impact_total }
    }

    pub fn sample(&mut self, sim: &Sim) -> Option<Sample> {
        if sim.sim_time - self.window_start < self.window_length {
            return None;
        }

        let total_kinetic_energy: f64 = sim.particles.iter().map(|p| 0.5 * p.vel.length_squared() * p.mass).sum();
        let sample = Sample {
            elapsed_time: self.window_length,
            impulse: sim.impact_total - self.start_impact_total,
            perimeter: 2.0 * (sim.world.x + sim.world.y),
            pressure: (sim.impact_total - self.start_impact_total) / (self.window_length * 2.0 * (sim.world.x + sim.world.y)),
            total_kinetic_energy,
            temperature: total_kinetic_energy / (k_B * sim.particles.len() as f64),
        };
        Some(sample)
    }
}
