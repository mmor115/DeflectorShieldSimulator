// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 Max Morris, Lucas Timotheo Sanches and Steven Robert Brandt

use crate::game_entities::bubbles;
use crate::game_entities::bubbles::{InnerBubble, OuterBubble};
use crate::game_entities::ship::Ship;
use crate::game_systems::timers::PhysicsUpdateTimer;
use crate::game_systems::ui::{PauseControls, ShutdownState, UiState, VisualSettings};
use crate::physics::physics_manager::PhysicsManager;
use crate::PHYSICS_SCALING_FACTOR;
use bevy::asset::Assets;
use bevy::prelude::{Mesh, Mesh2d, Res, ResMut, Single, Transform, Visibility, With, Without};

pub fn update_bubbles(timer: Res<PhysicsUpdateTimer>,
                      visual_settings: Res<VisualSettings>,
                      mut inner_bubble: Single<(&mut Transform, &mut Visibility, &mut Mesh2d), (With<InnerBubble>, Without<Ship>)>,
                      mut outer_bubble: Single<(&mut Transform, &mut Visibility, &mut Mesh2d), (With<OuterBubble>, Without<Ship>, Without<InnerBubble>)>,
                      mut meshes: ResMut<Assets<Mesh>>,
                      mut ui_state: ResMut<UiState>,
                      shutdown_state: Res<ShutdownState>,
                      physics: Res<PhysicsManager>,
                      pause_controls: Res<PauseControls>) {
    if pause_controls.paused {
        return;
    }

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
        *inner_bubble.2 = bubbles::make_inner_bubble_mesh(&mut meshes, &physics.physics_parameters);
        ui_state.need_remesh_inner_bubble = false;
    }

    if ui_state.need_remesh_outer_bubble {
        meshes.remove(outer_bubble.2.id()).unwrap();
        *outer_bubble.2 = bubbles::make_outer_bubble_mesh(&mut meshes, &physics.physics_parameters);
        ui_state.need_remesh_outer_bubble = false;
    }
}