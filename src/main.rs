mod physics_manager;
mod physics_parameters;
mod ui;

use crate::physics_manager::PhysicsManager;
use crate::physics_parameters::PhysicsParameters;
use bevy::prelude::*;
use bevy_mod_imgui::prelude::*;
use deflector_core::types::{ParticleState, ParticleStateComponents};
use rand::Rng;
use crate::ui::{UiPlugin, UiState, VisualSettings};

const INITIAL_SHIP_POS: Vec3 = Vec3::new(0., 0., 0.);
const SHIP_SCALE: Vec3 = Vec3::new(0.05, 0.05, 1.);

const DUST_COLOR: Color = Color::srgb(1.0, 0.5, 0.5);
const DUST_DIAMETER: f32 = 1.;

const DUST_SPAWN_LEAD: f32 = 275.;
const DUST_CULL_DRAG: f32 = 275.;
const DUST_Y_SPAWN_MIN: f32 = -150.;
const DUST_Y_SPAWN_MAX: f32 = 150.;

const INNER_BUBBLE_COLOR: Color = Color::srgb(0.0, 0.5, 1.0);
const OUTER_BUBBLE_COLOR: Color = Color::srgb(0.0, 0.0, 1.0);

const PHYSICS_SCALING_FACTOR: f64 = 10.;

const PHYSICS_STEP_SIZE: f64 = 0.1;

const CAMERA_ZOOM: f32 = 2.5;

const TICK_RATE: f32 = 60.;
const TICK_INTERVAL: f32 = 1. / TICK_RATE;

const SHOW_INNER_BUBBLE: bool = true;
const SHOW_OUTER_BUBBLE: bool = true;


fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(ImguiPlugin::default())
        .add_systems(Startup, setup)
        .insert_resource(SpaceDustSpawnTimer::default())
        .insert_resource(PhysicsUpdateTimer::default())
        .add_systems(Update, pan_camera)
        .add_systems(FixedUpdate, (update_ship, update_bubbles, spawn_space_dust, update_space_dust).chain())
        .add_plugins(UiPlugin)
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

    let space_dust_mesh = meshes.add(Circle::default());
    commands.insert_resource(SpaceDustMesh(space_dust_mesh));
    
    let space_dust_material = materials.add(DUST_COLOR);
    commands.insert_resource(SpaceDustMaterial(space_dust_material));

    let physics_manager = PhysicsManager::new(
        PhysicsParameters::default(),
        PHYSICS_STEP_SIZE
    );

    let params = &physics_manager.physics_parameters;

    commands.spawn((
        Sprite::from_image(ship_image),
        Transform::from_translation(INITIAL_SHIP_POS).with_scale(SHIP_SCALE),
        Ship,
        ShipPhysics(physics_manager.new_ship_particle_state(params.covariant_ship_speed()))
    ));
    
    commands.spawn((
        InnerBubble,
        make_inner_bubble_mesh(&mut meshes, params),
        MeshMaterial2d(materials.add(INNER_BUBBLE_COLOR)),
        Transform::from_translation(INITIAL_SHIP_POS.xy().extend(-5.))
    ));
    
    commands.spawn((
        OuterBubble,
        make_outer_bubble_mesh(&mut meshes, params),
        MeshMaterial2d(materials.add(OUTER_BUBBLE_COLOR)),
        Transform::from_translation(INITIAL_SHIP_POS.xy().extend(-5.))
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
                    mesh: Res<SpaceDustMesh>,
                    mat: Res<SpaceDustMaterial>,
                    physics_manager: Res<PhysicsManager>,
                    ship: Single<(&Transform, &ShipPhysics), With<Ship>>) {
    if !timer.tick(time.delta()).just_finished() {
        return;
    }

    let (ship_transform, ship_physics) = ship.into_inner();

    let mut rng = rand::rng();
    let pos = Vec3::new(
        ship_transform.translation.x + DUST_SPAWN_LEAD,
        rng.random_range(DUST_Y_SPAWN_MIN ..= DUST_Y_SPAWN_MAX),
        0.
    );

    commands.spawn(SpaceDust::new_entity(pos, ship_physics, mesh, mat, physics_manager));
}

fn update_ship(time: Res<Time>,
               mut timer: ResMut<PhysicsUpdateTimer>,
               physics: Res<PhysicsManager>,
               ship: Single<(&mut Transform, &mut ShipPhysics), With<Ship>>) {
    if !timer.tick(time.delta()).just_finished() {
        return;
    }

    let (mut ship_transform, mut ship_state) = ship.into_inner();

    physics.step_particle(&mut ship_state.0);
    ship_transform.translation = physics_to_game(ship_state.0).xy().extend(-10.);
}

fn update_bubbles(timer: Res<PhysicsUpdateTimer>,
                  visual_settings: Res<VisualSettings>,
                  ship_transform: Single<&Transform, With<Ship>>,
                  mut inner_bubble: Single<(&mut Transform, &mut Visibility, &mut Mesh2d), (With<InnerBubble>, Without<Ship>)>,
                  mut outer_bubble: Single<(&mut Transform, &mut Visibility, &mut Mesh2d), (With<OuterBubble>, Without<Ship>, Without<InnerBubble>)>,
                  mut meshes: ResMut<Assets<Mesh>>,
                  mut ui_state: ResMut<UiState>,
                  physics: Res<PhysicsManager>) {
    if !timer.just_finished() {
        return;
    }

    for bubble_translation in [&mut inner_bubble.0.translation, &mut outer_bubble.0.translation] {
        *bubble_translation = ship_transform.translation.xy().extend(-5.);
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
                     particles: Query<(Entity, &mut Transform, &mut SpaceDustPhysics), (With<SpaceDust>, Without<Ship>)>,
                     ship_transform: Single<&Transform, With<Ship>>) {
    if !timer.just_finished() {
        return;
    }

    for (entity_id, mut dust_pos, mut dust_state) in particles {
        physics.step_particle(&mut dust_state.0);

        dust_pos.translation = physics_to_game(dust_state.0);
        if dust_pos.translation.x < ship_transform.translation.x - DUST_CULL_DRAG {
            commands.entity(entity_id).despawn();
        }
    }
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
#[require(Sprite, Transform, Visibility)]
struct InnerBubble;

#[derive(Component)]
#[require(Sprite, Transform, Visibility)]
struct OuterBubble;

fn make_bubble_mesh(mut meshes: &mut ResMut<Assets<Mesh>>, physics_radius: f64) -> Mesh2d {
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
#[require(Sprite, Transform)]
struct SpaceDust;

#[derive(Component, Deref, DerefMut)]
struct SpaceDustPhysics(ParticleState<f64>);

#[derive(Component, Deref, DerefMut)]
struct ShipPhysics(ParticleState<f64>);

impl From<ParticleState<f64>> for SpaceDustPhysics {
    fn from(value: ParticleState<f64>) -> Self {
        Self(value)
    }
}

fn game_to_physics(particle_pos: Vec3,
                   ship_physics: &ShipPhysics,
                   physics_manager: Res<PhysicsManager>) -> SpaceDustPhysics {
    physics_manager.new_particle_state(
        particle_pos.x as f64 / PHYSICS_SCALING_FACTOR,
        particle_pos.y as f64 / PHYSICS_SCALING_FACTOR,
        particle_pos.z.into(),
        ship_physics.t()
    ).into()
}

fn physics_to_game(particle_pos: ParticleState<f64>) -> Vec3 {
    Vec3::new(
        (PHYSICS_SCALING_FACTOR * particle_pos.x()) as f32,
        (PHYSICS_SCALING_FACTOR * particle_pos.y()) as f32,
        particle_pos.z() as f32
    )
}

#[derive(Resource, Deref)]
struct SpaceDustMesh(Handle<Mesh>);

#[derive(Resource, Deref)]
struct SpaceDustMaterial(Handle<ColorMaterial>);

#[derive(Bundle)]
struct SpaceDustEntity(Mesh2d, MeshMaterial2d<ColorMaterial>, Transform, SpaceDust, SpaceDustPhysics);

impl SpaceDust {
    fn new_entity(starting_position: Vec3,
                  ship_physics: &ShipPhysics,
                  mesh: Res<SpaceDustMesh>,
                  mat: Res<SpaceDustMaterial>,
                  physics_manager: Res<PhysicsManager>) -> SpaceDustEntity {
        SpaceDustEntity(
            Mesh2d(mesh.clone()),
            MeshMaterial2d(mat.clone()),
            Transform::from_translation(starting_position).with_scale(Vec2::splat(DUST_DIAMETER).extend(1.)),
            SpaceDust,
            game_to_physics(starting_position, ship_physics, physics_manager)
        )
    }
}
