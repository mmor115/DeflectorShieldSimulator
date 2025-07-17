use bevy::input::keyboard::KeyboardInput;
use bevy::prelude::*;
use crate::game_systems::ui::PauseControls;

pub fn handle_space_bar(mut kb_events: EventReader<KeyboardInput>,
                        mut pause_controls: ResMut<PauseControls>) {
    for event in kb_events.read() {
        if let KeyCode::Space = event.key_code && event.state.is_pressed() {
            pause_controls.paused = !pause_controls.paused;
        }
    }
}