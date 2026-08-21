// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 Max Morris, Lucas Timotheo Sanches, and Steven R. Brandt

use bevy::asset::Handle;
use bevy::prelude::*;
use deflector_core::types::ParticleState;
use derive_more::From;
use serde::{Deserialize, Serialize};

#[derive(Component)]
#[require(Sprite, Transform)]
pub struct Ship;

#[derive(Component, Deref, DerefMut, Debug, Serialize, Deserialize, From, Clone)]
pub struct ShipPhysics(pub ParticleState<f64>);

#[derive(Resource, Deref)]
pub struct ShipImageAsset(Handle<Image>);

impl ShipImageAsset {
    pub fn new(img: Handle<Image>) -> Self {
        ShipImageAsset(img)
    }
}

#[derive(Bundle)]
pub struct ShipEntity(Sprite, Transform, Ship, ShipPhysics);

impl ShipEntity {
    pub fn new(ship_image_asset: Res<ShipImageAsset>, state: ShipPhysics) -> Self {
        Self::new_from_image_handle(ship_image_asset.clone(), state)
    }

    pub fn new_from_image_handle(ship_image: Handle<Image>, state: ShipPhysics) -> Self {
        let translation = crate::physics_to_game(*state);
        ShipEntity(
            Sprite::from_image(ship_image),
            Transform::from_translation(translation).with_scale(SHIP_SCALE),
            Ship,
            state
        )
    }
}

const SHIP_SCALE: Vec3 = Vec3::new(0.05, 0.05, 1.);