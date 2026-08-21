// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 Max Morris, Lucas Timotheo Sanches and Steven Robert Brandt

use crate::game_entities::ship::{Ship, ShipEntity, ShipImageAsset, ShipPhysics};
use crate::game_entities::space_dust::{SpaceDust, SpaceDustColorMaterials, SpaceDustEntity, SpaceDustMesh, TaggedSpaceDustMaterialAsset};
use crate::game_systems::config::{GlobalConfig, WarpDriveKind};
use crate::game_systems::fixed_update::space_dust_click_observer::space_dust_click_observer;
use crate::game_systems::history::SimulationHistory;
use crate::game_systems::tagging::TaggedParticles;
use crate::physics::physics_manager::PhysicsManager;
use crate::physics::physics_parameters::PhysicsParameters;
use crate::physics_to_game;
use bevy::app::{App, PostUpdate};
use bevy::math::Vec3Swizzles;
use bevy::prelude::*;
use bevy_mod_imgui::ImguiContext;
use imgui::{SliderFlags, StyleColor};
use serde::{Deserialize, Serialize};
use std::fs;
use std::fs::OpenOptions;
use std::io::{BufReader, BufWriter, Write};
use std::path::Path;
use enum_ordinalize::Ordinalize;

/// Shown in the About tab. Kept here so the strings appear once.
const SOURCE_URL: &str = "https://github.com/max-morris/DeflectorShieldSimulator";
const MANUAL_URL: &str = "https://max-morris.github.io/DeflectorShieldSimulator/manual/";
const PAPER_CITATION: &str = concat!(
    "L. T. Sanches, M. Morris and S. R. Brandt, \"Exploring Particle Geodesics in a ",
    "Warp Drive Spacetime\", Classical and Quantum Gravity (accepted); ",
    "arXiv:2608.08213 [gr-qc]."
);

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(VisualSettings::default())
            .insert_resource(ParticleSettings::default())
            .insert_resource(UiState::default())
            .insert_resource(ShutdownState::default())
            .insert_resource(SaveLoadState::default())
            .insert_resource(SpeedControls::default())
            .insert_resource(PauseControls::default())
            .insert_resource(ValidatorSettings::default())
            .add_systems(PostUpdate, ui);
    }
}

#[derive(Resource, Serialize, Deserialize, Clone)]
pub struct VisualSettings {
    pub show_inner_bubble: bool,
    pub show_outer_bubble: bool,
    #[serde(default)]
    pub hide_untagged_particles: bool
}

// Thanks, serde!
fn return_true() -> bool {
    true
}

#[derive(Resource, Serialize, Deserialize, Clone)]
pub struct ParticleSettings {
    #[serde(default = "return_true")]
    pub spawning_enabled: bool,
    pub y_position_variance: f32, // game units
    pub z_position_variance: f32, // game units
    pub x_velocity_variance: f64, // physics units
    pub y_velocity_variance: f64, // physics units
    pub z_velocity_variance: f64, // physics units
    pub photon_chance: f64 // [0, 1]
}

#[derive(Resource)]
pub struct UiState {
    pub need_remesh_inner_bubble: bool,
    pub need_remesh_outer_bubble: bool,
    config_path_buf: String,
    history_path_buf: String,
    history_format: HistoryFormat,
    err_text: String,
    resume_idx_buf: usize,
    dump_only_tagged_particles: bool
}

#[derive(Resource, Default)]
pub struct ShutdownState {
    pub in_shutdown_state: bool,
    pub temporary_parameters: Option<PhysicsParameters>
}

#[derive(Resource, Default)]
pub struct SaveLoadState {
    loaded_history: Option<SimulationHistory>
}

#[derive(Resource, Serialize, Deserialize, Clone)]
pub struct SpeedControls {
    pub tick_rate_factor: f32,
    pub need_apply: bool,
    pub keep_up_warning: bool
}

#[derive(Resource, Serialize, Deserialize, Clone, Default)]
pub struct PauseControls {
    pub paused: bool
}

#[derive(Resource, Serialize, Deserialize, Clone)]
pub struct ValidatorSettings {
    pub nan_validator: ValidatorErrBehavior,
    pub normalization_validator: ValidatorErrBehavior,
    pub normalized_tolerance: f64,
    normalized_tolerance_power: i32
}

