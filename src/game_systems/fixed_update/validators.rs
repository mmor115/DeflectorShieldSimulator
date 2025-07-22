use crate::game_entities::ship::ShipPhysics;
use crate::game_entities::space_dust::{ParticleTypeComponent, SpaceDustId, SpaceDustPhysics};
use crate::game_systems::ui::ValidatorSettings;
use bevy::prelude::*;
use deflector_core::debug::is_normalized;
use deflector_core::types::ParticleType;

pub fn nan_validator_predicate(settings: Res<ValidatorSettings>) -> bool {
    settings.check_nan
}

pub fn normalization_validator_predicate(settings: Res<ValidatorSettings>) -> bool {
    settings.check_normalized
}

pub fn nan_validator(particles: Query<(&SpaceDustPhysics, &SpaceDustId)>,
                     ship: Single<&ShipPhysics>) {
    for n in &ship.0 {
        if n.is_nan() {
            panic!("[Validator] Found a NaN in the ship state vector: {}", ship.0);
        }
    }

    for (state, id) in particles {
        for n in &state.0 {
            if n.is_nan() {
                panic!("[Validator] Found a NaN in the particle {} state vector: {}", id, ship.0);
            }
        }
    }
}

pub fn normalization_validator(particles: Query<(&SpaceDustPhysics, &SpaceDustId, &ParticleTypeComponent)>,
                               ship: Single<&ShipPhysics>,
                               settings: Res<ValidatorSettings>) {
    if !is_normalized(&ship.0, &ParticleType::Massive, settings.normalized_tolerance) {
        panic!("[Validator] Ship state vector is not normalized within tolerance {}: {}", settings.normalized_tolerance, ship.0);
    }

    for (state, id, p_type) in particles {
        if !is_normalized(&state.0, &p_type.0, settings.normalized_tolerance) {
            panic!("[Validator] Particle {} state vector is not normalized within tolerance {}: {}", id, settings.normalized_tolerance, state.0);
        }
    }
}