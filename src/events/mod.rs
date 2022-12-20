use crate::app_state::AppState;
use crate::image::save_as_png;
use crate::map::Map;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::EventPump;

pub fn handle_events(event_pump: &mut EventPump, app_state: &mut AppState) -> Result<bool, String> {
    for event in event_pump.poll_iter() {
        match event {
            Event::Quit { .. }
            | Event::KeyDown {
                keycode: Some(Keycode::Escape),
                ..
            } => {
                return Ok(true);
            }
            Event::KeyDown {
                keycode: Some(Keycode::R),
                ..
            } => {
                app_state.map_state.map =
                    Map::generate(app_state.map_state.map_size, app_state.map_state.iterations)?;
            }
            Event::KeyDown {
                keycode: Some(Keycode::P),
                ..
            } => {
                // 30: 400x400 works, 500x500 breaks
                // 40: 300x300 works,
                save_as_png(&app_state.map_state, 40)?;
            }
            _ => {
                app_state.viewport_state.handle_events(event);
            }
        }
    }
    return Ok(false);
}
