// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 Max Morris, Lucas Timotheo Sanches, and Steven R. Brandt

use bevy::prelude::*;
use rand::Rng;
use rand_xoshiro::rand_core::{RngCore, SeedableRng};
use rand_xoshiro::Xoshiro256PlusPlus;
use serde::{Deserialize, Serialize};

pub struct SeededRngPlugin;

impl Plugin for SeededRngPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SeededRng::default());
    }
}

pub type RngSeed = [u8; 32];

#[derive(Resource, Serialize, Deserialize, Clone)]
pub struct SeededRng {
    rng: Xoshiro256PlusPlus,
    seed: RngSeed
}

impl SeededRng {
    pub fn new(seed: RngSeed) -> Self {
        SeededRng {
            rng: Xoshiro256PlusPlus::from_seed(seed),
            seed
        }
    }
}

impl Default for SeededRng {
    fn default() -> Self {
        let mut seed_rng = rand::rng();
        let seed: RngSeed = seed_rng.random();
        SeededRng::new(seed)
    }
}

impl RngCore for SeededRng {
    fn next_u32(&mut self) -> u32 {
        self.rng.next_u32()
    }

    fn next_u64(&mut self) -> u64 {
        self.rng.next_u64()
    }

    fn fill_bytes(&mut self, dst: &mut [u8]) {
        self.rng.fill_bytes(dst)
    }
}