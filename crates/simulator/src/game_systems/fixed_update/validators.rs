use crate::game_entities::explosion::{ExplosionEntity, ExplosionMesh};
use crate::game_entities::ship::ShipPhysics;
use crate::game_entities::space_dust::{ParticleTypeComponent, SpaceDustId, SpaceDustPhysics};
use crate::game_systems::ui::{ValidatorErrBehavior, ValidatorSettings};
use bevy::prelude::*;
use deflector_core::debug::is_normalized;
use deflector_core::types::ParticleType;

/* If a particular validator is set to kill the program upon tripping, we want to run it
   after the snapshot is taken so that the bad state(s) are captured in the dump (once that's
   implemented, of course!) However, if the validator is set to remove the offending particle(s),
   then we want to run it BEFORE the snapshot because we don't want the running history to contain
   invalid states. */

pub fn nan_validator_pre_snapshot_predicate(settings: Res<ValidatorSettings>) -> bool {
    matches!(settings.nan_validator,
             ValidatorErrBehavior::RemoveParticle | ValidatorErrBehavior::ExplodeParticle)
}

pub fn normalization_validator_pre_snapshot_predicate(settings: Res<ValidatorSettings>) -> bool {
    matches!(settings.normalization_validator,
             ValidatorErrBehavior::RemoveParticle | ValidatorErrBehavior::ExplodeParticle)
}

pub fn nan_validator_post_snapshot_predicate(settings: Res<ValidatorSettings>) -> bool {
    matches!(settings.nan_validator, ValidatorErrBehavior::Die)
}

pub fn normalization_validator_post_snapshot_predicate(settings: Res<ValidatorSettings>) -> bool {
    matches!(settings.normalization_validator, ValidatorErrBehavior::Die)
}

pub fn nan_validator_pre_snapshot(particles: Query<(&SpaceDustPhysics, &Transform, Entity)>,
                                  ship: Single<&ShipPhysics>,
                                  mut commands: Commands,
                                  settings: Res<ValidatorSettings>,
                                  explosion_mesh: Res<ExplosionMesh>,
                                  mut materials: ResMut<Assets<ColorMaterial>>) {
    for n in &ship.0 {
        if n.is_nan() {
            panic!("[Validator] Found a NaN in the ship state vector: {}", ship.0);
        }
    }

    for (state, transform, entity) in particles {
        for n in &state.0 {
            if n.is_nan() {
                commands.entity(entity).despawn();
                match settings.nan_validator {
                    ValidatorErrBehavior::RemoveParticle => { }
                    ValidatorErrBehavior::ExplodeParticle => {
                        commands.spawn(ExplosionEntity::new(
                            &explosion_mesh,
                            &mut materials,
                            transform.translation
                        ));
                    }
                    _ => unreachable!()
                }
            }
        }
    }
}

pub fn normalization_validator_pre_snapshot(particles: Query<(&SpaceDustPhysics, &ParticleTypeComponent, &Transform, Entity)>,
                                            ship: Single<&ShipPhysics>,
                                            settings: Res<ValidatorSettings>,
                                            explosion_mesh: Res<ExplosionMesh>,
                                            mut commands: Commands,
                                            mut materials: ResMut<Assets<ColorMaterial>>) {
    if !is_normalized(&ship.0, &ParticleType::Massive, settings.normalized_tolerance) {
        panic!("[Validator] Ship state vector is not normalized within tolerance {}: {}", settings.normalized_tolerance, ship.0);
    }

    for (state, p_type, transform, entity) in particles {
        if !is_normalized(&state.0, &p_type.0, settings.normalized_tolerance) {
            commands.entity(entity).despawn();
            match settings.normalization_validator {
                ValidatorErrBehavior::RemoveParticle => { }
                ValidatorErrBehavior::ExplodeParticle => {
                    commands.spawn(ExplosionEntity::new(
                        &explosion_mesh,
                        &mut materials,
                        transform.translation
                    ));
                }
                _ => unreachable!()
            }
        }
    }
}

pub fn nan_validator_post_snapshot(particles: Query<(&SpaceDustPhysics, &SpaceDustId)>,
                                   ship: Single<&ShipPhysics>) {
    for n in &ship.0 {
        if n.is_nan() {
            panic!("[Validator] Found a NaN in the ship state vector: {}", ship.0);
        }
    }

    for (state, id) in particles {
        for n in &state.0 {
            if n.is_nan() {
                panic!("[Validator] Found a NaN in the particle {} state vector: {}", id, state.0);
            }
        }
    }
}

pub fn normalization_validator_post_snapshot(particles: Query<(&SpaceDustPhysics, &SpaceDustId, &ParticleTypeComponent)>,
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