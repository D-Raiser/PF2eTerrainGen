mod map;
mod viewport;

use crate::map::{Hex, HexType, Map, MapState};
use crate::viewport::ViewPortState;
use pf2e_terrain_gen::rendering::render_hex_indexed;
use sdl2::event::Event;
use sdl2::image::SaveSurface;
use sdl2::keyboard::Keycode;
use sdl2::pixels::{Color, PixelFormatEnum};
use sdl2::render::{Canvas, RenderTarget, WindowCanvas};
use sdl2::surface::Surface;
use sdl2::EventPump;

const SCREEN_WIDTH: u32 = 800;
const SCREEN_HEIGHT: u32 = 600;
const WINDOW_TITLE: &str = "PF2e Terrain Generator";
const MAP_SIZE: (i16, i16) = (10, 10);

struct AppState {
    pub viewport_state: ViewPortState,
    pub map_state: MapState,
}

fn main() -> Result<(), String> {
    let (mut event_pump, mut canvas) = show_window()?;

    // TODO: Export to image file with max/high zoom level
    // TODO: Continue refactoring into lib crate?
    // TODO: Zoom only until whole map is on screen at once (apply similar limit for "save as png"?)
    // TODO: Generation in separate thread (with RWMutex) so that we can already render the partial map
    //  and see updates
    // TODO: Maybe intentionally slow down generation then to be able to see the steps properly
    // TODO: Infinite Scrolling/Wrap-around effect
    // TODO: Zoom to MousePos?
    // TODO: Randomly generate elevation (highs/lows less likely?), smooth elevation & color depending on elevation
    // TODO: More terrains
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

        render_map(&canvas, &app_state.map_state.map, &app_state.viewport_state)?;

        canvas.present();
    }

    Ok(())
}

fn render_map<T: RenderTarget>(
    canvas: &Canvas<T>,
    map: &Map,
    viewport_state: &ViewPortState,
) -> Result<(), String> {
    for (y, row) in map.tiles.iter().enumerate() {
        for (x, hex) in row.iter().enumerate() {
            render_hex_indexed(
                &canvas,
                viewport_state.offset,
                (x as i16, y as i16),
                viewport_state.zoom_level,
                color_for_hex(hex),
            )?;
        }
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
            Event::KeyDown {
                keycode: Some(Keycode::P),
                ..
            } => {
                save_as_png(&app_state.map_state.map, &app_state.viewport_state)?;
            }
            _ => {
                app_state.viewport_state.handle_events(event);
            }
        }
    }
    return Ok(false);
}

fn save_as_png(map: &Map, viewport_state: &ViewPortState) -> Result<(), String> {
    let pixel_format = PixelFormatEnum::RGBA8888;
    let surface = Surface::new(1000, 1000, pixel_format)?;
    let canvas = Canvas::from_surface(surface)?;

    render_map(&canvas, map, viewport_state)?;

    let mut pixels = canvas.read_pixels(None, pixel_format)?;
    let (width, height) = canvas.output_size()?;
    let pitch = pixel_format.byte_size_of_pixels(width as usize);
    let surface = Surface::from_data(
        pixels.as_mut_slice(),
        width,
        height,
        pitch as u32,
        pixel_format,
    )?;

    surface.save("./test.png")
}

fn show_window() -> Result<(EventPump, WindowCanvas), String> {
    let sdl_context = sdl2::init()?;
    let window = sdl_context
        .video()?
        .window(WINDOW_TITLE, SCREEN_WIDTH, SCREEN_HEIGHT)
        .resizable()
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;
    let canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
    let event_pump = sdl_context.event_pump()?;

    Ok((event_pump, canvas))
}