#[derive(PartialEq, Copy, Clone, Serialize, Deserialize)]
pub enum ValidatorErrBehavior {
    Ignore,
    RemoveParticle,
    ExplodeParticle,
    Die
}

#[derive(PartialEq, Copy, Clone)]
enum HistoryFormat {
    Json,
    Bin
}

impl ShutdownState {
    fn mut_fields(&mut self) -> (&mut bool, &mut Option<PhysicsParameters>) {
        (&mut self.in_shutdown_state, &mut self.temporary_parameters)
    }
}

impl ParticleSettings {
    fn normalized_velocity_spread(&self) -> f64 {
        let vx = self.x_velocity_variance;
        let vy = self.y_velocity_variance;
        let vz = self.z_velocity_variance;
        vx * vx + vy * vy + vz * vz
    }

    fn validate_velocity(&self) -> bool {
        self.normalized_velocity_spread() < 1.
    }

    fn max_x_velocity(&self) -> f64 {
        let vy = self.y_velocity_variance;
        let vz = self.z_velocity_variance;
        (1. - f64::EPSILON - vy * vy - vz * vz).sqrt()
    }

    fn max_y_velocity(&self) -> f64 {
        let vx = self.x_velocity_variance;
        let vz = self.z_velocity_variance;
        (1. - f64::EPSILON - vx * vx - vz * vz).sqrt()
    }

    fn max_z_velocity(&self) -> f64 {
        let vx = self.x_velocity_variance;
        let vy = self.y_velocity_variance;
        (1. - f64::EPSILON - vx * vx - vy * vy).sqrt()
    }
}

impl Default for ParticleSettings {
    fn default() -> Self {
        Self {
            spawning_enabled: true,
            y_position_variance: 150.,
            z_position_variance: 0.,
            x_velocity_variance: 0.0,
            y_velocity_variance: 0.0,
            z_velocity_variance: 0.0,
            photon_chance: 0.0
        }
    }
}

impl Default for VisualSettings {
    fn default() -> Self {
        Self {
            show_inner_bubble: true,
            show_outer_bubble: true,
            hide_untagged_particles: false
        }
    }
}

impl Default for UiState {
    fn default() -> Self {
        Self {
            need_remesh_inner_bubble: false,
            need_remesh_outer_bubble: false,
            config_path_buf: String::new(),
            history_path_buf: String::new(),
            err_text: String::new(),
            history_format: HistoryFormat::Bin,
            resume_idx_buf: 0,
            dump_only_tagged_particles: false
        }
    }
}

impl Default for SpeedControls {
    fn default() -> Self {
        Self {
            tick_rate_factor: 1.,
            need_apply: false,
            keep_up_warning: false
        }
    }
}

impl Default for ValidatorSettings {
    fn default() -> Self {
        Self {
            nan_validator: ValidatorErrBehavior::Die,
            normalization_validator: ValidatorErrBehavior::ExplodeParticle,
            normalized_tolerance: 1e-6,
            normalized_tolerance_power: -6
        }
    }
}

