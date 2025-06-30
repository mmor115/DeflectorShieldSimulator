mod physics;

use bevy::prelude::*;
use deflector_core::types::{ParticleState, ParticleStateComponents};
use deflector_core::wd_ours::WarpDriveOurs;
use rand::Rng;
use crate::physics::PhysicsManager;

const SHIP_POS: Vec3 = Vec3::new(0., 0., 0.);
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

const PHYSICS_SHIP_SPEED: f64 = 0.0;
const PHYSICS_BUBBLE_RADIUS: f64 = 4.;
const PHYSICS_BUBBLE_SIGMA: f64 = 4.;
const PHYSICS_U: f64 = 0.5;
const PHYSICS_U0: f64 = 0.5;
const PHYSICS_K0: f64 = 0.1;
const PHYSICS_TS: f64 = f64::MAX;
const PHYSICS_DS: f64 = 10.;
const PHYSICS_EPSILON: f64 = 1e-12;

const CAMERA_ZOOM: f32 = 2.5;

const TICK_RATE: f32 = 60.;
const TICK_INTERVAL: f32 = 1. / TICK_RATE;

const SHOW_INNER_BUBBLE: bool = true;
const SHOW_OUTER_BUBBLE: bool = false;


fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .insert_resource(SpaceDustSpawnTimer(Timer::from_seconds(TICK_INTERVAL, TimerMode::Repeating)))
        .insert_resource(SpaceDustUpdateTimer(Timer::from_seconds(TICK_INTERVAL, TimerMode::Repeating)))
        .insert_resource(ShipUpdateTimer(Timer::from_seconds(TICK_INTERVAL, TimerMode::Repeating)))
        .add_systems(Update, pan_camera)
        .add_systems(FixedUpdate, (update_ship_and_bubbles, spawn_space_dust, update_space_dust).chain())
        .run();
}

fn setup(mut commands: Commands, 
         mut meshes: ResMut<Assets<Mesh>>,
         mut materials: ResMut<Assets<ColorMaterial>>,
         asset_server: Res<AssetServer>) {
    commands.spawn((
        Camera2d,
        Transform::from_translation(SHIP_POS).with_scale(Vec3::splat(1. / CAMERA_ZOOM))
    ));

    let ship_image = asset_server.load::<Image>("images/ship.png");

    let space_dust_mesh = meshes.add(Circle::default());
    commands.insert_resource(SpaceDustMesh(space_dust_mesh));
    
    let space_dust_material = materials.add(DUST_COLOR);
    commands.insert_resource(SpaceDustMaterial(space_dust_material));

    let physics_manager = PhysicsManager::new(
        WarpDriveOurs {
            radius: PHYSICS_BUBBLE_RADIUS,
            sigma: PHYSICS_BUBBLE_SIGMA,
            u: PHYSICS_U,
            u0: PHYSICS_U0,
            k0: PHYSICS_K0,
            ts: PHYSICS_TS,
            ds: PHYSICS_DS,
            epsilon: PHYSICS_EPSILON
        },
        PHYSICS_STEP_SIZE
    );

    commands.spawn((
        Sprite::from_image(ship_image),
        Transform::from_translation(SHIP_POS).with_scale(SHIP_SCALE),
        Ship,
        ShipPhysics(physics_manager.new_ship_particle_state(PHYSICS_SHIP_SPEED))
    ));

    if SHOW_INNER_BUBBLE {
        let inner_bubble_radius = (PHYSICS_BUBBLE_RADIUS * PHYSICS_SCALING_FACTOR) as f32;
        let inner_bubble = Annulus::new(inner_bubble_radius - 2.0, inner_bubble_radius);

        commands.spawn((
            Bubble,
            Mesh2d(meshes.add(inner_bubble)),
            MeshMaterial2d(materials.add(INNER_BUBBLE_COLOR)),
            Transform::from_translation(SHIP_POS)
        ));
    }

    if SHOW_OUTER_BUBBLE {
        let outer_bubble_radius = ((PHYSICS_BUBBLE_RADIUS + PHYSICS_BUBBLE_SIGMA) * PHYSICS_SCALING_FACTOR) as f32;
        let outer_bubble = Annulus::new(outer_bubble_radius - 2.0, outer_bubble_radius);

        commands.spawn((
            Bubble,
            Mesh2d(meshes.add(outer_bubble)),
            MeshMaterial2d(materials.add(OUTER_BUBBLE_COLOR)),
            Transform::from_translation(SHIP_POS)
        ));
    }

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

fn update_ship_and_bubbles(time: Res<Time>,
                           mut timer: ResMut<ShipUpdateTimer>,
                           physics: Res<PhysicsManager>,
                           ship: Single<(&mut Transform, &mut ShipPhysics), With<Ship>>,
                           bubbles: Query<&mut Transform, (With<Bubble>, Without<Ship>)>) {
    if !timer.tick(time.delta()).just_finished() {
        return;
    }

    let (mut ship_transform, mut ship_state) = ship.into_inner();

    physics.step_particle(&mut ship_state.0);
    ship_transform.translation = physics_to_game(ship_state.0);
    ship_transform.translation.z = -10.;

    for mut bubble_transform in bubbles {
        bubble_transform.translation = ship_transform.translation;
        bubble_transform.translation.z = -5.;
    }
}

fn update_space_dust(time: Res<Time>,
                     mut timer: ResMut<SpaceDustUpdateTimer>,
                     mut commands: Commands,
                     physics: Res<PhysicsManager>,
                     particles: Query<(Entity, &mut Transform, &mut SpaceDustPhysics), (With<SpaceDust>, Without<Ship>)>,
                     ship_transform: Single<&Transform, With<Ship>>) {
    if !timer.tick(time.delta()).just_finished() {
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

#[derive(Resource, Deref, DerefMut)]
struct SpaceDustUpdateTimer(Timer);

#[derive(Resource, Deref, DerefMut)]
struct ShipUpdateTimer(Timer);

#[derive(Component)]
#[require(Sprite, Transform)]
struct Ship;

#[derive(Component)]
#[require(Sprite, Transform)]
struct Bubble;

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
