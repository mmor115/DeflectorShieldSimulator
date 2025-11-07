use crate::game_entities::ship::ShipPhysics;
use bevy::prelude::Resource;
use deflector_core::warp_drive::WarpDrive;
use deflector_core::wd_natario::WarpDriveNatario;
use deflector_core::wd_ours::WarpDriveOurs;

#[derive(Resource, Clone, Debug)]
pub enum WarpDriveImpl {
    Ours(WarpDriveOurs),
    Natario(WarpDriveNatario)
}

impl WarpDriveImpl {
    pub fn get_dynamic(&self) -> &dyn WarpDrive {
        match self {
            WarpDriveImpl::Ours(w) => w,
            WarpDriveImpl::Natario(w) => w,
        }
    }
    
    pub fn get_mut_dynamic(&mut self) -> &mut dyn WarpDrive {
        match self {
            WarpDriveImpl::Ours(w) => w,
            WarpDriveImpl::Natario(w) => w,
        }
    }

    fn set_u0_pure(&mut self, value: f64) {
        match self {
            WarpDriveImpl::Ours(w) => {
                w.u0 = value;
            }
            _ => {}
        }
    }
}

#[derive(Resource, Clone, Debug)]
pub struct PhysicsParameters {
    pub warp_drive: WarpDriveImpl
}

impl PhysicsParameters {
    pub fn warp_drive(&self) -> &WarpDriveImpl {
        &self.warp_drive
    }

    pub fn bubble_radius(&self) -> f64 {
        match &self.warp_drive {
            WarpDriveImpl::Ours(wd) => wd.get_radius(),
            WarpDriveImpl::Natario(wd) => wd.get_radius()
        }
    }

    pub fn bubble_sigma(&self) -> f64 {
        match &self.warp_drive {
            WarpDriveImpl::Ours(wd) => wd.get_sigma(),
            WarpDriveImpl::Natario(wd) => wd.get_sigma()
        }
    }

    pub fn u(&self) -> f64 {
        match &self.warp_drive {
            WarpDriveImpl::Ours(wd) => wd.get_u(),
            WarpDriveImpl::Natario(wd) => wd.get_u()
        }
    }

    pub fn u0(&self) -> Option<f64> {
        match &self.warp_drive {
            WarpDriveImpl::Ours(wd) => Some(wd.get_u0()),
            WarpDriveImpl::Natario(_) => None
        }
    }

    pub fn k0(&self) -> Option<f64> {
        match &self.warp_drive {
            WarpDriveImpl::Ours(wd) => Some(wd.get_k0()),
            WarpDriveImpl::Natario(_) => None
        }
    }

    pub fn deflector_sigma_pushout(&self) -> Option<f64> {
        match &self.warp_drive {
            WarpDriveImpl::Ours(wd) => Some(wd.get_deflector_sigma_pushout()),
            WarpDriveImpl::Natario(_) => None
        }
    }

    pub fn deflector_sigma_factor(&self) -> Option<f64> {
        match &self.warp_drive {
            WarpDriveImpl::Ours(wd) => Some(wd.get_deflector_sigma_factor()),
            WarpDriveImpl::Natario(_) => None
        }
    }

    pub fn deflector_back(&self) -> Option<f64> {
        match &self.warp_drive {
            WarpDriveImpl::Ours(wd) => Some(wd.get_deflector_back()),
            WarpDriveImpl::Natario(_) => None
        }
    }
    
    pub fn set_bubble_radius(&mut self, value: f64) {
        match &mut self.warp_drive {
            WarpDriveImpl::Ours(wd) => wd.update_radius(value),
            WarpDriveImpl::Natario(wd) => wd.update_radius(value)
        }
    }

    pub fn set_bubble_sigma(&mut self, value: f64) {
        match &mut self.warp_drive {
            WarpDriveImpl::Ours(wd) => wd.update_sigma(value),
            WarpDriveImpl::Natario(wd) => wd.update_sigma(value)
        }
    }

    pub fn set_u(&mut self, value: f64, t: f64) {
        match &mut self.warp_drive {
            WarpDriveImpl::Ours(wd) => wd.update_u(t, value),
            WarpDriveImpl::Natario(wd) => wd.update_u(t, value)
        }
    }

    pub fn set_u0(&mut self, value: f64, ship_state: &mut ShipPhysics) {
        match &mut self.warp_drive {
            WarpDriveImpl::Ours(wd) => wd.update_u0(value, ship_state).expect("update_u0 failed"),
            WarpDriveImpl::Natario(_) => {  }
        }
    }

    pub fn set_u0_pure(&mut self, value: f64) {
        self.warp_drive.set_u0_pure(value)
    }

    pub fn set_k0(&mut self, value: f64) {
        match &mut self.warp_drive {
            WarpDriveImpl::Ours(wd) => wd.update_k0(value),
            WarpDriveImpl::Natario(_) => {  }
        }
    }

    pub fn set_deflector_sigma_pushout(&mut self, value: f64) {
        match &mut self.warp_drive {
            WarpDriveImpl::Ours(wd) => wd.update_deflector_sigma_pushout(value),
            WarpDriveImpl::Natario(_) => {  }
        }
    }

    pub fn set_deflector_sigma_factor(&mut self, value: f64) {
        match &mut self.warp_drive {
            WarpDriveImpl::Ours(wd) => wd.update_deflector_sigma_factor(value),
            WarpDriveImpl::Natario(_) => {  }
        }
    }

    pub fn set_deflector_back(&mut self, value: f64) {
        match &mut self.warp_drive {
            WarpDriveImpl::Ours(wd) => wd.update_deflector_back(value),
            WarpDriveImpl::Natario(_) => {  }
        }
    }
    
    pub fn shut_down(&mut self, global_time: f64) {
        self.warp_drive.get_mut_dynamic().shut_down(global_time);
    }

    pub fn shut_up(&mut self, global_time: f64, ship_state: &mut ShipPhysics, restart_parameters: &PhysicsParameters) {
        match &mut self.warp_drive {
            WarpDriveImpl::Ours(wd) => {
                match &restart_parameters.warp_drive {
                    WarpDriveImpl::Ours(rp) => {
                        wd.shut_up(global_time, ship_state, rp)
                    },
                    _ => panic!("shut_up(): restart_parameters.warp_drive is not WarpDriveImpl::Ours")
                }
            },
            WarpDriveImpl::Natario(wd) => {
                match &restart_parameters.warp_drive {
                    WarpDriveImpl::Natario(rp) => {
                        wd.shut_up(global_time, ship_state, rp)
                    },
                    _ => panic!("shut_up(): restart_parameters.warp_drive is not WarpDriveImpl::Natario")
                }
            }
        }.expect("shut_up() failed");
    }
}

impl Default for PhysicsParameters {
    fn default() -> Self {
        Self {
            warp_drive: WarpDriveImpl::Ours(
                WarpDriveOurs::new(
                    4.,
                    4.,
                    0.5,
                    0.5,
                    0.1,
                    0.8,
                    1.0,
                    1.0
                )
            )
        }
    }
}
