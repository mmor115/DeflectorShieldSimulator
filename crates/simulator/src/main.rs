mod game_systems;
mod physics;
mod game_entities;
mod util;

use bevy::prelude::*;
use bevy_embedded_assets::{EmbeddedAssetPlugin, PluginMode};
use bevy_mod_imgui::prelude::*;
use deflector_core::types::{ParticleState, ParticleStateComponents};
use game_entities::explosion::ExplosionPlugin;
use game_systems::camera_control::CameraControlPlugin;
use game_systems::fixed_update::update_explosions::update_explosions;
use game_systems::fixed_update::validators::*;
use game_systems::fixed_update::*;
use game_systems::frame_update::handle_space_bar::handle_space_bar;
use game_systems::frame_update::*;
use game_systems::history::{take_snapshot, HistoryPlugin};
use game_systems::seeded_rng::SeededRngPlugin;
use game_systems::setup;
use game_systems::tagging::TaggingPlugin;
use game_systems::timers::{PhysicsUpdateTimer, SpaceDustSpawnTimer};
use game_systems::ui::UiPlugin;
use pan_camera::pan_camera;
use post_update_physics::post_update_physics;
use pre_update_physics::pre_update_physics;
use setup::setup;
use spawn_space_dust::*;
use update_bubbles::update_bubbles;
use update_ship::update_ship;
use update_space_dust::update_space_dust;

const PHYSICS_SCALING_FACTOR: f64 = 10.;

const PHYSICS_STEP_SIZE: f64 = 0.1;

fn main() {
    App::new()
        /* Serves the embedded assets/ tree through the default asset source, so the
           load call in setup.rs needs no embedded:// prefix. Must be registered
           before DefaultPlugins. */
        .add_plugins(EmbeddedAssetPlugin { mode: PluginMode::ReplaceDefault })
        .add_plugins(DefaultPlugins)
        .add_plugins(ImguiPlugin::default())
        .add_plugins(MeshPickingPlugin)
        .add_systems(Startup, setup)
        .insert_resource(Time::<Fixed>::from_hz(2000.0))
        .insert_resource(SpaceDustSpawnTimer::default())
        .insert_resource(PhysicsUpdateTimer::default())
        .add_systems(Update, (pan_camera, handle_space_bar))
        .add_systems(
            FixedUpdate, (
                pre_update_physics,
                update_ship,
                (
                    update_explosions,
                    update_bubbles,
                    spawn_space_dust.run_if(spawn_space_dust_predicate)
                ),
                update_space_dust,
                (
                    nan_validator_pre_snapshot.run_if(nan_validator_pre_snapshot_predicate),
                    normalization_validator_pre_snapshot.run_if(normalization_validator_pre_snapshot_predicate)
                ),
                take_snapshot,
                (
                    nan_validator_post_snapshot.run_if(nan_validator_post_snapshot_predicate),
                    normalization_validator_post_snapshot.run_if(normalization_validator_post_snapshot_predicate)
                ),
                post_update_physics
            ).chain(),
        )
        .add_plugins(UiPlugin)
        .add_plugins(SeededRngPlugin)
        .add_plugins(HistoryPlugin)
        .add_plugins(TaggingPlugin)
        .add_plugins(CameraControlPlugin)
        .add_plugins(ExplosionPlugin)
        .run();
}

fn physics_to_game(particle_pos: ParticleState<f64>) -> Vec3 {
    /* In 2d, the z coordinate is only used for ordering. We do want the physical z coordinate
       to influence this, but scaling by PHYSICS_SCALING_FACTOR causes weird frustum culling
       behavior at very high zoom factors. */
    Vec3::new(
        (PHYSICS_SCALING_FACTOR * particle_pos.x()) as f32,
        (PHYSICS_SCALING_FACTOR * particle_pos.y()) as f32,
        particle_pos.z() as f32
    )
}

