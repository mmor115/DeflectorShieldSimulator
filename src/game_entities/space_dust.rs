use crate::game_systems::seeded_rng::SeededRng;
use crate::game_systems::ui::ParticleSettings;
use crate::physics::physics_manager::PhysicsManager;
use crate::PHYSICS_SCALING_FACTOR;
use bevy::asset::{Assets, Handle};
use bevy::color::Color;
use bevy::math::{Vec2, Vec3};
use bevy::prelude::*;
use deflector_core::types::{ParticleState, ParticleStateComponents, ParticleType};
use derive_more::with_trait::Into;
use derive_more::{Display, From};
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

const DUST_DIAMETER: f32 = 1.;

#[derive(Component)]
#[require(Transform, Pickable, Visibility)]
pub struct SpaceDust;

#[derive(Component, Deref, DerefMut, Debug, Serialize, Deserialize, From, Clone)]
pub struct SpaceDustPhysics(pub ParticleState<f64>);

#[derive(Resource, Deref)]
pub struct SpaceDustMesh(pub Handle<Mesh>);

#[derive(Resource)]
pub struct SpaceDustColorMaterials {
    massive_particle_materials: HashMap<i64, Handle<ColorMaterial>>,
    photon_particle_materials: HashMap<i64, Handle<ColorMaterial>>
}

#[derive(Resource, Deref)]
pub struct TaggedSpaceDustMaterialAsset(pub Handle<ColorMaterial>);

impl TaggedSpaceDustMaterialAsset {
    pub fn new(mat: Handle<ColorMaterial>) -> Self {
        Self(mat)
    }
}

impl SpaceDustColorMaterials {
    pub fn new() -> Self {
        Self {
            massive_particle_materials: HashMap::new(),
            photon_particle_materials: HashMap::new()
        }
    }

    pub fn get_space_dust_color(&mut self,
                                materials: &mut ResMut<Assets<ColorMaterial>>,
                                mut physics_z: f64,
                                particle_type: &ParticleType) -> Handle<ColorMaterial> {
        const Z_MIN: f64 = -150. / PHYSICS_SCALING_FACTOR;
        const Z_MAX: f64 = 150. / PHYSICS_SCALING_FACTOR;

        physics_z = physics_z.floor();
        let key = physics_z as i64;

        let interpolation = ((physics_z - Z_MIN) / (Z_MAX - Z_MIN)).clamp(0., 1.);
        let color = match particle_type {
            ParticleType::Massive => Color::srgb(1.0, interpolation as f32, 0.),
            ParticleType::Photon => Color::srgb(0.0, interpolation as f32, 1. - interpolation as f32)
        };

        let materials_bin = match particle_type {
            ParticleType::Massive => &mut self.massive_particle_materials,
            ParticleType::Photon => &mut self.photon_particle_materials
        };

        if let Some(weak_handle) = materials_bin.get_mut(&key) {
            if let Some(strong_handle) = materials.get_strong_handle(weak_handle.id()) {
                return strong_handle;
            }
        }

        let material = materials.add(color);
        materials_bin.insert(key, material.clone_weak());

        material
    }
}

#[derive(Serialize, Deserialize)]
#[serde(remote = "ParticleType")]
pub enum ParticleTypeDef {
    Massive,
    Photon
}

#[derive(Component, Serialize, Deserialize, Copy, Clone, From, Into, Debug)]
pub struct ParticleTypeComponent(#[serde(with = "ParticleTypeDef")] pub ParticleType);

#[derive(Bundle)]
pub struct SpaceDustEntity(Mesh2d, MeshMaterial2d<ColorMaterial>, Transform, SpaceDust, SpaceDustPhysics, SpaceDustId, ParticleTypeComponent);

#[derive(Component, From, Clone, Copy, Serialize, Deserialize, Debug, Display, PartialEq, Eq, Hash)]
pub struct SpaceDustId(pub Uuid);

impl SpaceDustEntity {
    pub fn new_from_spawn(starting_position: Vec3,
                          mesh: &Res<SpaceDustMesh>,
                          mat: Handle<ColorMaterial>,
                          physics_manager: &Res<PhysicsManager>,
                          particle_settings: &Res<ParticleSettings>,
                          rng: &mut ResMut<SeededRng>,
                          id: SpaceDustId,
                          particle_type: ParticleType) -> Self {
        SpaceDustEntity(
            Mesh2d(mesh.0.clone()),
            MeshMaterial2d(mat),
            Transform::from_translation(starting_position).with_scale(Vec2::splat(DUST_DIAMETER).extend(1.)),
            SpaceDust,
            get_state_for_new_particle(starting_position, particle_settings, physics_manager, rng, particle_type),
            id,
            particle_type.into()
        )
    }

    pub fn new_from_resume(mesh: &Res<SpaceDustMesh>,
                           mut color_materials: &mut ResMut<Assets<ColorMaterial>>,
                           space_dust_mats: &mut ResMut<SpaceDustColorMaterials>,
                           state: SpaceDustPhysics,
                           id: SpaceDustId,
                           particle_type: ParticleType) -> Self {
        let mat = space_dust_mats.get_space_dust_color(
            &mut color_materials,
            state.z(),
            &particle_type
        );

        SpaceDustEntity(
            Mesh2d(mesh.0.clone()),
            MeshMaterial2d(mat),
            Transform::from_translation(crate::physics_to_game(*state)).with_scale(Vec2::splat(DUST_DIAMETER).extend(1.)),
            SpaceDust,
            state,
            id,
            particle_type.into()
        )
    }

    pub fn new_tagged_from_resume(mesh: &Res<SpaceDustMesh>,
                                  mat: &Res<TaggedSpaceDustMaterialAsset>,
                                  state: SpaceDustPhysics,
                                  id: SpaceDustId,
                                  particle_type: ParticleType) -> Self {
        SpaceDustEntity(
            Mesh2d(mesh.0.clone()),
            MeshMaterial2d(mat.0.clone()),
            Transform::from_translation(crate::physics_to_game(*state)).with_scale(Vec2::splat(DUST_DIAMETER).extend(1.)),
            SpaceDust,
            state,
            id,
            particle_type.into()
        )
    }
}

fn get_state_for_new_particle(particle_pos: Vec3,
                              particle_settings: &Res<ParticleSettings>,
                              physics_manager: &Res<PhysicsManager>,
                              rng: &mut ResMut<SeededRng>,
                              particle_type: ParticleType) -> SpaceDustPhysics {
    physics_manager.new_particle_state(
        particle_pos.x as f64 / PHYSICS_SCALING_FACTOR,
        particle_pos.y as f64 / PHYSICS_SCALING_FACTOR,
        particle_pos.z as f64 / PHYSICS_SCALING_FACTOR,
        rng.random_range(-particle_settings.x_velocity_variance ..= particle_settings.x_velocity_variance),
        rng.random_range(-particle_settings.y_velocity_variance ..= particle_settings.y_velocity_variance),
        rng.random_range(-particle_settings.z_velocity_variance ..= particle_settings.z_velocity_variance),
        particle_type
    ).into()
}