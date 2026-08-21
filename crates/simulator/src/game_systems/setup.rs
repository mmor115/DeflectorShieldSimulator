// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 Max Morris, Lucas Timotheo Sanches and Steven Robert Brandt

use crate::game_entities::bubbles::{InnerBubble, OuterBubble};
use crate::game_entities::ship::{ShipEntity, ShipImageAsset, ShipPhysics};
use crate::game_entities::space_dust::{SpaceDustColorMaterials, SpaceDustMesh, TaggedSpaceDustMaterialAsset};
use crate::physics::physics_manager::PhysicsManager;
use crate::physics::physics_parameters::PhysicsParameters;
use crate::{game_entities, PHYSICS_STEP_SIZE};
use bevy::asset::{AssetServer, Assets};
use bevy::math::Vec3;
use bevy::prelude::*;

const INNER_BUBBLE_COLOR: Color = Color::srgb(0.0, 0.5, 1.0);
const OUTER_BUBBLE_COLOR: Color = Color::srgb(0.0, 0.0, 1.0);

const TAGGED_PARTICLE_COLOR: Color = Color::srgb(255., 0., 255.);

const INITIAL_SHIP_POS: Vec3 = Vec3::new(0., 0., 0.);

pub const CAMERA_ZOOM: f32 = 2.5;

pub fn setup(mut commands: Commands,
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
    commands.insert_resource(TaggedSpaceDustMaterialAsset::new(materials.add(TAGGED_PARTICLE_COLOR)));

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
        game_entities::bubbles::make_inner_bubble_mesh(&mut meshes, params),
        MeshMaterial2d(materials.add(INNER_BUBBLE_COLOR)),
        Transform::from_translation(INITIAL_SHIP_POS)
    ));

    commands.spawn((
        OuterBubble,
        game_entities::bubbles::make_outer_bubble_mesh(&mut meshes, params),
        MeshMaterial2d(materials.add(OUTER_BUBBLE_COLOR)),
        Transform::from_translation(INITIAL_SHIP_POS)
    ));

    commands.insert_resource(physics_manager);
}
