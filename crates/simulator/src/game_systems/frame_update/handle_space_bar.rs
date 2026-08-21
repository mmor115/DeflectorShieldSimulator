use bevy::input::keyboard::KeyboardInput;
use bevy::prelude::*;
use bevy_mod_imgui::ImguiContext;
use crate::game_systems::ui::PauseControls;

pub fn handle_space_bar(mut imgui_ctx: NonSendMut<ImguiContext>,
                        mut kb_events: EventReader<KeyboardInput>,
                        mut pause_controls: ResMut<PauseControls>) {
    /* The imgui frame opens in PreUpdate and closes in Last, so ui() is valid here.
       Without this check, a space typed into the Config Path or History Path field
       also toggles the pause state. */
    if imgui_ctx.ui().io().want_text_input {
        kb_events.clear();
        return;
    }

    for event in kb_events.read() {
        if let KeyCode::Space = event.key_code && event.state.is_pressed() {
            pause_controls.paused = !pause_controls.paused;
        }
    }
}
