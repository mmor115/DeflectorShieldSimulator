// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 Max Morris, Lucas Timotheo Sanches and Steven Robert Brandt

use crate::physics::physics_parameters::PhysicsParameters;
use crate::PHYSICS_SCALING_FACTOR;
use bevy::asset::Assets;
use bevy::prelude::*;

#[derive(Component)]
#[require(Transform, Visibility)]
pub struct InnerBubble;

#[derive(Component)]
#[require(Transform, Visibility)]
pub struct OuterBubble;

fn make_bubble_mesh(meshes: &mut ResMut<Assets<Mesh>>, physics_radius: f64) -> Mesh2d {
    let bubble_radius = (physics_radius * PHYSICS_SCALING_FACTOR) as f32;
    let annulus = Annulus::new(bubble_radius - 2.0, bubble_radius);
    Mesh2d(meshes.add(annulus))
}

pub fn make_inner_bubble_mesh(meshes: &mut ResMut<Assets<Mesh>>,
                              physics_parameters: &PhysicsParameters) -> Mesh2d {
    make_bubble_mesh(meshes, physics_parameters.warp_drive.bubble_radius())
}

pub fn make_outer_bubble_mesh(meshes: &mut ResMut<Assets<Mesh>>,
                              physics_parameters: &PhysicsParameters) -> Mesh2d {
    make_bubble_mesh(meshes, physics_parameters.warp_drive.bubble_radius() + physics_parameters.warp_drive.bubble_sigma())
}