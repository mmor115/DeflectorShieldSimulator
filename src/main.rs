mod physics_manager;
mod physics_parameters;
mod ui;
mod config;
mod history;
mod seeded_rng;

use std::collections::HashMap;
use bevy::math::ops::abs;
use crate::physics_manager::PhysicsManager;
use crate::physics_parameters::PhysicsParameters;
use bevy::prelude::*;
use bevy_mod_imgui::prelude::*;
use deflector_core::types::{ParticleState, ParticleStateComponents};
use derive_more::From;
use rand::Rng;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::history::{take_snapshot, HistoryPlugin};
use crate::seeded_rng::{SeededRng, SeededRngPlugin};
use crate::ui::{ParticleSettings, ShutdownState, UiPlugin, UiState, VisualSettings};

const INITIAL_SHIP_POS: Vec3 = Vec3::new(0., 0., 0.);
const SHIP_SCALE: Vec3 = Vec3::new(0.05, 0.05, 1.);

const DUST_DIAMETER: f32 = 1.;

const DUST_SPAWN_LEAD: f32 = 275.;
const DUST_CULL_DRAG_X: f32 = 275.;
const DUST_CULL_DRAG_Y: f32 = 160.;

const INNER_BUBBLE_COLOR: Color = Color::srgb(0.0, 0.5, 1.0);
const OUTER_BUBBLE_COLOR: Color = Color::srgb(0.0, 0.0, 1.0);

const PHYSICS_SCALING_FACTOR: f64 = 10.;

const PHYSICS_STEP_SIZE: f64 = 0.1;

const CAMERA_ZOOM: f32 = 2.5;

const TICK_RATE: f32 = 60.;
const TICK_INTERVAL: f32 = 1. / TICK_RATE;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(ImguiPlugin::default())
        .add_systems(Startup, setup)
        .insert_resource(SpaceDustSpawnTimer::default())
        .insert_resource(PhysicsUpdateTimer::default())
        .add_systems(Update, pan_camera)
        .add_systems(FixedUpdate, (pre_update_physics, update_ship, update_bubbles, spawn_space_dust, update_space_dust, take_snapshot).chain())
        .add_plugins(UiPlugin)
        .add_plugins(SeededRngPlugin)
        .add_plugins(HistoryPlugin)
        .run();
}

fn setup(mut commands: Commands, 
         mut meshes: ResMut<Assets<Mesh>>,
         mut materials: ResMut<Assets<ColorMaterial>>,
         asset_server: Res<AssetServer>) {
    commands.spawn((
        Camera2d,
        Transform::from_translation(INITIAL_SHIP_POS).with_scale(Vec3::splat(1. / CAMERA_ZOOM))
    ));

    let ship_image = asset_server.load::<Image>("images/ship.png");
    commands.insert_resource(ShipImageAsset::new(ship_image.clone()));

    let space_dust_mesh = meshes.add(Circle::default());
    commands.insert_resource(SpaceDustMesh(space_dust_mesh));
    
    commands.insert_resource(SpaceDustColorMaterials::new());

    let physics_manager = PhysicsManager::new(
        PhysicsParameters::default(),
        PHYSICS_STEP_SIZE
    );

    let params = &physics_manager.physics_parameters;

    commands.spawn(
        ShipEntity::new_from_image_handle(
            ship_image,
            ShipPhysics(physics_manager.new_ship_particle_state())
        )
    );
    
    commands.spawn((
        InnerBubble,
        make_inner_bubble_mesh(&mut meshes, params),
        MeshMaterial2d(materials.add(INNER_BUBBLE_COLOR)),
        Transform::from_translation(INITIAL_SHIP_POS)
    ));
    
    commands.spawn((
        OuterBubble,
        make_outer_bubble_mesh(&mut meshes, params),
        MeshMaterial2d(materials.add(OUTER_BUBBLE_COLOR)),
        Transform::from_translation(INITIAL_SHIP_POS)
    ));

    commands.insert_resource(physics_manager);
}

fn pan_camera(mut camera2d: Single<&mut Transform, (With<Camera2d>, Without<Ship>)>,
              ship: Single<&Transform, With<Ship>>) {
    camera2d.translation = ship.translation;
}

