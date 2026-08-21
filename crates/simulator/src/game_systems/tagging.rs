// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 Max Morris, Lucas Timotheo Sanches, and Steven R. Brandt

use std::collections::HashSet;
use bevy::app::{App, Plugin};
use bevy::prelude::Resource;
use serde::{Deserialize, Serialize};
use crate::game_entities::space_dust::SpaceDustId;

pub struct TaggingPlugin;

impl Plugin for TaggingPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(TaggedParticles::new());
    }
}

#[derive(Resource, Serialize, Deserialize, Clone, Debug)]
pub struct TaggedParticles {
    tagged_particles: HashSet<SpaceDustId>
}

impl TaggedParticles {
    pub fn new() -> Self {
        TaggedParticles {
            tagged_particles: HashSet::new()
        }
    }

    pub fn toggle(&mut self, id: SpaceDustId) -> bool {
        if self.tagged_particles.contains(&id) {
            self.tagged_particles.remove(&id);
            false
        } else {
            self.tagged_particles.insert(id);
            true
        }
    }

    pub fn is_tagged(&self, id: &SpaceDustId) -> bool {
        self.tagged_particles.contains(id)
    }
}

impl Default for TaggedParticles {
    fn default() -> Self {
        Self::new()
    }
}