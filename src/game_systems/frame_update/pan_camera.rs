use crate::game_entities::ship::Ship;
use bevy::prelude::{Camera2d, Single, Transform, With, Without};

pub fn pan_camera(mut camera2d: Single<&mut Transform, (With<Camera2d>, Without<Ship>)>,
                  ship: Single<&Transform, With<Ship>>) {
    camera2d.translation = ship.translation;
}