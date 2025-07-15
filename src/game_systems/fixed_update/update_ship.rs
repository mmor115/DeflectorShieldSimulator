use crate::game_entities::ship::{Ship, ShipPhysics};
use crate::game_systems::timers::PhysicsUpdateTimer;
use crate::physics::physics_manager::PhysicsManager;
use bevy::prelude::{Res, ResMut, Single, Transform, With};

pub fn update_ship(timer: ResMut<PhysicsUpdateTimer>,
                   physics: Res<PhysicsManager>,
                   ship: Single<(&mut Transform, &mut ShipPhysics), With<Ship>>) {
    if !timer.just_finished() {
        return;
    }

    let (mut ship_transform, mut ship_state) = ship.into_inner();

    physics.step_particle(&mut ship_state.0);
    ship_transform.translation = crate::physics_to_game(ship_state.0);
}