fn spawn_space_dust(time: Res<Time>,
                    mut timer: ResMut<SpaceDustSpawnTimer>,
                    mut commands: Commands,
                    mut materials: ResMut<Assets<ColorMaterial>>,
                    mut space_dust_mats: ResMut<SpaceDustColorMaterials>,
                    mut rng: ResMut<SeededRng>,
                    mesh: Res<SpaceDustMesh>,
                    physics_manager: Res<PhysicsManager>,
                    ship_transform: Single<&Transform, With<Ship>>,
                    particle_settings: Res<ParticleSettings>,
                    shutdown_state: Res<ShutdownState>) {
    if !timer.tick(time.delta()).just_finished() {
        return;
    }

    if shutdown_state.in_shutdown_state {
        return;
    }

    let pos = Vec3::new(
        ship_transform.translation.x + DUST_SPAWN_LEAD,
        rng.random_range(-particle_settings.y_position_variance ..= particle_settings.y_position_variance),
        rng.random_range(-particle_settings.z_position_variance ..= particle_settings.z_position_variance),
    );

    let mat = space_dust_mats.get_space_dust_color(
        &mut materials,
        pos.z as f64 / PHYSICS_SCALING_FACTOR
    );

    commands.spawn(SpaceDustEntity::new_from_spawn(pos, mesh, mat, physics_manager, particle_settings, rng));
}

fn update_ship(timer: ResMut<PhysicsUpdateTimer>,
               physics: Res<PhysicsManager>,
               ship: Single<(&mut Transform, &mut ShipPhysics), With<Ship>>) {
    if !timer.just_finished() {
        return;
    }

    let (mut ship_transform, mut ship_state) = ship.into_inner();

    physics.step_particle(&mut ship_state.0);
    ship_transform.translation = physics_to_game(ship_state.0);
}

fn update_bubbles(timer: Res<PhysicsUpdateTimer>,
                  visual_settings: Res<VisualSettings>,
                  mut inner_bubble: Single<(&mut Transform, &mut Visibility, &mut Mesh2d), (With<InnerBubble>, Without<Ship>)>,
                  mut outer_bubble: Single<(&mut Transform, &mut Visibility, &mut Mesh2d), (With<OuterBubble>, Without<Ship>, Without<InnerBubble>)>,
                  mut meshes: ResMut<Assets<Mesh>>,
                  mut ui_state: ResMut<UiState>,
                  shutdown_state: Res<ShutdownState>,
                  physics: Res<PhysicsManager>) {
    if !timer.just_finished() {
        return;
    }

    if shutdown_state.in_shutdown_state {
        *inner_bubble.1 = Visibility::Hidden;
        *outer_bubble.1 = Visibility::Hidden;
        return;
    }

    for bubble_translation in [&mut inner_bubble.0.translation, &mut outer_bubble.0.translation] {
        bubble_translation.x = (PHYSICS_SCALING_FACTOR * physics.bubble_x_position()) as f32;
    }

    *inner_bubble.1 = match visual_settings.show_inner_bubble {
        true => Visibility::Visible,
        false => Visibility::Hidden
    };

    *outer_bubble.1 = match visual_settings.show_outer_bubble {
        true => Visibility::Visible,
        false => Visibility::Hidden
    };

    if ui_state.need_remesh_inner_bubble {
        meshes.remove(inner_bubble.2.id()).unwrap();
        *inner_bubble.2 = make_inner_bubble_mesh(&mut meshes, &physics.physics_parameters);
        ui_state.need_remesh_inner_bubble = false;
    }

    if ui_state.need_remesh_outer_bubble {
        meshes.remove(outer_bubble.2.id()).unwrap();
        *outer_bubble.2 = make_outer_bubble_mesh(&mut meshes, &physics.physics_parameters);
        ui_state.need_remesh_outer_bubble = false;
    }
}

fn update_space_dust(timer: Res<PhysicsUpdateTimer>,
                     mut commands: Commands,
                     physics: Res<PhysicsManager>,
                     mut materials: ResMut<Assets<ColorMaterial>>,
                     mut space_dust_materials: ResMut<SpaceDustColorMaterials>,
                     particles: Query<(Entity, &mut Transform, &mut SpaceDustPhysics, &mut MeshMaterial2d<ColorMaterial>), (With<SpaceDust>, Without<Ship>)>,
                     ship_transform: Single<&Transform, With<Ship>>) {
    if !timer.just_finished() {
        return;
    }

    for (entity_id, mut dust_pos, mut dust_state, mut material) in particles {
        physics.step_particle(&mut dust_state.0);

        dust_pos.translation = physics_to_game(dust_state.0);

        *material = MeshMaterial2d(space_dust_materials.get_space_dust_color(&mut materials, dust_state.z()));

        if dust_pos.translation.x < ship_transform.translation.x - DUST_CULL_DRAG_X
           || abs(dust_pos.translation.y) > DUST_CULL_DRAG_Y {
            commands.entity(entity_id).despawn();
        }
    }
}

