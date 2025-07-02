use crate::physics_manager::PhysicsManager;
use bevy::app::{App, PostUpdate};
use bevy::prelude::{NonSendMut, Plugin, ResMut, Resource};
use bevy_mod_imgui::ImguiContext;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(VisualSettings::default())
           .insert_resource(UiState::default())
           .add_systems(PostUpdate, ui);
    }
}

#[derive(Resource)]
pub struct VisualSettings {
    pub show_inner_bubble: bool,
    pub show_outer_bubble: bool
}

impl Default for VisualSettings {
    fn default() -> Self {
        Self {
            show_inner_bubble: true,
            show_outer_bubble: true
        }
    }
}

#[derive(Resource)]
pub struct UiState {
    pub need_remesh_inner_bubble: bool,
    pub need_remesh_outer_bubble: bool
}


impl Default for UiState {
    fn default() -> Self {
        Self {
            need_remesh_inner_bubble: false,
            need_remesh_outer_bubble: false,
        }
    }
}

fn ui(mut imgui_ctx: NonSendMut<ImguiContext>,
      mut visual_settings: ResMut<VisualSettings>,
      mut state: ResMut<UiState>,
      mut physics_manager: ResMut<PhysicsManager>) {
    let ui = imgui_ctx.ui();
    let physics_params = &mut physics_manager.physics_parameters;

    let window = ui
        .window("Parameters")
        .size([500., 200.], imgui::Condition::FirstUseEver)
        .position([1250., 0.,], imgui::Condition::FirstUseEver)
        .position_pivot([1.0, 0.])
        .build(|| {
            if let Some(_tab_bar) = ui.tab_bar("SettingsTabBar") {
                if let Some(_tab_item) = ui.tab_item("Physics") {
                    if ui.slider("Shield Radius", 1., 4., &mut physics_params.warp_drive.radius) {
                        state.need_remesh_inner_bubble = true;
                        state.need_remesh_outer_bubble = true;
                    }
                    if ui.is_item_hovered() {
                        ui.tooltip_text("The radius of the inner shield.");
                    }

                    if ui.slider("Shield Sigma", 0.1, 4., &mut physics_params.warp_drive.sigma) {
                        state.need_remesh_outer_bubble = true;
                    }
                    if ui.is_item_hovered() {
                        ui.tooltip_text("The width of the transition between the inner and outer shield regions.");
                    }

                    /*if ui.slider("u, u0", 0.1, 0.9, &mut physics_params.warp_drive.u) {
                        physics_params.set_u0(physics_params.warp_drive.u);
                    }
                    if ui.is_item_hovered() {
                        ui.tooltip_text("Shield Speed");
                    }*/

                    ui.slider("k0", 0.0, 0.9, &mut physics_params.warp_drive.k0);
                    if ui.is_item_hovered() {
                        ui.tooltip_text("Deflection Strength");
                    }
                }

                if let Some(_tab_item) = ui.tab_item("Visuals") {
                    ui.checkbox("Draw inner shield", &mut visual_settings.show_inner_bubble);
                    ui.checkbox("Draw outer shield", &mut visual_settings.show_outer_bubble);
                }
            }
        });
}