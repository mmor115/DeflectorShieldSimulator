mod game_systems;
mod physics;
mod game_entities;

use bevy::prelude::*;
use bevy_mod_imgui::prelude::*;
use deflector_core::types::{ParticleState, ParticleStateComponents};
use game_systems::fixed_update::{pre_update_physics, spawn_space_dust, update_bubbles, update_ship, update_space_dust};
use game_systems::frame_update::pan_camera::pan_camera;
use game_systems::history::{take_snapshot, HistoryPlugin};
use game_systems::seeded_rng::SeededRngPlugin;
use game_systems::setup;
use game_systems::timers::{PhysicsUpdateTimer, SpaceDustSpawnTimer};
use game_systems::ui::UiPlugin;
use pre_update_physics::pre_update_physics;
use setup::setup;
use spawn_space_dust::spawn_space_dust;
use update_bubbles::update_bubbles;
use update_ship::update_ship;
use update_space_dust::update_space_dust;

const PHYSICS_SCALING_FACTOR: f64 = 10.;

const PHYSICS_STEP_SIZE: f64 = 0.1;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(ImguiPlugin::default())
        .add_systems(Startup, setup)
        .insert_resource(SpaceDustSpawnTimer::default())
        .insert_resource(PhysicsUpdateTimer::default())
        .add_systems(Update, pan_camera)
        .add_systems(
            FixedUpdate, (
                pre_update_physics,
                update_ship,
                update_bubbles,
                spawn_space_dust,
                update_space_dust,
                take_snapshot
            ).chain()
        )
        .add_plugins(UiPlugin)
        .add_plugins(SeededRngPlugin)
        .add_plugins(HistoryPlugin)
        .run();
}

fn physics_to_game(particle_pos: ParticleState<f64>) -> Vec3 {
    Vec3::new(
        (PHYSICS_SCALING_FACTOR * particle_pos.x()) as f32,
        (PHYSICS_SCALING_FACTOR * particle_pos.y()) as f32,
        (PHYSICS_SCALING_FACTOR * particle_pos.z()) as f32,
    )
}