// hard cap of 16 args (¬▂¬)
fn ui(mut imgui_ctx: NonSendMut<ImguiContext>,
      config_states: (ResMut<VisualSettings>, ResMut<ParticleSettings>, ResMut<UiState>, ResMut<SaveLoadState>, ResMut<ShutdownState>, ResMut<ValidatorSettings>),
      controls: (ResMut<SpeedControls>, ResMut<PauseControls>),
      mut physics_manager: ResMut<PhysicsManager>,
      mut commands: Commands,
      ship_image_asset: Res<ShipImageAsset>,
      space_dust_mesh: Res<SpaceDustMesh>,
      mut color_materials: ResMut<Assets<ColorMaterial>>,
      mut space_dust_mats: ResMut<SpaceDustColorMaterials>,
      mut history: ResMut<SimulationHistory>,
      ship: Single<(Entity, &mut Transform, &mut ShipPhysics), With<Ship>>,
      particles: Query<Entity, (With<SpaceDust>, Without<Ship>)>,
      mut tagged_particles: ResMut<TaggedParticles>,
      tagged_mat: Res<TaggedSpaceDustMaterialAsset>) {
    let ui = imgui_ctx.ui();

    let (
        mut visual_settings,
        mut particle_settings,
        mut ui_state,
        mut config_state,
        mut shutdown_state,
        mut validator_settings
    ) = config_states;

    let _window = ui
        .window("Parameters")
        .size([500., 200.], imgui::Condition::FirstUseEver)
        .position([1250., 0.], imgui::Condition::FirstUseEver)
        .position_pivot([1.0, 0.])
        .build(|| {
            let (ship_entity, mut ship_transform, mut ship_state) = ship.into_inner();
            let global_time = physics_manager.global_time();

            if let Some(_tab_bar) = ui.tab_bar("SettingsTabBar") {
                if let Some(_tab_item) = ui.tab_item("Drive") {
                    let warp_drive = &mut physics_manager.physics_parameters.warp_drive;
                    let mut current_drive_index = WarpDriveKind::from(&*warp_drive).ordinal();

                    ui.disabled(shutdown_state.in_shutdown_state, || {
                        if ui.combo("Warp Drive", &mut current_drive_index, WarpDriveKind::VARIANTS, |v| v.to_string().into()) {
                            let new_drive = WarpDriveKind::from_ordinal(current_drive_index).expect("Invalid WarpDriveKind ordinal");
                            *warp_drive = warp_drive.new_from(new_drive, global_time, &mut ship_state);
                        }
                    });
                }

                if let Some(_tab_item) = ui.tab_item("Bubble") {
                    let (in_shutdown_state, temporary_parameters_opt) = shutdown_state.mut_fields();

                    let parameters = if *in_shutdown_state {
                        temporary_parameters_opt.as_mut().unwrap()
                    } else {
                        &mut physics_manager.physics_parameters
                    };
                    
                    
                    let mut radius_scratch = parameters.warp_drive.bubble_radius();
                    if ui.slider("Radius", 1., 4., &mut radius_scratch) {
                        ui_state.need_remesh_inner_bubble = true;
                        ui_state.need_remesh_outer_bubble = true;
                        parameters.warp_drive.set_bubble_radius(radius_scratch);
                    }
                    if ui.is_item_hovered() {
                        ui.tooltip_text("The radius of the inner shield.");
                    }

                    let mut sigma_scratch = parameters.warp_drive.bubble_sigma();
                    if ui.slider("Sigma", 0.1, 4., &mut sigma_scratch) {
                        ui_state.need_remesh_outer_bubble = true;
                        parameters.warp_drive.set_bubble_sigma(sigma_scratch);
                    }
                    if ui.is_item_hovered() {
                        ui.tooltip_text("The width of the transition between the inner and outer shield regions.");
                    }

                    let mut u_scratch = parameters.warp_drive.u();
                    let prev_u = u_scratch;

                    if ui.slider("Speed", 0.0, 0.9, &mut u_scratch) {
                        parameters.warp_drive.set_u(u_scratch, global_time);

                        /* The Natario drive has no shield drag, so there is nothing to
                           keep in range for it. */
                        if let Some(u0) = parameters.warp_drive.u0()
                            && !(0.1..=0.9).contains(&u0) {
                            parameters.warp_drive.set_u(prev_u, global_time);
                            ui.tooltip(|| {
                                ui.text_colored([1., 0., 0., 1.], "Shield Drag out of range!")
                            });
                        }
                    }
                    if ui.is_item_hovered() {
                        ui.tooltip_text("u");
                    }

                    match parameters.warp_drive.u0() {
                        Some(u0) => {
                            let mut u0_scratch = u0;

                            if ui.slider("Drag", 0.0, 0.9, &mut u0_scratch) {
                                if *in_shutdown_state {
                                    parameters.warp_drive.set_u0_pure(u0_scratch);
                                } else {
                                    parameters.warp_drive.set_u0(u0_scratch, &mut ship_state);
                                    ship_transform.translation = physics_to_game(ship_state.0).xy().extend(-10.);
                                }
                            }
                            if ui.is_item_hovered() {
                                ui.tooltip_text("u0");
                            }
                        },
                        None => {
                            ui.disabled(true, || {
                                let mut dummy = 0.0;
                                ui.slider("Drag", 0.0, 0.9, &mut dummy);

                                if ui.is_item_hovered() {
                                    ui.tooltip_text("Selected warp drive does not use this parameter.");
                                }
                            });
                        }
                    }

                    match parameters.warp_drive.k0() {
                        Some(k0) => {
                            let mut k0_scratch = k0;
                            if ui.slider("Deflection Strength", 0.0, 0.9, &mut k0_scratch) {
                                parameters.warp_drive.set_k0(k0_scratch);
                            }
                            if ui.is_item_hovered() {
                                ui.tooltip_text("k0");
                            }
                        },
                        None => {
                            ui.disabled(true, || {
                                let mut dummy = 0.0;
                                ui.slider("Deflection Strength", 0.0, 0.9, &mut dummy);

                                if ui.is_item_hovered() {
                                    ui.tooltip_text("Selected warp drive does not use this parameter.");
                                }
                            });
                        }
                    }

                    match parameters.warp_drive.deflector_sigma_pushout() {
                        Some(val) => {
                            let mut scratch = val;
                            if ui.slider("Sigma Pushout", 0.0, 4., &mut scratch) {
                                parameters.warp_drive.set_deflector_sigma_pushout(scratch);
                            }
                        },
                        None => {
                            ui.disabled(true, || {
                                let mut dummy = 0.0;
                                ui.slider("Sigma Pushout", 0.0, 4., &mut dummy);

                                if ui.is_item_hovered() {
                                    ui.tooltip_text("Selected warp drive does not use this parameter.");
                                }
                            });
                        }
                    };

                    match parameters.warp_drive.deflector_sigma_factor() {
                        Some(val) => {
                            let mut scratch = val;
                            if ui.slider("Sigma Factor", 0.0, 4., &mut scratch) {
                                parameters.warp_drive.set_deflector_sigma_factor(scratch);
                            }
                        },
                        None => {
                            ui.disabled(true, || {
                                let mut dummy = 0.0;
                                ui.slider("Sigma Factor", 0.0, 4., &mut dummy);

                                if ui.is_item_hovered() {
                                    ui.tooltip_text("Selected warp drive does not use this parameter.");
                                }
                            });
                        }
                    };

                    match parameters.warp_drive.deflector_back() {
                        Some(val) => {
                            let mut scratch = val;
                            if ui.slider("Deflector Back", 0.0, 1., &mut scratch) {
                                parameters.warp_drive.set_deflector_back(scratch);
                            }
                        },
                        None => {
                            ui.disabled(true, || {
                                let mut dummy = 0.0;
                                ui.slider("Deflector Back", 0.0, 1., &mut dummy);

                                if ui.is_item_hovered() {
                                    ui.tooltip_text("Selected warp drive does not use this parameter.");
                                }
                            });
                        }
                    };

                    if *in_shutdown_state {
                        if ui.button("Shut up") {
                            let restart_parameters = temporary_parameters_opt.take().unwrap();
                            physics_manager.physics_parameters.shut_up(global_time, &mut ship_state, &restart_parameters);
                            *in_shutdown_state = false;
                        }
                    } else {
                        if ui.button("Shut down") {
                            *in_shutdown_state = true;
                            *temporary_parameters_opt = Some(physics_manager.physics_parameters.clone());
                            physics_manager.physics_parameters.shut_down(global_time);
                        }
                    }
                }

                if let Some(_tab_item) = ui.tab_item("Particles") {
                    ui.checkbox("Spawn Particles", &mut particle_settings.spawning_enabled);

                    ui.separator();
                    ui.text("Initial Position");

                    ui.slider("y-Position Spread", 0.01, 150., &mut particle_settings.y_position_variance);

                    ui.slider("z-Position Spread", 0.0, 150., &mut particle_settings.z_position_variance);

                    ui.separator();
                    ui.text("Initial Velocity");

                    if ui.slider("x-Velocity Spread", 0., 0.9, &mut particle_settings.x_velocity_variance)
                        && !particle_settings.validate_velocity() {
                        particle_settings.x_velocity_variance = particle_settings.max_x_velocity();
                        ui.tooltip(|| {
                            ui.text_colored([1., 0., 0., 1.], "Normalized velocity is too great!")
                        });
                    }

                    if ui.slider("y-Velocity Spread", 0., 0.9, &mut particle_settings.y_velocity_variance)
                        && !particle_settings.validate_velocity() {
                        particle_settings.y_velocity_variance = particle_settings.max_y_velocity();
                        ui.tooltip(|| {
                            ui.text_colored([1., 0., 0., 1.], "Normalized velocity is too great!")
                        });
                    }

                    if ui.slider("z-Velocity Spread", 0., 0.9, &mut particle_settings.z_velocity_variance)
                        && !particle_settings.validate_velocity() {
                        particle_settings.z_velocity_variance = particle_settings.max_z_velocity();
                        ui.tooltip(|| {
                            ui.text_colored([1., 0., 0., 1.], "Normalized velocity is too great!")
                        });
                    }

                    ui.text_colored(
                        [0.75, 0.75, 0.75, 1.],
                        format!(
                            "Normalized velocity spread: {:.6}",
                            (particle_settings.normalized_velocity_spread() * 1000000.).floor() / 1000000.
                        ),
                    );

                    ui.separator();
                    ui.text("Particle Types");

                    ui.slider("Photon Spawn Ratio", 0.0, 1.0, &mut particle_settings.photon_chance);
                }

                if let Some(_tab_item) = ui.tab_item("Visuals") {
                    ui.checkbox("Draw inner shield", &mut visual_settings.show_inner_bubble);
                    ui.checkbox("Draw outer shield", &mut visual_settings.show_outer_bubble);
                    ui.checkbox("Hide untagged particles", &mut visual_settings.hide_untagged_particles);
                }
                
                if let Some(_tab_item) = ui.tab_item("Save & Load") {
                    ui.text("Config");

                    ui.input_text("Config Path", &mut ui_state.config_path_buf)
                        .hint("sim.json")
                        .build();
                    
                    if ui.button("Save Config") {
                        let path = if ui_state.config_path_buf.is_empty() {
                            Path::new("sim.json")
                        } else {
                            Path::new(&ui_state.config_path_buf)
                        };
                        
                        let config = GlobalConfig {
                            physics_config: (&physics_manager.physics_parameters).into(),
                            particle_settings: particle_settings.clone(),
                            visual_settings: visual_settings.clone(),
                            shutdown_config: shutdown_state.as_ref().into(),
                            validator_settings: validator_settings.clone()
                        };

                        let json = serde_json::to_string_pretty(&config).unwrap();
                        if let Err(e) = fs::write(path, json) {
                            ui_state.err_text = format!("Failed to save config: {}", e);
                            ui.open_popup("SaveLoadErr");
                        }
                    }

                    ui.same_line();

                    if ui.button("Load & Apply Config") {
                        let path = if ui_state.config_path_buf.is_empty() {
                            Path::new("sim.json")
                        } else {
                            Path::new(&ui_state.config_path_buf)
                        };

                        match fs::read_to_string(path) {
                            Ok(json) => {
                                match serde_json::from_str::<GlobalConfig>(&json) {
                                    Ok(config) => {
                                        *visual_settings = config.visual_settings;
                                        *particle_settings = config.particle_settings;
                                        *validator_settings = config.validator_settings;
                                        *shutdown_state = (&config.shutdown_config).into();
                                        ui_state.need_remesh_inner_bubble = true;
                                        ui_state.need_remesh_outer_bubble = true;

                                        let global_time = physics_manager.global_time();
                                        let params: PhysicsParameters = (&config.physics_config).into();
                                        physics_manager.physics_parameters.shut_up(global_time, ship_state.as_mut(), &params);
                                    }
                                    Err(e) => {
                                        ui_state.err_text = format!("Failed to parse config: {}", e);
                                        ui.open_popup("SaveLoadErr");
                                    }
                                }
                            }
                            Err(e) => {
                                ui_state.err_text = format!("Failed to read config file: {}", e);
                                ui.open_popup("SaveLoadErr");
                            }
                        }
                    }

                    ui.separator();
                    ui.text("Live History");

                    ui.text(format!("Checkpoints: {}", history.snapshots.len()));

                    let path_hint = match ui_state.history_format {
                        HistoryFormat::Json => "dump.json",
                        HistoryFormat::Bin => "dump.bin"
                    };
                    
                    ui.input_text("History Path", &mut ui_state.history_path_buf)
                        .hint(path_hint)
                        .build();

                    ui.text("History Format:");

                    ui.same_line();
                    ui.radio_button("Binary", &mut ui_state.history_format, HistoryFormat::Bin);
                    if ui.is_item_hovered() {
                        ui.tooltip_text("Fast. Produces small, indecipherable files.");
                    }

                    ui.same_line();
                    ui.radio_button("JSON", &mut ui_state.history_format, HistoryFormat::Json);
                    if ui.is_item_hovered() {
                        ui.tooltip_text("Slow. Produces large, human-readable files.");
                    }

                    ui.checkbox("Dump only tagged particles", &mut ui_state.dump_only_tagged_particles);

                    let dump_checkpoint = ui.button("Dump Single Checkpoint");

                    ui.same_line();

                    let dump_history;
                    {
                        let _t = ui.push_style_color(StyleColor::Button, [1., 0.25, 0.25, 1.]);
                        dump_history = ui.button("Dump Entire History");
                        if ui.is_item_hovered() {
                            ui.tooltip_text("This can be very slow and produce very large files!");
                        }
                    }

                    ui.same_line();

                    if ui.button("Trim History") {
                        history.trim();
                    }

                    if dump_checkpoint || dump_history {
                        let mut history = if dump_checkpoint {
                            history.checkpoint()
                        } else {
                            history.as_borrowed()
                        };
                        history.tagged_particles = &tagged_particles;

                        if ui_state.dump_only_tagged_particles {
                            history = history.tagged_only();
                        }

                        match ui_state.history_format {
                            HistoryFormat::Json => {
                                let path = if ui_state.history_path_buf.is_empty() {
                                    Path::new("dump.json")
                                } else {
                                    Path::new(&ui_state.history_path_buf)
                                };

                                let dump = serde_json::to_string_pretty(&history).unwrap();

                                if let Err(e) = fs::write(path, dump) {
                                    ui_state.err_text = format!("Failed to dump: {}", e);
                                    ui.open_popup("SaveLoadErr");
                                }
                            }
                            HistoryFormat::Bin => {
                                let path = if ui_state.history_path_buf.is_empty() {
                                    Path::new("dump.bin")
                                } else {
                                    Path::new(&ui_state.history_path_buf)
                                };

                                let mut oo = OpenOptions::new();
                                oo.read(true)
                                  .write(true)
                                  .create(true)
                                  // Without this, a shorter dump leaves stale bytes from a longer one.
                                  .truncate(true);

                                match oo.open(path) {
                                    Ok(file) => {
                                        let mut writer = BufWriter::new(file);

                                        let dump_result =
                                            bincode::serde::encode_into_std_write(&history, &mut writer, bincode::config::standard());

                                        if let Err(e) = dump_result {
                                            ui_state.err_text = format!("Failed to dump: {}", e);
                                            ui.open_popup("SaveLoadErr");
                                        } else {
                                            if let Err(e) = writer.flush() {
                                                ui_state.err_text = format!("Failed to dump: {}", e);
                                                ui.open_popup("SaveLoadErr");
                                            }
                                        }
                                    }
                                    Err(e) => {
                                        ui_state.err_text = format!("Failed to open file: {}", e);
                                        ui.open_popup("SaveLoadErr");
                                    }
                                }
                            }
                        }
                    }

                    ui.separator();

                    ui.text("Loaded History");

                    if ui.button("Load History") {
                        match ui_state.history_format {
                            HistoryFormat::Json => {
                                let path = if ui_state.history_path_buf.is_empty() {
                                    Path::new("dump.json")
                                } else {
                                    Path::new(&ui_state.history_path_buf)
                                };

                                match fs::read_to_string(path) {
                                    Ok(json) => {
                                        match serde_json::from_str::<SimulationHistory>(&json) {
                                            Ok(history) => {
                                                config_state.loaded_history = Some(history);
                                            }
                                            Err(e) => {
                                                ui_state.err_text = format!("Failed to parse history: {}", e);
                                                ui.open_popup("SaveLoadErr");
                                            }
                                        }
                                    }
                                    Err(e) => {
                                        ui_state.err_text = format!("Failed to read history file: {}", e);
                                        ui.open_popup("SaveLoadErr");
                                    }
                                }
                            }
                            HistoryFormat::Bin => {
                                let path = if ui_state.history_path_buf.is_empty() {
                                    Path::new("dump.bin")
                                } else {
                                    Path::new(&ui_state.history_path_buf)
                                };

                                let mut oo = OpenOptions::new();
                                oo.read(true);

                                match oo.open(path) {
                                    Ok(file) => {
                                        let mut reader = BufReader::new(file);

                                        match bincode::serde::decode_from_std_read(&mut reader, bincode::config::standard()) {
                                            Ok(history) => {
                                                config_state.loaded_history = Some(history);
                                            },
                                            Err(e) => {
                                                ui_state.err_text = format!("Failed to read history file: {}", e);
                                                ui.open_popup("SaveLoadErr");
                                            }
                                        }
                                    }
                                    Err(e) => {
                                        ui_state.err_text = format!("Failed to open file: {}", e);
                                        ui.open_popup("SaveLoadErr");
                                    }
                                }
                            }
                        }
                    }

                    ui.same_line();

                    if ui.button("Unload History") {
                        config_state.loaded_history = None;
                    }

                    match &config_state.loaded_history {
                        Some(loaded_history) => {
                            let snapshots_len = loaded_history.snapshots.len();

                            ui.text(format!("Checkpoints: {}", snapshots_len));
                            let mut load_idx: Option<usize> = None;

                            if ui.button("Resume from Start") {
                                load_idx = Some(0);
                            }

                            if ui.button("Resume from") {
                                load_idx = Some(ui_state.resume_idx_buf);
                            }

                            ui.same_line();

                            ui.input_scalar("##resume_idx_buf", &mut ui_state.resume_idx_buf)
                              .step(1)
                              .step_fast(10)
                              .build();

                            if ui.button("Resume from End") {
                                // saturating_sub keeps an empty history out of the range check below,
                                // which reports it as an error instead of underflowing.
                                load_idx = Some(snapshots_len.saturating_sub(1));
                            }

                            if let Some(load_idx) = load_idx {
                                if load_idx >= snapshots_len {
                                    ui_state.err_text = format!("Index out of range: 0 <= {} < {}", load_idx, snapshots_len);
                                    ui.open_popup("SaveLoadErr");
                                } else {
                                    let snapshot = &loaded_history.snapshots[load_idx];
                                    let config = &snapshot.global_config;

                                    *visual_settings = config.visual_settings.clone();
                                    *particle_settings = config.particle_settings.clone();
                                    *shutdown_state = (&config.shutdown_config).into();
                                    ui_state.need_remesh_inner_bubble = true;
                                    ui_state.need_remesh_outer_bubble = true;

                                    physics_manager.set_global_time(snapshot.global_time);
                                    physics_manager.physics_parameters = (&config.physics_config).into();

                                    commands.insert_resource(snapshot.seeded_rng.clone());

                                    for entity_id in particles {
                                        commands.entity(entity_id).despawn();
                                    }

                                    commands.entity(ship_entity).despawn();

                                    commands.spawn(
                                        ShipEntity::new(
                                            ship_image_asset,
                                            snapshot.ship_state.physics.clone()
                                        )
                                    );

                                    *tagged_particles = loaded_history.tagged_particles.clone();

                                    let particles_batch = snapshot.particle_states.clone();
                                    let particles_batch = particles_batch.into_iter().map(|s| {
                                        if tagged_particles.is_tagged(&s.id) {
                                            SpaceDustEntity::new_tagged_from_resume(
                                                &space_dust_mesh,
                                                &tagged_mat,
                                                s.physics,
                                                s.id,
                                                s.particle_type
                                            )
                                        } else {
                                            SpaceDustEntity::new_from_resume(
                                                &space_dust_mesh,
                                                &mut color_materials,
                                                &mut space_dust_mats,
                                                s.physics,
                                                s.id,
                                                s.particle_type
                                            )
                                        }
                                    }).collect::<Vec<_>>();

                                    //commands.spawn_batch(particles_batch);

                                    for p in particles_batch {
                                        commands.spawn(p).observe(space_dust_click_observer);
                                    }

                                    *history = loaded_history.truncated(load_idx);
                                }
                            }
                        }
                        None => {
                            ui.text_disabled("No history loaded.");
                        }
                    }

                    ui.popup("SaveLoadErr", || {
                        ui.text(&ui_state.err_text);
                        if ui.button("Dang it") {
                            ui.close_current_popup();
                        }
                    });
                }

                let (mut speed_controls, mut pause_controls) = controls;

                if let Some(_tab_item) = ui.tab_item("Speed") {
                    let invert_paused = if pause_controls.paused {
                        ui.button("Unpause")
                    } else {
                        ui.button("Pause")
                    };

                    if invert_paused {
                        pause_controls.paused = !pause_controls.paused;
                    }

                    ui.same_line();

                    if ui.button("Reset Speed") {
                        speed_controls.tick_rate_factor = 1.;
                    }

                    if ui.slider("Simulation Speed", 0.1, 6., &mut speed_controls.tick_rate_factor) {
                        speed_controls.need_apply = true;
                    }

                    if speed_controls.keep_up_warning {
                        ui.tooltip(|| {
                            ui.text_colored([1., 0.5, 0.5, 1.], "Can't keep up!");
                        });
                    }
                }

                if let Some(_tab_item) = ui.tab_item("Validation") {
                    ui.text("Check for NaN");

                    ui.radio_button("Ignore##NaNCheckIgnore", &mut validator_settings.nan_validator, ValidatorErrBehavior::Ignore);
                    ui.same_line();
                    ui.radio_button("Die##NaNCheckDie", &mut validator_settings.nan_validator, ValidatorErrBehavior::Die);
                    ui.same_line();
                    ui.radio_button("Remove Particle##NaNCheckRemove", &mut validator_settings.nan_validator, ValidatorErrBehavior::RemoveParticle);
                    ui.same_line();
                    ui.radio_button("Explode Particle##NaNCheckExplode", &mut validator_settings.nan_validator, ValidatorErrBehavior::ExplodeParticle);

                    ui.separator();

                    ui.text("Check for non-normalized state");

                    ui.radio_button("Ignore##NormCheckIgnore", &mut validator_settings.normalization_validator, ValidatorErrBehavior::Ignore);
                    ui.same_line();
                    ui.radio_button("Die##NormCheckDie", &mut validator_settings.normalization_validator, ValidatorErrBehavior::Die);
                    ui.same_line();
                    ui.radio_button("Remove Particle##NormCheckRemove", &mut validator_settings.normalization_validator, ValidatorErrBehavior::RemoveParticle);
                    ui.same_line();
                    ui.radio_button("Explode Particle##NormCheckExplode", &mut validator_settings.normalization_validator, ValidatorErrBehavior::ExplodeParticle);

                    let tolerance_slider =
                        ui.slider_config("Tolerance", -12, -1)
                          .flags(SliderFlags::NO_ROUND_TO_FORMAT)
                          .display_format("1e%d");

                    if tolerance_slider.build(&mut validator_settings.normalized_tolerance_power) {
                        validator_settings.normalized_tolerance = 10f64.powi(validator_settings.normalized_tolerance_power);
                    }
                }

                /* AGPL section 5(d) asks an interactive program to display its legal
                   notices. This tab is where they live, and it also carries the
                   CC-BY attribution the ship sprite requires. */
                if let Some(_tab) = ui.tab_item("About") {
                    ui.text(format!("DeflectorShieldSimulator {}", env!("CARGO_PKG_VERSION")));
                    ui.text("Max Morris, Lucas Timotheo Sanches, Steven Robert Brandt");
                    ui.text("Louisiana State University");

                    ui.separator();

                    ui.text("Licensed under the GNU Affero General Public License,");
                    ui.text("version 3 or later. This program comes with absolutely no");
                    ui.text("warranty. You may redistribute it under those terms.");
                    ui.text("Source:");
                    ui.same_line();
                    ui.text(SOURCE_URL);

                    ui.separator();

                    ui.text("If this simulator contributes to published research, cite:");
                    ui.text_wrapped(PAPER_CITATION);

                    ui.separator();

                    ui.text("Ship sprite by arin48, CC-BY 4.0.");
                    ui.text("opengameart.org/content/top-down-spaceships");
                    ui.text("Third-party crate notices ship in THIRD-PARTY-NOTICES.md.");

                    ui.separator();

                    ui.text("Manual:");
                    ui.same_line();
                    ui.text(MANUAL_URL);
                }
            }
        });
}