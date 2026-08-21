// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 Max Morris, Lucas Timotheo Sanches, and Steven R. Brandt

use bevy::prelude::*;
use crate::game_entities::explosion::*;

pub fn update_explosions(time: Res<Time>,
                         mut explosions: Query<(
                             Entity,
                             &mut Transform,
                             &mut ExplosionSecsLived,
                             &MeshMaterial2d<ColorMaterial>
                         )>,
                         mut commands: Commands,
                         mut materials: ResMut<Assets<ColorMaterial>>) {
    for (
        entity, 
        mut tr,
        mut secs_lived,
        mat
    ) in explosions.iter_mut() {
        secs_lived.0 += time.delta_secs();

        if secs_lived.0 > EXPLOSION_LIFETIME_SECS {
            commands.entity(entity).despawn();
        } else {
            let time_ratio = secs_lived.0 / EXPLOSION_LIFETIME_SECS;

            let scale = EXPLOSION_START_RADIUS + (EXPLOSION_END_RADIUS - EXPLOSION_START_RADIUS) * time_ratio;
            tr.scale = Vec3::splat(scale);

            let alpha = 1. - time_ratio;
            materials.get_mut(mat.0.id()).unwrap().color.set_alpha(alpha);
        }
    }
}