// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 Max Morris, Lucas Timotheo Sanches and Steven Robert Brandt

use bevy::prelude::*;

pub struct CameraControlPlugin;

#[derive(Resource, Debug)]
pub struct CameraControls {
    pub translation: Vec3,
    pub zoom: f32,
    pub panning: bool
}

impl CameraControls {
    pub fn new(translation: Vec3, zoom: f32) -> Self {
        Self {
            translation,
            zoom,
            panning: false
        }
    }
}

impl Default for CameraControls {
    fn default() -> Self {
        Self::new(Vec3::splat(0.), 1.)
    }
}

impl Plugin for CameraControlPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(CameraControls::default());
    }
}