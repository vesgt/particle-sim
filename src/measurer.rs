use crate::sim::Sim;

pub struct Measurer {
    pub force_average: f64,
    pub pressure_average: f64,
    pub area: f64,
    pub kinetic_energy_history: Vec<f64>,
}

impl Measurer {
    pub fn new(sim: &Sim) -> Measurer {
        let area = sim.world.x * sim.world.y;
        Measurer { force_average: 0.0, pressure_average: 0.0, area, kinetic_energy_history: Vec::new() }
    }

    pub fn measure(&mut self, sim: &Sim) {
        
    }
}
