use crate::physics_manager::PhysicsManager;
use bevy::app::{App, PostUpdate};
use bevy::math::Vec3Swizzles;
use bevy::prelude::{NonSendMut, Plugin, ResMut, Resource, Single, Transform, With};
use bevy_mod_imgui::ImguiContext;
use crate::{physics_to_game, Ship, ShipPhysics};

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(VisualSettings::default())
           .insert_resource(ParticleSettings::default())
           .insert_resource(UiState::default())
           .add_systems(PostUpdate, ui);
    }
}

#[derive(Resource)]
pub struct VisualSettings {
    pub show_inner_bubble: bool,
    pub show_outer_bubble: bool
}

#[derive(Resource)]
pub struct ParticleSettings {
    pub y_position_variance: f32, // game units
    pub x_velocity_variance: f64, // physics units
    pub y_velocity_variance: f64, // physics units
}

#[derive(Resource)]
pub struct UiState {
    pub need_remesh_inner_bubble: bool,
    pub need_remesh_outer_bubble: bool,
    pub in_shutdown_state: bool
}


impl Default for VisualSettings {
    fn default() -> Self {
        Self {
            show_inner_bubble: true,
            show_outer_bubble: true
        }
    }
}

impl ParticleSettings {
    fn normalized_velocity_spread(&self) -> f64 {
        let vx = self.x_velocity_variance;
        let vy = self.y_velocity_variance;
        vx*vx + vy*vy
    }
    
    fn validate_velocity(&self) -> bool {
        let vx = self.x_velocity_variance;
        let vy = self.y_velocity_variance;
        self.normalized_velocity_spread() < 1.
    }
    
    fn max_x_velocity(&self) -> f64 {
        let vy = self.y_velocity_variance;
        (1. - f64::EPSILON - vy*vy).sqrt()
    }
    
    fn max_y_velocity(&self) -> f64 {
        let vx = self.x_velocity_variance;
        (1. - f64::EPSILON - vx*vx).sqrt()
    }
}

impl Default for ParticleSettings {
    fn default() -> Self {
        Self {
            y_position_variance: 150.,
            x_velocity_variance: 0.0,
            y_velocity_variance: 0.0
        }
    }
}

impl Default for UiState {
    fn default() -> Self {
        Self {
            need_remesh_inner_bubble: false,
            need_remesh_outer_bubble: false,
            in_shutdown_state: false
        }
    }
}

fn ui(mut imgui_ctx: NonSendMut<ImguiContext>,
      mut visual_settings: ResMut<VisualSettings>,
      mut particle_settings: ResMut<ParticleSettings>,
      mut state: ResMut<UiState>,
      mut physics_manager: ResMut<PhysicsManager>,
      ship: Single<(&mut Transform, &mut ShipPhysics), With<Ship>>) {
    let ui = imgui_ctx.ui();

    let window = ui
        .window("Parameters")
        .size([500., 200.], imgui::Condition::FirstUseEver)
        .position([1250., 0.,], imgui::Condition::FirstUseEver)
        .position_pivot([1.0, 0.])
        .build(|| {
            if let Some(_tab_bar) = ui.tab_bar("SettingsTabBar") {
                if let Some(_tab_item) = ui.tab_item("Shield") {
                    if state.in_shutdown_state {
                        ui.text_colored([1., 0., 0., 1.], "Shield has been shut down")
                    } else {
                        let mut radius_scratch = physics_manager.physics_parameters.bubble_radius();
                        if ui.slider("Radius", 1., 4., &mut radius_scratch) {
                            state.need_remesh_inner_bubble = true;
                            state.need_remesh_outer_bubble = true;
                            physics_manager.physics_parameters.set_bubble_radius(radius_scratch);
                        }
                        if ui.is_item_hovered() {
                            ui.tooltip_text("The radius of the inner shield.");
                        }

                        let mut sigma_scratch = physics_manager.physics_parameters.bubble_sigma();
                        if ui.slider("Sigma", 0.1, 4., &mut sigma_scratch) {
                            state.need_remesh_outer_bubble = true;
                            physics_manager.physics_parameters.set_bubble_sigma(sigma_scratch);
                        }
                        if ui.is_item_hovered() {
                            ui.tooltip_text("The width of the transition between the inner and outer shield regions.");
                        }

                        let mut u_scratch = physics_manager.physics_parameters.u();
                        let prev_u = u_scratch;

                        if ui.slider("Speed", 0.1, 0.9, &mut u_scratch) {
                            let t = physics_manager.global_time();
                            physics_manager.physics_parameters.set_u(u_scratch, t);

                            let u0 = physics_manager.physics_parameters.u0();
                            if u0 < 0.1 || u0 > 0.9 {
                                physics_manager.physics_parameters.set_u(prev_u, t);
                                ui.tooltip(|| {
                                    ui.text_colored([1., 0., 0., 1.], "Shield Drag out of range!")
                                });
                            }
                        }
                        if ui.is_item_hovered() {
                            ui.tooltip_text("u");
                        }

                        let mut u0_scratch = physics_manager.physics_parameters.u0();
                        if ui.slider("Drag", 0.1, 0.9, &mut u0_scratch) {
                            let (mut ship_transform, mut ship_state) = ship.into_inner();
                            physics_manager.physics_parameters.set_u0(u0_scratch, &mut ship_state);
                            ship_transform.translation = physics_to_game(ship_state.0).xy().extend(-10.);
                        }
                        if ui.is_item_hovered() {
                            ui.tooltip_text("u0");
                        }

                        let mut k0_scratch = physics_manager.physics_parameters.k0();
                        ui.slider("Deflection Strength", 0.0, 0.9, &mut k0_scratch);
                        if ui.is_item_hovered() {
                            ui.tooltip_text("k0");
                            physics_manager.physics_parameters.set_k0(k0_scratch);
                        }

                        if ui.button("Shut down") {
                            physics_manager.physics_parameters.shut_down_now();
                            state.in_shutdown_state = true;
                        }
                    }
                }

                if let Some(_tab_item) = ui.tab_item("Particles") {
                    ui.slider("y-Position Spread", 0.01, 150., &mut particle_settings.y_position_variance);

                    if ui.slider("x-Velocity Spread", 0., 0.9, &mut particle_settings.x_velocity_variance) {
                        if !particle_settings.validate_velocity() {
                            particle_settings.x_velocity_variance = particle_settings.max_x_velocity();
                            ui.tooltip(|| {
                                ui.text_colored([1., 0., 0., 1.], "Normalized velocity is too great!")
                            });
                        }
                    }
                    
                    if ui.slider("y-Velocity Spread", 0., 0.9, &mut particle_settings.y_velocity_variance) {
                        if !particle_settings.validate_velocity() {
                            particle_settings.y_velocity_variance = particle_settings.max_y_velocity();
                            ui.tooltip(|| {
                                ui.text_colored([1., 0., 0., 1.], "Normalized velocity is too great!")
                            });
                        }
                    }

                    ui.separator();

                    ui.text_colored(
                        [0.75, 0.75, 0.75, 1.],
                        format!(
                            "Normalized velocity spread: {:.6}",
                            (particle_settings.normalized_velocity_spread() * 1000000.).floor() / 1000000.
                        )
                    );
                }

                if let Some(_tab_item) = ui.tab_item("Visuals") {
                    ui.checkbox("Draw inner shield", &mut visual_settings.show_inner_bubble);
                    ui.checkbox("Draw outer shield", &mut visual_settings.show_outer_bubble);
                }
            }
        });
}