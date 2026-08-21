// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 Max Morris, Lucas Timotheo Sanches, and Steven R. Brandt

use crate::game_systems::timers::PhysicsUpdateTimer;
use crate::physics::physics_manager::PhysicsManager;
use bevy::prelude::{Res, ResMut, Time};
use crate::game_systems::ui::PauseControls;

pub fn pre_update_physics(time: Res<Time>,
                          mut timer: ResMut<PhysicsUpdateTimer>,
                          mut physics: ResMut<PhysicsManager>,
                          pause_controls: Res<PauseControls>) {
    if pause_controls.paused {
        return;
    }

    if !timer.tick(time.delta()).just_finished() {
        return;
    }

    physics.incr_global_time();
}