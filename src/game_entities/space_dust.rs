use crate::game_systems::seeded_rng::SeededRng;
use crate::game_systems::ui::ParticleSettings;
use crate::physics::physics_manager::PhysicsManager;
use crate::PHYSICS_SCALING_FACTOR;
use bevy::asset::{Assets, Handle};
use bevy::color::Color;
use bevy::math::{Vec2, Vec3};
use bevy::prelude::{Bundle, ColorMaterial, Component, Deref, DerefMut, Mesh, Mesh2d, MeshMaterial2d, Res, ResMut, Resource, Transform};
use deflector_core::types::{ParticleState, ParticleStateComponents};
use derive_more::From;
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Component)]
#[require(Transform)]
pub struct SpaceDust;

#[derive(Component, Deref, DerefMut, Debug, Serialize, Deserialize, From, Clone)]
pub struct SpaceDustPhysics(pub ParticleState<f64>);

#[derive(Resource, Deref)]
pub struct SpaceDustMesh(pub Handle<Mesh>);

#[derive(Resource)]
pub struct SpaceDustColorMaterials(HashMap<i64, Handle<ColorMaterial>>);

impl SpaceDustColorMaterials {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn get_space_dust_color(&mut self,
                            materials: &mut ResMut<Assets<ColorMaterial>>,
                            mut physics_z: f64) -> Handle<ColorMaterial> {
        const Z_MIN: f64 = -150. / PHYSICS_SCALING_FACTOR;
        const Z_MAX: f64 = 150. / PHYSICS_SCALING_FACTOR;

        physics_z = physics_z.floor();
        let key = physics_z as i64;

        let interpolation = ((physics_z - Z_MIN) / (Z_MAX - Z_MIN)).clamp(0., 1.);
        let color = Color::srgb(1.0, interpolation as f32, 0.);

        if let Some(weak_handle) = self.0.get_mut(&key) {
            if let Some(strong_handle) = materials.get_strong_handle(weak_handle.id()) {
                return strong_handle;
            }
        }

        let material = materials.add(color);
        self.0.insert(key, material.clone_weak());

        material
    }
}

#[derive(Bundle)]
pub struct SpaceDustEntity(Mesh2d, MeshMaterial2d<ColorMaterial>, Transform, SpaceDust, SpaceDustPhysics, SpaceDustId);

#[derive(Component, From, Clone, Serialize, Deserialize)]
pub struct SpaceDustId(Uuid);

impl SpaceDustEntity {
    pub fn new_from_spawn(starting_position: Vec3,
                      mesh: Res<SpaceDustMesh>,
                      mat: Handle<ColorMaterial>,
                      physics_manager: Res<PhysicsManager>,
                      particle_settings: Res<ParticleSettings>,
                      rng: ResMut<SeededRng>) -> Self {
        SpaceDustEntity(
            Mesh2d(mesh.0.clone()),
            MeshMaterial2d(mat),
            Transform::from_translation(starting_position).with_scale(Vec2::splat(DUST_DIAMETER).extend(1.)),
            SpaceDust,
            get_state_for_new_particle(starting_position, particle_settings, physics_manager, rng),
            Uuid::new_v4().into()
        )
    }

    pub fn new_from_resume(mesh: &Res<SpaceDustMesh>,
                       mut color_materials: &mut ResMut<Assets<ColorMaterial>>,
                       space_dust_mats: &mut ResMut<SpaceDustColorMaterials>,
                       state: SpaceDustPhysics,
                       id: SpaceDustId) -> Self {
        let mat = space_dust_mats.get_space_dust_color(
            &mut color_materials,
            state.z()
        );

        SpaceDustEntity(
            Mesh2d(mesh.0.clone()),
            MeshMaterial2d(mat),
            Transform::from_translation(crate::physics_to_game(*state)).with_scale(Vec2::splat(DUST_DIAMETER).extend(1.)),
            SpaceDust,
            state,
            id
        )
    }
}

fn get_state_for_new_particle(particle_pos: Vec3,
                              particle_settings: Res<ParticleSettings>,
                              physics_manager: Res<PhysicsManager>,
                              mut rng: ResMut<SeededRng>) -> SpaceDustPhysics {
    physics_manager.new_particle_state(
        particle_pos.x as f64 / PHYSICS_SCALING_FACTOR,
        particle_pos.y as f64 / PHYSICS_SCALING_FACTOR,
        particle_pos.z as f64 / PHYSICS_SCALING_FACTOR,
        rng.random_range(-particle_settings.x_velocity_variance ..= particle_settings.x_velocity_variance),
        rng.random_range(-particle_settings.y_velocity_variance ..= particle_settings.y_velocity_variance),
        rng.random_range(-particle_settings.z_velocity_variance ..= particle_settings.z_velocity_variance)
    ).into()
}

const DUST_DIAMETER: f32 = 1.;