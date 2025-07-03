use bevy::math::Vec3;
use crate::physics_parameters::PhysicsParameters;
use bevy::prelude::Resource;
use deflector_core::evolve::rk4_step;
use deflector_core::types::ParticleState;
use deflector_core::types::ParticleType::Massive;
use deflector_core::warp_drive::WarpDrive;
use crate::PHYSICS_SCALING_FACTOR;

#[derive(Resource)]
pub struct PhysicsManager {
    pub physics_parameters: PhysicsParameters,
    step_size: f64,
    global_time: f64,
    bubble_pos: Vec3
}

impl PhysicsManager {
    pub fn new(physics_parameters: PhysicsParameters, step_size: f64) -> PhysicsManager {
        PhysicsManager {
            physics_parameters,
            step_size,
            global_time: 0.,
            bubble_pos: Vec3::default()
        }
    }
    
    pub fn step_size(&self) -> f64 {
        self.step_size
    }

    pub fn global_time(&self) -> f64 {
        self.global_time
    }
    
    pub fn bubble_pos(&self) -> Vec3 {
        self.bubble_pos
    }

    pub fn incr_global_time(&mut self) {
        self.global_time += self.step_size;
        self.bubble_pos.x += 
            (PHYSICS_SCALING_FACTOR * self.step_size * self.physics_parameters.warp_drive.get_bubble_speed()) as f32;
    }

    pub fn new_particle_state(&self,
                              initial_x: f64,
                              initial_y: f64,
                              initial_z: f64) -> ParticleState<f64> {
        self.physics_parameters.warp_drive().make_normalized_state(
            initial_x,
            initial_y,
            initial_z,
            0.,
            0.,
            0.,
            &Massive
        ).expect("warp_drive.make_normalized_state() failed")
    }
    
    pub fn new_ship_particle_state(&self, ship_speed: f64) -> ParticleState<f64> {
        self.physics_parameters.warp_drive().make_normalized_state(
            0.,
            0.,
            0.,
            ship_speed,
            0.,
            0.,
            &Massive
        ).expect("warp_drive.make_normalized_state() failed")
    }
    
    pub fn step_particle(&self, p: &mut ParticleState<f64>) {
        rk4_step(self.global_time, self.step_size, self.physics_parameters.warp_drive(), p);
    }
}