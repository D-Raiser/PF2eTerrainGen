mod map;
mod renderer;

#[macro_use]
extern crate lazy_static;

use crate::map::{Hex, HexType, Map};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::render::WindowCanvas;
use sdl2::EventPump;

const SCREEN_WIDTH: u32 = 800;
const SCREEN_HEIGHT: u32 = 600;
const WINDOW_TITLE: &str = "PF2e Terrain Generator";
const HEX_SIZE: i16 = 20;

fn main() -> Result<(), String> {
    let (mut event_pump, mut canvas) = show_window()?;

    // TODO: Make it possible to generate a new map without restarting the app
    // TODO: h&w must be even to for seamless looping
    // TODO: Generation in separate thread (with RWMutex) so that we can already render the partial map
    //  and see updates
    // TODO: Maybe intentionally slow down generation then to be able to see the steps properly
    // TODO: Infinite Scrolling/Wrap-around effect
    let map = Map::generate(10, 10)?;

    loop {
        let quit = handle_events(&mut event_pump);
        if quit {
            break;
        }

        // TODO: Store origin + window size in struct to be able to drag map
        canvas.set_draw_color(Color::RGB(50, 50, 50));
        canvas.clear();

        for (y, row) in map.tiles.iter().enumerate() {
            for (x, hex) in row.iter().enumerate() {
                renderer::render_hex_indexed(
                    &canvas,
                    (x as i16, y as i16),
                    HEX_SIZE,
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

fn handle_events(event_pump: &mut EventPump) -> bool {
    for event in event_pump.poll_iter() {
        match event {
            Event::Quit { .. }
            | Event::KeyDown {
                keycode: Some(Keycode::Escape),
                ..
            } => {
                return true;
            }
            _ => {}
        }
    }
    return false;
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
