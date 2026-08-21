// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 Max Morris, Lucas Timotheo Sanches and Steven Robert Brandt

use crate::game_systems::timers::{PhysicsUpdateTimer, SpaceDustSpawnTimer, BASE_TICK_RATE};
use crate::game_systems::ui::SpeedControls;
use bevy::prelude::{Res, ResMut, Time};

pub fn post_update_physics(time: Res<Time>,
                           mut physics_update_timer: ResMut<PhysicsUpdateTimer>,
                           mut space_dust_spawn_timer: ResMut<SpaceDustSpawnTimer>,
                           mut speed_controls: ResMut<SpeedControls>) {
    let tick_rate = speed_controls.tick_rate_factor * BASE_TICK_RATE;
    let tick_interval = 1. / tick_rate;

    speed_controls.keep_up_warning = time.delta().as_secs_f32() > tick_interval;

    if !speed_controls.need_apply {
        return;
    }

    *physics_update_timer = physics_update_timer.with_new_interval(tick_interval);
    *space_dust_spawn_timer = space_dust_spawn_timer.with_new_interval(tick_interval);

    speed_controls.need_apply = false;
}