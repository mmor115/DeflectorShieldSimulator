// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 Max Morris, Lucas Timotheo Sanches, and Steven R. Brandt

use crate::physics::physics_parameters::PhysicsParameters;
use bevy::prelude::Resource;
use deflector_core::types::{ParticleState, ParticleType};

#[derive(Resource)]
pub struct PhysicsManager {
    pub physics_parameters: PhysicsParameters,
    step_size: f64,
    global_time: f64
}

impl PhysicsManager {
    pub fn new(physics_parameters: PhysicsParameters, step_size: f64) -> PhysicsManager {
        PhysicsManager {
            physics_parameters,
            step_size,
            global_time: 0.
        }
    }
    
    pub fn _step_size(&self) -> f64 {
        self.step_size
    }

    pub fn global_time(&self) -> f64 {
        self.global_time
    }

    pub fn incr_global_time(&mut self) {
        self.global_time += self.step_size;
    }

    pub fn set_global_time(&mut self, global_time: f64) {
        self.global_time = global_time;
    }

    pub fn bubble_x_position(&self) -> f64{
        self.physics_parameters.warp_drive.get_dynamic().get_bubble_position(self.global_time)
    }

    pub fn new_particle_state(&self,
                              initial_x: f64,
                              initial_y: f64,
                              initial_z: f64,
                              initial_vx: f64, 
                              initial_vy: f64, 
                              initial_vz: f64,
                              particle_type: ParticleType) -> ParticleState<f64> {
        self.physics_parameters.warp_drive().get_dynamic().make_normalized_state(
            initial_x,
            initial_y,
            initial_z,
            initial_vx,
            initial_vy,
            initial_vz,
            &particle_type
        ).expect("warp_drive.make_normalized_state() failed")
    }
    
    pub fn new_ship_particle_state(&self) -> ParticleState<f64> {
        self.physics_parameters.warp_drive.get_dynamic()
                                          .make_ship_state(0.)
                                          .expect("warp_drive.make_ship_state() failed")
    }
    
    pub fn step_particle(&self, p: &mut ParticleState<f64>, particle_type: &ParticleType) {
        self.physics_parameters.warp_drive().get_dynamic().rk4_step_adaptive(
            self.global_time,
            self.step_size,
            p,
            particle_type
        );
    }
}