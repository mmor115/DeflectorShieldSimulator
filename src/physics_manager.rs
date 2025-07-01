use crate::physics_parameters::PhysicsParameters;
use bevy::prelude::Resource;
use deflector_core::evolve::rk4_step;
use deflector_core::types::ParticleState;
use deflector_core::types::ParticleType::Massive;
use deflector_core::warp_drive::WarpDrive;

#[derive(Resource)]
pub struct PhysicsManager {
    pub physics_parameters: PhysicsParameters,
    step_size: f64  // 0.1
}

impl PhysicsManager {
    pub fn new(physics_parameters: PhysicsParameters, step_size: f64) -> PhysicsManager {
        PhysicsManager {
            physics_parameters,
            step_size
        }
    }
    
    pub fn new_particle_state(&self, 
                              initial_x: f64, 
                              initial_y: f64, 
                              initial_z: f64, 
                              initial_t: f64) -> ParticleState<f64> {
        self.physics_parameters.warp_drive().make_normalized_state(
            initial_t,
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
            0.,
            ship_speed,
            0.,
            0.,
            &Massive
        ).expect("warp_drive.make_normalized_state() failed")
    }
    
    pub fn step_particle(&self, p: &mut ParticleState<f64>) {
        rk4_step(self.step_size, self.physics_parameters.warp_drive(), p);
    }
}