fn pre_update_physics(time: Res<Time>,
                      mut timer: ResMut<PhysicsUpdateTimer>,
                      mut physics: ResMut<PhysicsManager>) {
    if !timer.tick(time.delta()).just_finished() {
        return;
    }

    physics.incr_global_time();
}

#[derive(Resource, Deref, DerefMut)]
struct SpaceDustSpawnTimer(Timer);

impl Default for SpaceDustSpawnTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(TICK_INTERVAL, TimerMode::Repeating))
    }
}

#[derive(Resource, Deref, DerefMut)]
struct PhysicsUpdateTimer(Timer);

impl Default for PhysicsUpdateTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(TICK_INTERVAL, TimerMode::Repeating))
    }
}

#[derive(Component)]
#[require(Sprite, Transform)]
struct Ship;

#[derive(Component)]
#[require(Transform, Visibility)]
struct InnerBubble;

#[derive(Component)]
#[require(Transform, Visibility)]
struct OuterBubble;

fn make_bubble_mesh(meshes: &mut ResMut<Assets<Mesh>>, physics_radius: f64) -> Mesh2d {
    let bubble_radius = (physics_radius * PHYSICS_SCALING_FACTOR) as f32;
    let annulus = Annulus::new(bubble_radius - 2.0, bubble_radius);
    Mesh2d(meshes.add(annulus))
}

fn make_inner_bubble_mesh(meshes: &mut ResMut<Assets<Mesh>>, 
                          physics_parameters: &PhysicsParameters) -> Mesh2d {
    make_bubble_mesh(meshes, physics_parameters.bubble_radius())
}

fn make_outer_bubble_mesh(meshes: &mut ResMut<Assets<Mesh>>,
                          physics_parameters: &PhysicsParameters) -> Mesh2d {
    make_bubble_mesh(meshes, physics_parameters.bubble_radius() + physics_parameters.bubble_sigma())
}

#[derive(Component)]
#[require(Transform)]
struct SpaceDust;

#[derive(Component, Deref, DerefMut, Debug, Serialize, Deserialize, From, Clone)]
struct SpaceDustPhysics(ParticleState<f64>);

#[derive(Component, Deref, DerefMut, Debug, Serialize, Deserialize, From, Clone)]
struct ShipPhysics(ParticleState<f64>);

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

fn physics_to_game(particle_pos: ParticleState<f64>) -> Vec3 {
    Vec3::new(
        (PHYSICS_SCALING_FACTOR * particle_pos.x()) as f32,
        (PHYSICS_SCALING_FACTOR * particle_pos.y()) as f32,
        (PHYSICS_SCALING_FACTOR * particle_pos.z()) as f32
    )
}

#[derive(Resource, Deref)]
struct SpaceDustMesh(Handle<Mesh>);

#[derive(Resource)]
struct SpaceDustColorMaterials(HashMap<i64, Handle<ColorMaterial>>);

impl SpaceDustColorMaterials {
    fn new() -> Self {
        Self(HashMap::new())
    }

    fn get_space_dust_color(&mut self,
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
struct SpaceDustEntity(Mesh2d, MeshMaterial2d<ColorMaterial>, Transform, SpaceDust, SpaceDustPhysics, SpaceDustId);

#[derive(Component, From, Clone, Serialize, Deserialize)]
struct SpaceDustId(Uuid);

impl SpaceDustEntity {
    fn new_from_spawn(starting_position: Vec3,
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

    fn new_from_resume(mesh: &Res<SpaceDustMesh>,
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
            Transform::from_translation(physics_to_game(*state)).with_scale(Vec2::splat(DUST_DIAMETER).extend(1.)),
            SpaceDust,
            state,
            id
        )
    }
}

#[derive(Resource, Deref)]
struct ShipImageAsset(Handle<Image>);

impl ShipImageAsset {
    fn new(img: Handle<Image>) -> Self {
        ShipImageAsset(img)
    }
}

#[derive(Bundle)]
struct ShipEntity(Sprite, Transform, Ship, ShipPhysics);

impl ShipEntity {
    fn new(ship_image_asset: Res<ShipImageAsset>, state: ShipPhysics) -> Self {
        Self::new_from_image_handle(ship_image_asset.clone(), state)
    }

    fn new_from_image_handle(ship_image: Handle<Image>, state: ShipPhysics) -> Self {
        let translation = physics_to_game(*state);
        ShipEntity(
            Sprite::from_image(ship_image),
            Transform::from_translation(translation).with_scale(SHIP_SCALE),
            Ship,
            state
        )
    }
}
