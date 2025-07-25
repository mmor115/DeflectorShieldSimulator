use crate::game_systems::ui::{ParticleSettings, ShutdownState, ValidatorSettings, VisualSettings};
use crate::physics::physics_parameters::PhysicsParameters;
use deflector_core::wd_ours::WarpDriveOurs;
use serde::{Deserialize, Serialize};

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
    pub deflector_sigma_pushout: f64
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
            warp_drive: WarpDriveOurs {
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
        }
    }
}

impl From<&PhysicsParameters> for PhysicsConfig {
    fn from(value: &PhysicsParameters) -> Self {
        PhysicsConfig {
            radius: value.warp_drive.radius,
            sigma: value.warp_drive.sigma,
            u: value.warp_drive.u,
            u0: value.warp_drive.u0,
            k0: value.warp_drive.k0,
            x0: value.warp_drive.x0,
            t0: value.warp_drive.t0,
            gamma: value.warp_drive.gamma,
            epsilon: value.warp_drive.epsilon,
            deflector_back: value.warp_drive.deflector_back,
            deflector_sigma_factor: value.warp_drive.deflector_sigma_factor,
            deflector_sigma_pushout: value.warp_drive.deflector_sigma_pushout
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