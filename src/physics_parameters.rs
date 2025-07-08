use bevy::prelude::Resource;
use deflector_core::warp_drive::WarpDrive;
use deflector_core::wd_ours::WarpDriveOurs;
use crate::ShipPhysics;

#[derive(Resource, Clone, Debug)]
pub struct PhysicsParameters {
    pub warp_drive: WarpDriveOurs
}

impl PhysicsParameters {
    pub fn warp_drive(&self) -> &WarpDriveOurs {
        &self.warp_drive
    }

    pub fn ship_speed(&self) -> f64 {
        self.warp_drive.compute_ship_speed().unwrap()
    }

    pub fn bubble_radius(&self) -> f64 {
        self.warp_drive.get_radius()
    }

    pub fn bubble_sigma(&self) -> f64 {
        self.warp_drive.get_sigma()
    }

    pub fn u(&self) -> f64 {
        self.warp_drive.get_u()
    }

    pub fn u0(&self) -> f64 {
        self.warp_drive.get_u0()
    }

    pub fn k0(&self) -> f64 {
        self.warp_drive.get_k0()
    }
    
    pub fn set_bubble_radius(&mut self, value: f64) {
        self.warp_drive.update_radius(value);
    }

    pub fn set_bubble_sigma(&mut self, value: f64) {
        self.warp_drive.update_sigma(value);
    }

    pub fn set_u(&mut self, value: f64, t: f64) {
        self.warp_drive.update_u(t, value);
    }

    pub fn set_u0(&mut self, value: f64, ship_state: &mut ShipPhysics) {
        *ship_state = self.warp_drive.update_u0(value, &ship_state).unwrap().into();
    }

    pub fn set_k0(&mut self, value: f64) {
        self.warp_drive.update_k0(value);
    }
    
    pub fn shut_down_now(&mut self) {
        self.warp_drive.shut_down_now();
    }
}

impl Default for PhysicsParameters {
    fn default() -> Self {
        Self {
            warp_drive: WarpDriveOurs::new(
                4., 
                4., 
                0.5, 
                0.5, 
                0.1
            )
        }
    }
}
