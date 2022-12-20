use pf2e_terrain_gen::map::{Map, MapState};
use pf2e_terrain_gen::rendering::HexRenderer;
use pf2e_terrain_gen::viewport::ViewPortState;
use sdl2::event::Event;
use sdl2::image::SaveSurface;
use sdl2::keyboard::Keycode;
use sdl2::pixels::{Color, PixelFormatEnum};
use sdl2::render::{Canvas, WindowCanvas};
use sdl2::surface::Surface;
use sdl2::EventPump;

const SCREEN_WIDTH: u32 = 800;
const SCREEN_HEIGHT: u32 = 600;
const WINDOW_TITLE: &str = "PF2e Terrain Generator";
// roughly earth sized: 2076
const MAP_SIZE: (i16, i16) = (2076, 2076);
const SMOOTHING_ITERATIONS: u16 = 0;

struct AppState {
    pub viewport_state: ViewPortState,
    pub map_state: MapState,
}

fn main() -> Result<(), String> {
    let (mut event_pump, mut canvas) = show_window()?;

    // TODO: Zoom only until whole map is on screen at once
    // TODO: Generation in separate thread (with RWMutex) so that we can already render the partial map
    //  and see updates
    //  + Maybe intentionally slow down generation then to be able to see the steps properly
    // TODO: Infinite Scrolling/Wrap-around effect (only in horizontal direction)
    // TODO: Zoom to MousePos

    // TODO: MAYBE Randomly generate elevation (highs/lows less likely?), smooth elevation & color depending on elevation
    let mut app_state = AppState {
        map_state: MapState::new(MAP_SIZE.0, MAP_SIZE.0, SMOOTHING_ITERATIONS)?,
        viewport_state: ViewPortState::new(),
    };

    loop {
        let quit = handle_events(&mut event_pump, &mut app_state)?;
        if quit {
            break;
        }

        canvas.set_draw_color(Color::RGB(50, 50, 50));
        canvas.clear();

        HexRenderer::new(
            app_state.viewport_state.zoom_level,
            app_state.viewport_state.offset,
        )
        .render_map(&canvas, &app_state.map_state, true)?;

        canvas.present();
    }

    Ok(())
}

// TODO: Maybe refactor to `events` module?
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
                app_state.map_state.map =
                    Map::generate(app_state.map_state.map_size, app_state.map_state.iterations)?;
            }
            Event::KeyDown {
                keycode: Some(Keycode::P),
                ..
            } => {
                save_as_png(&app_state.map_state)?;
            }
            _ => {
                app_state.viewport_state.handle_events(event);
            }
        }
    }
    return Ok(false);
}

fn save_as_png(map_state: &MapState) -> Result<(), String> {
    let pixel_format = PixelFormatEnum::RGBA8888;
    // TODO: Memory Crash when total size too big
    let renderer = HexRenderer::new(6, (0, 0));
    let (width, height) = renderer.get_bounds(map_state.map_size);
    println!("{:?}", renderer.get_bounds(map_state.map_size));

    let surface = Surface::new(width, height, pixel_format)?;
    let canvas = Canvas::from_surface(surface)?;

    renderer.render_map(&canvas, &map_state, false)?;

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

    // TODO: Timestamp in name to not overwrite existing ones?
    surface.save("./test.png")?;
    println!("Successfully saved image");
    Ok(())
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
