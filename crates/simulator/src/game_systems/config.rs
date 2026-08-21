use crate::game_systems::ui::{ParticleSettings, ShutdownState, ValidatorSettings, VisualSettings};
use crate::physics::physics_parameters::{PhysicsParameters, WarpDriveImpl};
use deflector_core::wd_natario::WarpDriveNatario;
use deflector_core::wd_ours::WarpDriveOurs;
use derive_more::Display;
use enum_ordinalize::Ordinalize;
use serde::{Deserialize, Serialize};

/* Which warp-drive metric the simulation evolves. `Ours` is the CCT deflector-shield
   drive of the paper; `Natario` is the zero-expansion drive, which has no deflector and
   therefore ignores the u0, k0 and deflector_* parameters. */
#[derive(Serialize, Deserialize, Clone, Debug, Ordinalize, Display, Eq, PartialEq, Default)]
#[repr(usize)]
pub enum WarpDriveKind {
    #[default]
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
    /* These three arrived after the first configs were written. They carry defaults so
       that a file saved by an older build still loads. The values match
       PhysicsParameters::default(); a plain #[serde(default)] would silently substitute
       0.0 and change the physics. */
    #[serde(default = "default_deflector_back")]
    pub deflector_back: f64,
    #[serde(default = "default_deflector_sigma_factor")]
    pub deflector_sigma_factor: f64,
    #[serde(default = "default_deflector_sigma_pushout")]
    pub deflector_sigma_pushout: f64,
    /* Added with the Natario drive. Anything written before that names no drive, and
       every such file described the CCT drive, which is what the default resolves to. */
    #[serde(default)]
    pub warp_drive_kind: WarpDriveKind
}

fn default_deflector_back() -> f64 {
    1.0
}

fn default_deflector_sigma_factor() -> f64 {
    1.0
}

fn default_deflector_sigma_pushout() -> f64 {
    0.8
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
                        epsilon: value.epsilon
                    }
                )
            }
        }
    }
}

impl From<&PhysicsParameters> for PhysicsConfig {
    fn from(value: &PhysicsParameters) -> Self {
        /* The Natario drive carries no deflector, so the fields it does not own are
           written out as zeros. Reloading such a file rebuilds a Natario drive and
           ignores them again. */
        match &value.warp_drive {
            WarpDriveImpl::Ours(wd) => PhysicsConfig {
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
                warp_drive_kind: WarpDriveKind::Ours
            },
            WarpDriveImpl::Natario(wd) => PhysicsConfig {
                radius: wd.radius,
                sigma: wd.sigma,
                u: wd.u,
                u0: 0.,
                k0: 0.,
                x0: wd.x0,
                t0: wd.t0,
                gamma: 0.,
                epsilon: wd.epsilon,
                deflector_back: 0.,
                deflector_sigma_factor: 0.,
                deflector_sigma_pushout: 0.,
                warp_drive_kind: WarpDriveKind::Natario
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
#[cfg(test)]
mod tests {
    use super::{GlobalConfig, WarpDriveKind};
    use std::path::{Path, PathBuf};

    /* configs/ sits at the repository root, which is the crate root before the workspace
       migration and two levels up after it. Try both so this test survives the move. */
    fn configs_dir() -> PathBuf {
        let manifest = Path::new(env!("CARGO_MANIFEST_DIR"));

        for candidate in [manifest.join("configs"), manifest.join("../../configs")] {
            if candidate.is_dir() {
                return candidate;
            }
        }

        panic!("could not locate the configs directory");
    }

    /* The deflector_* fields were added after the shipped configs were written, and for a
       while these files failed to parse. Keep them loadable. */
    #[test]
    fn shipped_configs_deserialize() {
        let dir = configs_dir();
        let mut checked = 0;

        for entry in std::fs::read_dir(&dir).expect("read configs dir") {
            let path = entry.expect("read dir entry").path();

            if path.extension().and_then(|e| e.to_str()) != Some("json") {
                continue;
            }

            let text = std::fs::read_to_string(&path).expect("read config file");

            serde_json::from_str::<GlobalConfig>(&text)
                .unwrap_or_else(|e| panic!("{} failed to deserialize: {e}", path.display()));

            checked += 1;
        }

        assert!(checked > 0, "no configs found in {}", dir.display());
    }

    /* A config written by a build that predates the deflector parameters and the drive
       selector must still load, and must pick up the documented defaults rather than
       zeros. */
    #[test]
    fn pre_deflector_config_gets_documented_defaults() {
        let legacy = r#"{
            "physics_config": {
                "radius": 4.0, "sigma": 4.0, "u": 0.5, "u0": 0.5, "k0": 0.1,
                "x0": 0.0, "t0": 0.0, "gamma": 0.0, "epsilon": 1e-12
            },
            "particle_settings": {
                "y_position_variance": 150.0, "z_position_variance": 0.0,
                "x_velocity_variance": 0.0, "y_velocity_variance": 0.0,
                "z_velocity_variance": 0.0, "photon_chance": 0.0
            },
            "visual_settings": { "show_inner_bubble": true, "show_outer_bubble": true },
            "shutdown_config": { "in_shutdown_state": false, "temporary_parameters": null }
        }"#;

        let config: GlobalConfig = serde_json::from_str(legacy).expect("legacy config loads");

        assert_eq!(config.physics_config.deflector_back, 1.0);
        assert_eq!(config.physics_config.deflector_sigma_factor, 1.0);
        assert_eq!(config.physics_config.deflector_sigma_pushout, 0.8);
        assert_eq!(config.physics_config.warp_drive_kind, WarpDriveKind::Ours);
        assert!(config.particle_settings.spawning_enabled);
    }
}
