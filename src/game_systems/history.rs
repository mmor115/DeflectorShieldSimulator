use crate::game_entities::ship::{Ship, ShipPhysics};
use crate::game_entities::space_dust::{SpaceDust, SpaceDustId, SpaceDustPhysics};
use crate::game_systems::config::GlobalConfig;
use crate::game_systems::seeded_rng::SeededRng;
use crate::game_systems::tagging::TaggedParticles;
use crate::game_systems::timers::PhysicsUpdateTimer;
use crate::game_systems::ui::{ParticleSettings, PauseControls, ShutdownState, VisualSettings};
use crate::physics::physics_manager::PhysicsManager;
use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use crate::util::bull::Bull;

pub struct HistoryPlugin;

impl Plugin for HistoryPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SimulationHistory::new());
    }
}

#[derive(Resource, Serialize, Deserialize, Clone)]
pub struct SimulationHistory {
    pub snapshots: Vec<GlobalSnapshot>,
    #[serde(default)]
    pub tagged_particles: TaggedParticles
}

impl SimulationHistory {
    pub fn new() -> Self {
        SimulationHistory {
            snapshots: Vec::new(),
            tagged_particles: TaggedParticles::new()
        }
    }

    pub fn as_borrowed(&self) -> BorrowedSimulationHistory {
        self.into()
    }

    pub fn checkpoint(&self) -> BorrowedSimulationHistory {
        let snapshots = if self.snapshots.is_empty() {
            vec![]
        } else {
            vec![(&self.snapshots[self.snapshots.len() - 1]).into()]
        };

        BorrowedSimulationHistory {
            snapshots: Bull::InnerBorrowed(snapshots),
            tagged_particles: &self.tagged_particles
        }
    }

    pub fn trim(&mut self) {
        if let Some(snapshot) = self.snapshots.pop() {
            self.snapshots = vec![snapshot];
        }
    }

    pub fn truncated(&self, last_idx: usize) -> Self {
        SimulationHistory {
            snapshots: self.snapshots[..=last_idx].to_owned(),
            tagged_particles: self.tagged_particles.clone()
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct GlobalSnapshot {
    pub global_config: GlobalConfig,
    pub global_time: f64,
    pub seeded_rng: SeededRng,
    pub ship_state: ShipStateSnapshot,
    pub particle_states: Vec<SpaceDustStateSnapshot>
}

#[derive(Serialize, Deserialize, Clone)]
pub struct SpaceDustStateSnapshot {
    pub physics: SpaceDustPhysics,
    pub id: SpaceDustId
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ShipStateSnapshot {
    pub physics: ShipPhysics
}

pub fn take_snapshot(timer: Res<PhysicsUpdateTimer>,
                     mut history: ResMut<SimulationHistory>,
                     physics: Res<PhysicsManager>,
                     particles: Query<(&SpaceDustPhysics, &SpaceDustId), (With<SpaceDust>, Without<Ship>)>,
                     ship: Single<&ShipPhysics, With<Ship>>,
                     visual_settings: Res<VisualSettings>,
                     particle_settings: Res<ParticleSettings>,
                     shutdown_state: Res<ShutdownState>,
                     seeded_rng: Res<SeededRng>,
                     pause_controls: Res<PauseControls>) {
    if pause_controls.paused {
        return;
    }

    if !timer.just_finished() {
        return;
    }

    let global_config = GlobalConfig {
        physics_config: (&physics.physics_parameters).into(),
        particle_settings: particle_settings.clone(),
        visual_settings: visual_settings.clone(),
        shutdown_config: shutdown_state.as_ref().into()
    };

    let global_time = physics.global_time();

    let ship_state = ShipStateSnapshot {
        physics: ship.clone()
    };

    let particle_states = particles.iter().map(|p| {
        SpaceDustStateSnapshot {
            physics: p.0.clone(),
            id: p.1.clone(),
        }
    }).collect::<Vec<_>>();

    let seeded_rng = seeded_rng.clone();

    history.snapshots.push(GlobalSnapshot {
        global_config,
        global_time,
        seeded_rng,
        ship_state,
        particle_states
    });
}

#[derive(Serialize)]
pub struct BorrowedSimulationHistory<'a> {
    pub snapshots: Bull<'a, GlobalSnapshot, BorrowedGlobalSnapshot<'a>>,
    #[serde(default)]
    pub tagged_particles: &'a TaggedParticles
}

impl <'a> From<&'a SimulationHistory> for BorrowedSimulationHistory<'a> {
    fn from(value: &'a SimulationHistory) -> Self {
        Self {
            snapshots: Bull::OuterBorrowed(&value.snapshots),
            tagged_particles: &value.tagged_particles,
        }
    }
}

impl <'a> BorrowedSimulationHistory<'a> {
    pub fn tagged_only(&self) -> Self {
        let tagged_particles = self.tagged_particles;

        BorrowedSimulationHistory {
            snapshots: Bull::InnerBorrowed(
                self.snapshots.as_owned_vec().iter().map(|s| {
                    let mut particle_states = s.particle_states.as_owned_vec();
                    particle_states.retain(|p| tagged_particles.is_tagged(&p.id));

                    BorrowedGlobalSnapshot {
                        global_config: s.global_config,
                        global_time: s.global_time,
                        seeded_rng: s.seeded_rng,
                        ship_state: s.ship_state,
                        particle_states: Bull::InnerBorrowed(particle_states)
                    }
                }).collect()
            ),
            tagged_particles
        }
    }
}

#[derive(Serialize, Clone)]
pub struct BorrowedGlobalSnapshot<'a> {
    pub global_config: &'a GlobalConfig,
    pub global_time: f64,
    pub seeded_rng: &'a SeededRng,
    pub ship_state: &'a ShipStateSnapshot,
    pub particle_states: Bull<'a, SpaceDustStateSnapshot, &'a SpaceDustStateSnapshot>
}

impl <'a> From<&'a GlobalSnapshot> for BorrowedGlobalSnapshot<'a> {
    fn from(value: &'a GlobalSnapshot) -> Self {
        Self {
            global_config: &value.global_config,
            global_time: value.global_time,
            seeded_rng: &value.seeded_rng,
            ship_state: &value.ship_state,
            particle_states: Bull::OuterBorrowed(&value.particle_states),
        }
    }
}