use pf2e_terrain_gen::app_state::AppState;
use pf2e_terrain_gen::events::handle_events;
use pf2e_terrain_gen::map::MapState;
use pf2e_terrain_gen::rendering::HexRenderer;
use pf2e_terrain_gen::viewport::ViewPortState;
use sdl2::pixels::Color;
use sdl2::render::WindowCanvas;
use sdl2::EventPump;

// TODO: Double check types (unsigned vs. signed & size)
const SCREEN_WIDTH: u32 = 1024;
const SCREEN_HEIGHT: u32 = 720;
const WINDOW_TITLE: &str = "PF2e Terrain Generator";
// roughly earth sized: 2076
const MAP_SIZE: (u16, u16) = (1500, 1500);
const SMOOTHING_ITERATIONS: u16 = 0; //5;

fn main() -> Result<(), String> {
    let (mut event_pump, mut canvas) = show_window()?;

    // TODO: Maybe add option to intentionally slow down generation to be able to see the steps more easily

    // TODO: Infinite Scrolling/Wrap-around effect (only in horizontal direction)

    // TODO: Zoom only until whole map is on screen at once
    //  (would prevent "bug" of map "disappearing" when zooming where it actually just gets moved offscreen)

    // TODO: Zoom to MousePos

    // TODO: MAYBE Randomly generate elevation (highs/lows less likely?), smooth elevation & color depending on elevation

    let mut app_state = AppState {
        map_state: MapState::new((MAP_SIZE.0, MAP_SIZE.1), SMOOTHING_ITERATIONS)?,
        viewport_state: ViewPortState::new(),
    };

    loop {
        let quit = handle_events(&mut event_pump, &mut app_state)?;
        if quit {
            break;
        }

        canvas.set_draw_color(Color::RGB(50, 50, 50));
        canvas.clear();

        HexRenderer::new(app_state.viewport_state.zoom_level).render_map(
            &canvas,
            app_state.viewport_state.offset,
            &app_state.map_state,
            true,
        )?;

        canvas.present();
    }

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
