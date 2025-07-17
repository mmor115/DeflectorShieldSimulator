use crate::game_entities::ship::Ship;
use crate::game_entities::space_dust::{SpaceDustColorMaterials, SpaceDustEntity, SpaceDustId, SpaceDustMesh, TaggedSpaceDustMaterialAsset};
use crate::game_systems::seeded_rng::SeededRng;
use crate::game_systems::timers::SpaceDustSpawnTimer;
use crate::game_systems::ui::{ParticleSettings, PauseControls, ShutdownState};
use crate::physics::physics_manager::PhysicsManager;
use crate::PHYSICS_SCALING_FACTOR;
use bevy::asset::Assets;
use bevy::math::Vec3;
use bevy::prelude::{ColorMaterial, Commands, Res, ResMut, Single, Time, Transform, With};
use rand::Rng;
use crate::game_systems::fixed_update::space_dust_click_observer::space_dust_click_observer;
use crate::game_systems::tagging::TaggedParticles;

const DUST_SPAWN_LEAD: f32 = 275.;

pub fn spawn_space_dust(time: Res<Time>,
                        mut timer: ResMut<SpaceDustSpawnTimer>,
                        mut commands: Commands,
                        mut materials: ResMut<Assets<ColorMaterial>>,
                        mut space_dust_mats: ResMut<SpaceDustColorMaterials>,
                        mut rng: ResMut<SeededRng>,
                        mesh: Res<SpaceDustMesh>,
                        physics_manager: Res<PhysicsManager>,
                        ship_transform: Single<&Transform, With<Ship>>,
                        particle_settings: Res<ParticleSettings>,
                        shutdown_state: Res<ShutdownState>,
                        pause_controls: Res<PauseControls>,
                        tagged_particles: Res<TaggedParticles>,
                        tagged_space_dust_material_asset: Res<TaggedSpaceDustMaterialAsset>) {
    if pause_controls.paused {
        return;
    }

    if !timer.tick(time.delta()).just_finished() {
        return;
    }

    if shutdown_state.in_shutdown_state {
        return;
    }

    let y_position_variance = particle_settings.y_position_variance;
    let z_position_variance = particle_settings.z_position_variance;

    let pos = Vec3::new(
        ship_transform.translation.x + DUST_SPAWN_LEAD,
        rng.random_range(-y_position_variance..=y_position_variance),
        rng.random_range(-z_position_variance..=z_position_variance),
    );

    let id = SpaceDustId(uuid::Builder::from_random_bytes(rng.random()).into_uuid());

    let mat = if tagged_particles.is_tagged(&id) {
        tagged_space_dust_material_asset.0.clone()
    } else {
        space_dust_mats.get_space_dust_color(
            &mut materials,
            pos.z as f64 / PHYSICS_SCALING_FACTOR,
        )
    };

    commands.spawn(SpaceDustEntity::new_from_spawn(pos, &mesh, mat, &physics_manager, &particle_settings, &mut rng, id))
        .observe(space_dust_click_observer);
}