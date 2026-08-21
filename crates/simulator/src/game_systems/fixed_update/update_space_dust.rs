use crate::game_entities::ship::Ship;
use crate::game_entities::space_dust::{ParticleTypeComponent, SpaceDust, SpaceDustColorMaterials, SpaceDustId, SpaceDustPhysics, TaggedSpaceDustMaterialAsset};
use crate::game_systems::tagging::TaggedParticles;
use crate::game_systems::timers::PhysicsUpdateTimer;
use crate::game_systems::ui::{PauseControls, VisualSettings};
use crate::physics::physics_manager::PhysicsManager;
use bevy::asset::Assets;
use bevy::math::ops::abs;
use bevy::prelude::*;
use deflector_core::types::ParticleStateComponents;

const DUST_CULL_DRAG_X: f32 = 275.;
const DUST_CULL_DRAG_Y: f32 = 160.;

pub fn update_space_dust(timer: Res<PhysicsUpdateTimer>,
                         par_commands: ParallelCommands,
                         physics: Res<PhysicsManager>,
                         mut materials: ResMut<Assets<ColorMaterial>>,
                         mut space_dust_materials: ResMut<SpaceDustColorMaterials>,
                         mut particles: Query<
                             (Entity, &mut Transform, &mut SpaceDustPhysics, &mut MeshMaterial2d<ColorMaterial>, &SpaceDustId, &mut Visibility, &ParticleTypeComponent),
                             (With<SpaceDust>, Without<Ship>)
                         >,
                         ship_transform: Single<&Transform, With<Ship>>,
                         pause_controls: Res<PauseControls>,
                         tagged_particles: Res<TaggedParticles>,
                         tagged_space_dust_material_asset: Res<TaggedSpaceDustMaterialAsset>,
                         visual_settings: Res<VisualSettings>) {
    if pause_controls.paused {
        return;
    }

    if !timer.just_finished() {
        return;
    }

    particles.par_iter_mut().for_each(|(
        entity_id,
        mut dust_pos,
        mut dust_state,
        _,
        _,
        _,
        particle_type
    )| {
        physics.step_particle(&mut dust_state.0, &particle_type.0);

        dust_pos.translation = crate::physics_to_game(dust_state.0);

        if (dust_pos.translation.x - ship_transform.translation.x).abs() > DUST_CULL_DRAG_X || abs(dust_pos.translation.y) > DUST_CULL_DRAG_Y {
            par_commands.command_scope(|mut commands| {
                commands.entity(entity_id).despawn();
            });
        }
    });

    for (
        _,
        _,
        dust_state,
        mut material,
        id,
        mut visibility,
        particle_type
    ) in particles {
        if tagged_particles.is_tagged(id) {
            *material = MeshMaterial2d(tagged_space_dust_material_asset.0.clone());
            *visibility = Visibility::Visible;
        } else {
            *material = MeshMaterial2d(space_dust_materials.get_space_dust_color(&mut materials, dust_state.z(), &particle_type.0));
            *visibility = if visual_settings.hide_untagged_particles {
                Visibility::Hidden
            } else {
                Visibility::Visible
            };
        }
    }
}