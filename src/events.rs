use std::cmp::Ordering;

#[derive(Clone, Copy)]
pub struct ParticleInfo {
    pub index: usize,
    pub col_count: u64,
}

#[derive(Clone, Copy)]
pub enum EventType {
    Wall(Axis),
    Particle(ParticleInfo)
}

#[derive(Clone, Copy)]
pub enum Axis {
    X,
    Y
}

#[derive(Clone, Copy)]
pub struct CollisionEvent {
    pub particle: ParticleInfo,
    pub abs_time: f64,
    pub event_type: EventType,
}

impl Eq for CollisionEvent {}

impl PartialEq<Self> for CollisionEvent {
    fn eq(&self, other: &Self) -> bool {
        self.abs_time == other.abs_time
    }
}

impl PartialOrd<Self> for CollisionEvent {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for CollisionEvent {
    fn cmp(&self, other: &Self) -> Ordering { other.abs_time.total_cmp(&self.abs_time) }
}