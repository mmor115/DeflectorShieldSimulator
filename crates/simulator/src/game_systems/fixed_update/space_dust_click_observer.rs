use bevy::prelude::*;
use deflector_core::types::ParticleStateComponents;
use crate::game_entities::space_dust::{ParticleTypeComponent, SpaceDustColorMaterials, SpaceDustId, SpaceDustPhysics, TaggedSpaceDustMaterialAsset};
use crate::game_systems::tagging::TaggedParticles;

pub fn space_dust_click_observer(mut trigger: Trigger<Pointer<Pressed>>,
                                 mut particles: Query<(&SpaceDustId, &mut MeshMaterial2d<ColorMaterial>, &SpaceDustPhysics, &ParticleTypeComponent)>,
                                 tagged_mat: Res<TaggedSpaceDustMaterialAsset>,
                                 mut tagged_particles: ResMut<TaggedParticles>,
                                 mut materials: ResMut<Assets<ColorMaterial>>,
                                 mut space_dust_materials: ResMut<SpaceDustColorMaterials>) {
    trigger.propagate(false);

    let (
        id,
        mut mat,
        state,
        particle_type
    ) = particles.get_mut(trigger.target).unwrap();

    if tagged_particles.toggle(*id) {
        *mat = MeshMaterial2d(tagged_mat.clone());
    } else {
        *mat = MeshMaterial2d(space_dust_materials.get_space_dust_color(&mut materials, state.z(), &particle_type.0));
    }
}