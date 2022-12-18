mod map;
mod renderer;
mod viewport;

#[macro_use]
extern crate lazy_static;

use crate::map::{Hex, HexType, Map, MapState};
use crate::viewport::ViewPortState;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::render::WindowCanvas;
use sdl2::EventPump;

const SCREEN_WIDTH: u32 = 800;
const SCREEN_HEIGHT: u32 = 600;
const WINDOW_TITLE: &str = "PF2e Terrain Generator";
const MAP_SIZE: (i16, i16) = (50, 50);

struct AppState {
    pub viewport_state: ViewPortState,
    pub map_state: MapState,
}

fn main() -> Result<(), String> {
    let (mut event_pump, mut canvas) = show_window()?;

    // TODO: Export to image file with max/high zoom level
    // TODO: Generation in separate thread (with RWMutex) so that we can already render the partial map
    //  and see updates
    // TODO: Maybe intentionally slow down generation then to be able to see the steps properly
    // TODO: Infinite Scrolling/Wrap-around effect
    // TODO: Zoom to MousePos?
    let mut app_state = AppState {
        map_state: MapState::new(MAP_SIZE.0, MAP_SIZE.0)?,
        viewport_state: ViewPortState::new(),
    };

    loop {
        let quit = handle_events(&mut event_pump, &mut app_state)?;
        if quit {
            break;
        }

        canvas.set_draw_color(Color::RGB(50, 50, 50));
        canvas.clear();

        for (y, row) in app_state.map_state.map.tiles.iter().enumerate() {
            for (x, hex) in row.iter().enumerate() {
                renderer::render_hex_indexed(
                    &canvas,
                    app_state.viewport_state.offset,
                    (x as i16, y as i16),
                    app_state.viewport_state.zoom_level,
                    color_for_hex(hex),
                )?;
            }
        }
        canvas.present();
    }

    Ok(())
}

fn color_for_hex(hex: &Hex) -> Color {
    match hex.hex_type {
        HexType::Water => Color::BLUE,
        HexType::Forest => Color::GREEN,
        _ => Color::RGB(50, 50, 50),
    }
}

fn handle_events(event_pump: &mut EventPump, app_state: &mut AppState) -> Result<bool, String> {
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
                app_state.map_state.map = Map::generate(app_state.map_state.map_size)?;
            }
            _ => {
                app_state.viewport_state.handle_events(event);
            }
        }
    }
    return Ok(false);
}

fn show_window() -> Result<(EventPump, WindowCanvas), String> {
    let sdl_context = sdl2::init()?;
    let window = match sdl_context
        .video()?
        .window(WINDOW_TITLE, SCREEN_WIDTH, SCREEN_HEIGHT)
        .resizable()
        .position_centered()
        .build()
    {
        Ok(w) => w,
        Err(e) => return Err(e.to_string()),
    };
    let canvas = match window.into_canvas().build() {
        Ok(c) => c,
        Err(e) => return Err(e.to_string()),
    };
    let event_pump = sdl_context.event_pump()?;

    Ok((event_pump, canvas))
}
