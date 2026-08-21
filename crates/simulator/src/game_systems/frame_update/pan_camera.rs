// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 Max Morris, Lucas Timotheo Sanches, and Steven R. Brandt

use crate::game_entities::ship::Ship;
use crate::game_systems::camera_control::CameraControls;
use crate::game_systems::setup::CAMERA_ZOOM;
use bevy::input::mouse::{MouseButtonInput, MouseMotion, MouseScrollUnit, MouseWheel};
use bevy::input::ButtonState;
use bevy::prelude::*;

pub fn pan_camera(mut camera2d: Single<&mut Transform, (With<Camera2d>, Without<Ship>)>,
                  ship: Single<&Transform, With<Ship>>,
                  mut scroll_events: EventReader<MouseWheel>,
                  mut mouse_motion_events: EventReader<MouseMotion>,
                  mut mouse_button_events: EventReader<MouseButtonInput>,
                  mut camera_controls: ResMut<CameraControls>) {
    for event in scroll_events.read() {
        match event.unit {
            MouseScrollUnit::Line => {
                camera_controls.zoom = (camera_controls.zoom + (0.05 * event.y)).clamp(1., 10.);
            }
            MouseScrollUnit::Pixel => {
                camera_controls.zoom = (camera_controls.zoom + (0.05 * event.y)).clamp(1., 10.);
            }
        }
    }

    for event in mouse_button_events.read() {
        match event.button {
            MouseButton::Right => {
                camera_controls.panning = event.state == ButtonState::Pressed;
            }
            MouseButton::Middle => {
                camera_controls.translation = Vec3::ZERO;
                camera_controls.zoom = 1.;
            }
            _ => {}
        }
    }

    camera2d.scale = Vec3::splat(1. / (camera_controls.zoom * CAMERA_ZOOM));

    for event in mouse_motion_events.read() {
        if camera_controls.panning {
            camera_controls.translation.x -= (camera2d.scale.x) * event.delta.x;
            camera_controls.translation.y += (camera2d.scale.y) * event.delta.y;
        }
    }

    camera2d.translation = ship.translation;
    camera2d.translation += camera_controls.translation;
}