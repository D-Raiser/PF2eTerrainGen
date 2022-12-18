mod renderer;

#[macro_use]
extern crate lazy_static;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::render::WindowCanvas;
use sdl2::EventPump;

const SCREEN_WIDTH: u32 = 2560;
const SCREEN_HEIGHT: u32 = 1440;
const WINDOW_TITLE: &str = "PF2e Terrain Generator";

fn main() -> Result<(), String> {
    let (mut event_pump, mut canvas) = show_window()?;

    loop {
        let quit = handle_events(&mut event_pump);
        if quit {
            break;
        }

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        renderer::render_hex(&canvas, (600, 600), 500, Color::RGB(128, 0, 128))?;
        renderer::render_hex(&canvas, (50, 50), 40, Color::RGB(0, 128, 128))?;

        canvas.present();
    }

    Ok(())
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
