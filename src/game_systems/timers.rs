use bevy::prelude::{Deref, DerefMut, Resource, Timer, TimerMode};

const TICK_RATE: f32 = 60.;
const TICK_INTERVAL: f32 = 1. / TICK_RATE;

#[derive(Resource, Deref, DerefMut)]
pub struct SpaceDustSpawnTimer(Timer);

impl Default for SpaceDustSpawnTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(TICK_INTERVAL, TimerMode::Repeating))
    }
}

#[derive(Resource, Deref, DerefMut)]
pub struct PhysicsUpdateTimer(Timer);

impl Default for PhysicsUpdateTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(TICK_INTERVAL, TimerMode::Repeating))
    }
}