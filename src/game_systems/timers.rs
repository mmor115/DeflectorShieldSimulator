use bevy::prelude::{Deref, DerefMut, Resource, Timer, TimerMode};

pub const BASE_TICK_RATE: f32 = 60.;
pub const BASE_TICK_INTERVAL: f32 = 1. / BASE_TICK_RATE;


#[derive(Resource, Deref, DerefMut)]
pub struct SpaceDustSpawnTimer(Timer);

impl SpaceDustSpawnTimer {
    pub fn new(tick_interval: f32) -> Self {
        Self(Timer::from_seconds(tick_interval, TimerMode::Repeating))
    }

    pub fn with_new_interval(&self, tick_interval: f32) -> Self {
        let mut t = Self::new(tick_interval);
        t.tick(self.elapsed());
        t
    }
}

impl Default for SpaceDustSpawnTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(BASE_TICK_INTERVAL, TimerMode::Repeating))
    }
}

#[derive(Resource, Deref, DerefMut)]
pub struct PhysicsUpdateTimer(Timer);

impl PhysicsUpdateTimer {
    pub fn new(tick_interval: f32) -> Self {
        Self(Timer::from_seconds(tick_interval, TimerMode::Repeating))
    }

    pub fn with_new_interval(&self, tick_interval: f32) -> Self {
        let mut t = Self::new(tick_interval);
        t.tick(self.elapsed());
        t
    }
}

impl Default for PhysicsUpdateTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(BASE_TICK_INTERVAL, TimerMode::Repeating))
    }
}