// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 Max Morris, Lucas Timotheo Sanches, and Steven R. Brandt

use crate::game_entities::ship::ShipPhysics;
use crate::game_systems::config::WarpDriveKind;
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

    pub fn bubble_radius(&self) -> f64 {
        match &self {
            WarpDriveImpl::Ours(wd) => wd.get_radius(),
            WarpDriveImpl::Natario(wd) => wd.get_radius()
        }
    }

    pub fn bubble_sigma(&self) -> f64 {
        match &self {
            WarpDriveImpl::Ours(wd) => wd.get_sigma(),
            WarpDriveImpl::Natario(wd) => wd.get_sigma()
        }
    }

    pub fn u(&self) -> f64 {
        match &self {
            WarpDriveImpl::Ours(wd) => wd.get_u(),
            WarpDriveImpl::Natario(wd) => wd.get_u()
        }
    }

    pub fn u0(&self) -> Option<f64> {
        match &self {
            WarpDriveImpl::Ours(wd) => Some(wd.get_u0()),
            WarpDriveImpl::Natario(_) => None
        }
    }

    pub fn k0(&self) -> Option<f64> {
        match &self {
            WarpDriveImpl::Ours(wd) => Some(wd.get_k0()),
            WarpDriveImpl::Natario(_) => None
        }
    }

    pub fn t0(&self) -> f64 {
        match &self {
            WarpDriveImpl::Ours(wd) => wd.t0,
            WarpDriveImpl::Natario(wd) => wd.t0
        }
    }

    pub fn x0(&self) -> f64 {
        match &self {
            WarpDriveImpl::Ours(wd) => wd.x0,
            WarpDriveImpl::Natario(wd) => wd.x0
        }
    }

    /* The softening added to r^2 so that the metric functions stay finite at the bubble
       center. It must not be zero: shut_up() and resume() place the bubble exactly on the
       ship, and the ship then sits at r = 0, where the derivatives of r divide by it. */
    pub fn epsilon(&self) -> f64 {
        match &self {
            WarpDriveImpl::Ours(wd) => wd.epsilon,
            WarpDriveImpl::Natario(wd) => wd.epsilon
        }
    }

    pub fn deflector_sigma_pushout(&self) -> Option<f64> {
        match &self {
            WarpDriveImpl::Ours(wd) => Some(wd.get_deflector_sigma_pushout()),
            WarpDriveImpl::Natario(_) => None
        }
    }

    pub fn deflector_sigma_factor(&self) -> Option<f64> {
        match &self {
            WarpDriveImpl::Ours(wd) => Some(wd.get_deflector_sigma_factor()),
            WarpDriveImpl::Natario(_) => None
        }
    }

    pub fn deflector_back(&self) -> Option<f64> {
        match &self {
            WarpDriveImpl::Ours(wd) => Some(wd.get_deflector_back()),
            WarpDriveImpl::Natario(_) => None
        }
    }

    pub fn set_bubble_radius(&mut self, value: f64) {
        match self {
            WarpDriveImpl::Ours(wd) => wd.update_radius(value),
            WarpDriveImpl::Natario(wd) => wd.update_radius(value)
        }
    }

    pub fn set_bubble_sigma(&mut self, value: f64) {
        match self {
            WarpDriveImpl::Ours(wd) => wd.update_sigma(value),
            WarpDriveImpl::Natario(wd) => wd.update_sigma(value)
        }
    }

    pub fn set_u(&mut self, value: f64, t: f64) {
        match self {
            WarpDriveImpl::Ours(wd) => wd.update_u(t, value),
            WarpDriveImpl::Natario(wd) => wd.update_u(t, value)
        }
    }

    pub fn set_u0(&mut self, value: f64, ship_state: &mut ShipPhysics) {
        match self {
            WarpDriveImpl::Ours(wd) => wd.update_u0(value, ship_state).expect("update_u0 failed"),
            WarpDriveImpl::Natario(_) => {  }
        }
    }

    pub fn set_k0(&mut self, value: f64) {
        match self {
            WarpDriveImpl::Ours(wd) => wd.update_k0(value),
            WarpDriveImpl::Natario(_) => {  }
        }
    }

    pub fn set_deflector_sigma_pushout(&mut self, value: f64) {
        match self {
            WarpDriveImpl::Ours(wd) => wd.update_deflector_sigma_pushout(value),
            WarpDriveImpl::Natario(_) => {  }
        }
    }

    pub fn set_deflector_sigma_factor(&mut self, value: f64) {
        match self {
            WarpDriveImpl::Ours(wd) => wd.update_deflector_sigma_factor(value),
            WarpDriveImpl::Natario(_) => {  }
        }
    }

    pub fn set_deflector_back(&mut self, value: f64) {
        match self {
            WarpDriveImpl::Ours(wd) => wd.update_deflector_back(value),
            WarpDriveImpl::Natario(_) => {  }
        }
    }

    pub fn new_from(&self, kind: WarpDriveKind, global_time: f64, ship_state: &mut ShipPhysics) -> WarpDriveImpl {
        match kind {
            WarpDriveKind::Ours => WarpDriveImpl::Ours(WarpDriveOurs::resume(
                global_time,
                self.bubble_radius(),
                self.bubble_sigma(),
                self.u(),
                self.u0().unwrap_or_else(|| self.u()),
                self.k0().unwrap_or(0.1),
                self.x0(),
                self.t0(),
                self.deflector_sigma_pushout().unwrap_or(0.8),
                self.deflector_sigma_factor().unwrap_or(1.0),
                self.deflector_back().unwrap_or(1.0),
                ship_state
            )),
            WarpDriveKind::Natario => WarpDriveImpl::Natario(WarpDriveNatario {
                radius: self.bubble_radius(),
                sigma: self.bubble_sigma(),
                u: self.u(),
                x0: self.x0(),
                t0: self.t0(),
                epsilon: self.epsilon()
            })
        }
    }

    pub(crate) fn set_u0_pure(&mut self, value: f64) {
        /* Natario has no drag term, so there is nothing to set. */
        if let WarpDriveImpl::Ours(w) = self {
            w.u0 = value;
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game_systems::config::WarpDriveKind;
    use deflector_core::types::ParticleState;

    fn ship_at_origin() -> ShipPhysics {
        ShipPhysics(ParticleState::from_column_slice(&[0., 0., 0., 0., 0., 0., 1.]))
    }

    /* shut_up() and resume() both place the bubble exactly on the ship, which puts the
       ship at r = 0. The metric functions divide by r there, so the epsilon softening
       has to survive a change of drive; switching used to reset it to zero, and the
       shift vector went NaN on the first step after Shut up. */
    #[test]
    fn shift_vector_stays_finite_after_a_shutdown_cycle_on_natario() {
        let mut ship = ship_at_origin();
        let mut parameters = PhysicsParameters::default();

        parameters.warp_drive = parameters.warp_drive
                                          .new_from(WarpDriveKind::Natario, 0.0, &mut ship);

        assert!(parameters.warp_drive.epsilon() > 0.0,
                "changing drive dropped the epsilon softening");

        let stashed = parameters.clone();
        parameters.shut_down(1.0);
        parameters.shut_up(2.0, &mut ship, &stashed);

        let warp_drive = parameters.warp_drive.get_dynamic();
        let q = nalgebra::Vector4::new(2.0, ship[0], ship[1], ship[2]);

        assert!(warp_drive.vx(&q).is_finite(), "vx went NaN at the ship");
        assert!(warp_drive.vy(&q).is_finite(), "vy went NaN at the ship");
        assert!(warp_drive.vz(&q).is_finite(), "vz went NaN at the ship");
    }

    /* Every drive carries the softening, so the same has to hold going the other way. */
    #[test]
    fn changing_drive_preserves_epsilon_in_both_directions() {
        let mut ship = ship_at_origin();
        let mut parameters = PhysicsParameters::default();
        let original = parameters.warp_drive.epsilon();

        assert!(original > 0.0);

        parameters.warp_drive = parameters.warp_drive
                                          .new_from(WarpDriveKind::Natario, 0.0, &mut ship);
        assert_eq!(parameters.warp_drive.epsilon(), original);

        parameters.warp_drive = parameters.warp_drive
                                          .new_from(WarpDriveKind::Ours, 0.0, &mut ship);
        assert_eq!(parameters.warp_drive.epsilon(), original);
    }
}
