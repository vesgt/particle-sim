use std::collections::BinaryHeap;
use macroquad::math::{vec2, Vec2};
use macroquad::rand::gen_range;
use crate::events::{Axis, EventType, CollisionEvent, ParticleInfo};
use crate::events::EventType::Wall;
use crate::particle::Particle;

pub struct Sim {
    pub world: Vec2,
    pub radius: f32,
    pub mass: f32,
    pub particles: Vec<Particle>,
    pub heap: BinaryHeap<CollisionEvent>,
    pub sim_time: f64,
}

impl Sim {
    pub fn new(world: Vec2, radius: f32, mass: f32, count: usize) -> Sim {
        let mut particles = Vec::with_capacity(count);
        let usable_span_x = world.x - 2.0 * radius;
        let usable_span_y = world.y - 2.0 * radius;
        let col_count = (count as f32 * usable_span_x / usable_span_y).sqrt().ceil();
        let row_count = (count as f32 / col_count).ceil();

        // FIX: keep spacing in f32 (the i64 cast threw away the fraction and would
        // give 0 for a dense grid), and guard the single-row/column div-by-zero.
        let spacing = (usable_span_x / (col_count - 1.0).max(1.0))
            .min(usable_span_y / (row_count - 1.0).max(1.0));

        let offset_x = radius + ((usable_span_x - (col_count - 1.0) * spacing) / 2.0);
        let offset_y = radius + ((usable_span_y - (row_count - 1.0) * spacing) / 2.0);

        for i in 0..count {
            let c = i as f32 % col_count;
            // FIX: floor, not ceil. ceil put row 0 and row 1 on top of each other
            // and pushed the last row outside the world.
            let d = (i as f32 / col_count).floor();

            particles.push(Particle {
                radius,
                mass,
                vel: vec2(gen_range(-50.0, 50.0), gen_range(-50.0, 50.0)),
                pos: vec2(
                    offset_x + (c * spacing),
                    offset_y + (d * spacing),
                ),
                collision_count: 0,
            });
        }

        let heap: BinaryHeap<CollisionEvent> = BinaryHeap::new();

        let mut sim = Sim { world, radius, mass, particles, sim_time: 0.0, heap };

        for i in 0..sim.particles.len() {
            sim.predict(i);
        }

        sim
    }

    fn predict(&mut self, index: usize) {
        let particle_info: ParticleInfo = ParticleInfo {
            index,
            col_count: self.particles[index].collision_count
        };
        let wall = self.particles[index].time_to_wall(self.world);
        match wall {
            Some(wall) => {
                let event = CollisionEvent {
                    particle: particle_info,
                    abs_time: self.sim_time + wall.0,
                    event_type: Wall(wall.1),
                };
                self.heap.push(event);
            }
            None => {}
        }

        for (j, p) in self.particles.iter().enumerate() {
            let event = self.time_to_particle(index, j);
            match event {
                Some(event) => {
                    self.heap.push(event);
                },
                None => {}
            }
        }
    }

    fn resolve(&mut self, event: CollisionEvent) {
        match event.event_type {
            Wall(a) => {
                match a {
                    Axis::X => {
                        self.particles[event.particle.index].vel.x = -self.particles[event.particle.index].vel.x;
                        self.particles[event.particle.index].collision_count += 1;
                    },
                    Axis::Y => {
                        self.particles[event.particle.index].vel.y = -self.particles[event.particle.index].vel.y;
                        self.particles[event.particle.index].collision_count += 1;
                    },
                }
            },
            EventType::Particle(j) => {
                // FIX: normalize(). Vec2::abs() is componentwise |x|,|y|, so the old
                // "normal" was always a length-sqrt(2) diagonal, not the contact normal.
                let normal = (self.particles[event.particle.index].pos - self.particles[j.index].pos).normalize();
                let relative_vel = (self.particles[event.particle.index].vel - self.particles[j.index].vel).dot(normal);

                // FIX: use each particle's own mass instead of self.mass for both.
                let m_i = self.particles[event.particle.index].mass;
                let m_j = self.particles[j.index].mass;
                let impulse_mag = 2.0 * m_i * m_j * relative_vel / (m_i + m_j);

                self.particles[event.particle.index].vel -= (impulse_mag / m_i) * normal;
                self.particles[j.index].vel += (impulse_mag / m_j) * normal;
                self.particles[event.particle.index].collision_count += 1;
                self.particles[j.index].collision_count += 1;
            }
        }
    }

    pub fn time_to_particle(&self, i: usize, j: usize) -> Option<CollisionEvent> {
        let pos_diff = self.particles[j].pos - self.particles[i].pos;
        let vel_diff = self.particles[j].vel - self.particles[i].vel;
        let total_radius = self.particles[i].radius + self.particles[j].radius;

        let h = pos_diff.dot(vel_diff);
        let a = vel_diff.dot(vel_diff);
        let c = pos_diff.dot(pos_diff) - total_radius.powi(2); // FIX: powi
        let d = h.powi(2) - a * c;                             // FIX: powi
        if h >= 0.0 {
            return None;
        }

        if a == 0.0 {
            return None;
        }

        if d < 0.0 {
            return None;
        }

        let time = (-h - d.sqrt()) / a;

        // FIX: a pair that already overlaps yields a negative root. Dropping it here
        // keeps negative times out of the queue entirely.
        if time < 0.0 {
            return None;
        }

        Some(CollisionEvent {
            particle: ParticleInfo {
                index: i,
                col_count: self.particles[i].collision_count
            },
            abs_time: self.sim_time + time as f64,
            event_type: EventType::Particle(
                ParticleInfo {
                    index: j,
                    col_count: self.particles[j].collision_count }
            )
        })
    }

    pub fn advance(&mut self, dt: f32) {
        let target_time = self.sim_time + dt as f64;

        while self.sim_time < target_time {
            let event = self.heap.peek();
            match event {
                Some(event) => {
                    if target_time < event.abs_time {
                        self.drift(target_time);
                        continue;
                    }
                }
                None => {
                    self.drift(target_time);
                    continue;
                },
            }

            if let Some(event) = self.heap.pop() {
                if self.particles[event.particle.index].collision_count != event.particle.col_count {
                    continue;
                }

                match event.event_type {
                    EventType::Particle(j) => {
                        if self.particles[j.index].collision_count != j.col_count {
                            continue;
                        }

                        self.drift(event.abs_time);
                        self.resolve(event);
                        self.predict(event.particle.index);
                        self.predict(j.index);
                    }
                    Wall(_) => {
                        self.drift(event.abs_time);
                        self.resolve(event);
                        self.predict(event.particle.index);
                    }
                }
            }
        }
    }

    pub fn drift(&mut self, target: f64) {
        for p in self.particles.iter_mut() {
            p.pos += p.vel * (target - self.sim_time) as f32;
        }

        self.sim_time = target;
    }
}