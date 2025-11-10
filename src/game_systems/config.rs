use deflector_core::wd_natario::WarpDriveNatario;
use crate::game_systems::ui::{ParticleSettings, ShutdownState, ValidatorSettings, VisualSettings};
use crate::physics::physics_parameters::{PhysicsParameters, WarpDriveImpl};
use deflector_core::wd_ours::WarpDriveOurs;
use derive_more::Display;
use enum_ordinalize::Ordinalize;
use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Clone, Ordinalize, Display, Eq, PartialEq)]
#[repr(usize)]
pub enum WarpDriveKind {
    Ours,
    Natario
}

impl From<&WarpDriveImpl> for WarpDriveKind {
    fn from(value: &WarpDriveImpl) -> Self {
        match value {
            WarpDriveImpl::Ours(_) => WarpDriveKind::Ours,
            WarpDriveImpl::Natario(_) => WarpDriveKind::Natario
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct PhysicsConfig {
    pub radius: f64,
    pub sigma: f64,
    pub u: f64,
    pub u0: f64,
    pub k0: f64,
    pub x0: f64,
    pub t0: f64,
    pub gamma: f64,
    pub epsilon: f64,
    pub deflector_back: f64,
    pub deflector_sigma_factor: f64,
    pub deflector_sigma_pushout: f64,
    pub warp_drive_kind: WarpDriveKind
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ShutdownConfig {
    pub in_shutdown_state: bool,
    pub temporary_parameters: Option<PhysicsConfig>
}

#[derive(Serialize, Deserialize, Clone)]
pub struct GlobalConfig {
    pub physics_config: PhysicsConfig,
    pub particle_settings: ParticleSettings,
    pub visual_settings: VisualSettings,
    pub shutdown_config: ShutdownConfig,
    #[serde(default)]
    pub validator_settings: ValidatorSettings
}

impl From<&PhysicsConfig> for PhysicsParameters {
    fn from(value: &PhysicsConfig) -> Self {
        PhysicsParameters {
            warp_drive: match value.warp_drive_kind {
                WarpDriveKind::Ours => WarpDriveImpl::Ours(
                    WarpDriveOurs {
                        radius: value.radius,
                        sigma: value.sigma,
                        u: value.u,
                        u0: value.u0,
                        k0: value.k0,
                        x0: value.x0,
                        t0: value.t0,
                        gamma: value.gamma,
                        epsilon: value.epsilon,
                        deflector_sigma_pushout: value.deflector_sigma_pushout,
                        deflector_sigma_factor: value.deflector_sigma_factor,
                        deflector_back: value.deflector_back
                    }
                ),
                WarpDriveKind::Natario => WarpDriveImpl::Natario(
                    WarpDriveNatario {
                        radius: value.radius,
                        sigma: value.sigma,
                        u: value.u,
                        x0: value.x0,
                        t0: value.t0,
                        epsilon: value.epsilon,
                    }
                )
            }
        }
    }
}

impl From<&PhysicsParameters> for PhysicsConfig {
    fn from(value: &PhysicsParameters) -> Self {
        match &value.warp_drive {
            WarpDriveImpl::Ours(wd) => {
                PhysicsConfig {
                    radius: wd.radius,
                    sigma: wd.sigma,
                    u: wd.u,
                    u0: wd.u0,
                    k0: wd.k0,
                    x0: wd.x0,
                    t0: wd.t0,
                    gamma: wd.gamma,
                    epsilon: wd.epsilon,
                    deflector_back: wd.deflector_back,
                    deflector_sigma_factor: wd.deflector_sigma_factor,
                    deflector_sigma_pushout: wd.deflector_sigma_pushout,
                    warp_drive_kind: (&value.warp_drive).into()
                }
            }
            WarpDriveImpl::Natario(wd) => {
                PhysicsConfig {
                    radius: wd.radius,
                    sigma: wd.sigma,
                    u: wd.u,
                    u0: Default::default(),
                    k0: Default::default(),
                    x0: wd.x0,
                    t0: wd.t0,
                    gamma: Default::default(),
                    epsilon: wd.epsilon,
                    deflector_back: Default::default(),
                    deflector_sigma_factor: Default::default(),
                    deflector_sigma_pushout: Default::default(),
                    warp_drive_kind: (&value.warp_drive).into()
                }
            }
        }
    }
}

impl From<&ShutdownConfig> for ShutdownState {
    fn from(value: &ShutdownConfig) -> Self {
        ShutdownState {
            in_shutdown_state: value.in_shutdown_state,
            temporary_parameters: value.temporary_parameters.as_ref().map(|p| p.into())
        }
    }
}

impl From<&ShutdownState> for ShutdownConfig {
    fn from(value: &ShutdownState) -> Self {
        ShutdownConfig {
            in_shutdown_state: value.in_shutdown_state,
            temporary_parameters: value.temporary_parameters.as_ref().map(|p| p.into()),
        }
    }
}