use bevy::prelude::Resource;
use deflector_core::wd_ours::WarpDriveOurs;

#[derive(Resource, Debug)]
pub struct PhysicsParameters {
    pub ship_speed: f64,
    pub warp_drive: WarpDriveOurs
}

impl PhysicsParameters {
    pub fn warp_drive(&self) -> &WarpDriveOurs {
        &self.warp_drive
    }
    
    pub fn ship_speed(&self) -> f64 {
        self.ship_speed
    }

    pub fn bubble_radius(&self) -> f64 {
        self.warp_drive.radius
    }

    pub fn bubble_sigma(&self) -> f64 {
        self.warp_drive.sigma
    }

    pub fn u(&self) -> f64 {
        self.warp_drive.u
    }

    pub fn u0(&self) -> f64 {
        self.warp_drive.u0
    }

    pub fn k0(&self) -> f64 {
        self.warp_drive.k0
    }

    pub fn ts(&self) -> f64 {
        self.warp_drive.ts
    }

    pub fn ds(&self) -> f64 {
        self.warp_drive.ds
    }

    pub fn epsilon(&self) -> f64 {
        self.warp_drive.epsilon
    }

    pub fn set_ship_speed(&mut self, value: f64) {
        self.ship_speed = value;
    }

    pub fn set_bubble_radius(&mut self, value: f64) {
        self.warp_drive.radius = value;
    }

    pub fn set_bubble_sigma(&mut self, value: f64) {
        self.warp_drive.sigma = value;
    }

    pub fn set_u(&mut self, value: f64) {
        self.warp_drive.u = value;
    }

    pub fn set_u0(&mut self, value: f64) {
        self.warp_drive.u0 = value;
    }

    pub fn set_k0(&mut self, value: f64) {
        self.warp_drive.k0 = value;
    }

    pub fn set_ts(&mut self, value: f64) {
        self.warp_drive.ts = value;
    }

    pub fn set_ds(&mut self, value: f64) {
        self.warp_drive.ds = value;
    }

    pub fn set_epsilon(&mut self, value: f64) {
        self.warp_drive.epsilon = value;
    }
}

impl Default for PhysicsParameters {
    fn default() -> Self {
        Self {
            ship_speed: 0.0,
            warp_drive: WarpDriveOurs {
                radius: 4.,
                sigma: 4.,
                u: 0.5,
                u0: 0.5,
                k0: 0.1,
                ts: f64::MAX,
                ds: 10.,
                epsilon: 1e-12,
            }
        }
    }
}
