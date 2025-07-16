use crate::game_entities::ship::Ship;
use crate::game_entities::space_dust::{SpaceDust, SpaceDustColorMaterials, SpaceDustPhysics};
use crate::game_systems::timers::PhysicsUpdateTimer;
use crate::physics::physics_manager::PhysicsManager;
use bevy::asset::Assets;
use bevy::math::ops::abs;
use bevy::prelude::{ColorMaterial, Commands, Entity, MeshMaterial2d, Query, Res, ResMut, Single, Transform, With, Without};
use deflector_core::types::ParticleStateComponents;
use crate::game_systems::ui::PauseControls;

const DUST_CULL_DRAG_X: f32 = 275.;
const DUST_CULL_DRAG_Y: f32 = 160.;

pub fn update_space_dust(timer: Res<PhysicsUpdateTimer>,
                         mut commands: Commands,
                         physics: Res<PhysicsManager>,
                         mut materials: ResMut<Assets<ColorMaterial>>,
                         mut space_dust_materials: ResMut<SpaceDustColorMaterials>,
                         particles: Query<(Entity, &mut Transform, &mut SpaceDustPhysics, &mut MeshMaterial2d<ColorMaterial>), (With<SpaceDust>, Without<Ship>)>,
                         ship_transform: Single<&Transform, With<Ship>>,
                         pause_controls: Res<PauseControls>) {
    if pause_controls.paused {
        return;
    }

    if !timer.just_finished() {
        return;
    }

    for (entity_id, mut dust_pos, mut dust_state, mut material) in particles {
        physics.step_particle(&mut dust_state.0);

        dust_pos.translation = crate::physics_to_game(dust_state.0);

        *material = MeshMaterial2d(space_dust_materials.get_space_dust_color(&mut materials, dust_state.z()));

        if dust_pos.translation.x < ship_transform.translation.x - DUST_CULL_DRAG_X
           || abs(dust_pos.translation.y) > DUST_CULL_DRAG_Y {
            commands.entity(entity_id).despawn();
        }
    }
}