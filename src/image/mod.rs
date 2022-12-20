use crate::map::MapState;
use crate::rendering::HexRenderer;
use chrono::Utc;
use sdl2::image::SaveSurface;
use sdl2::pixels::PixelFormatEnum;
use sdl2::render::Canvas;
use sdl2::surface::Surface;

// MAYBE TODO: When the map and/or hex-size gets too big, calling this function will crash the
//  application because of OOM (see attempt to fix in split_attempt.rs)
pub fn save_as_png(map_state: &MapState, hex_radius: i16) -> Result<(), String> {
    let pixel_format = PixelFormatEnum::RGBA8888;
    let renderer = HexRenderer::new(hex_radius);
    let (width, height) = renderer.get_bounds(map_state.map_size);
    println!(
        "Exporting image with dimensions {:?}, depending on the size and resolution,\
        this might take a while and the application might also be marked as not responding!",
        (width, height),
    );

    let surface = Surface::new(width as u32, height as u32, pixel_format)?;
    let canvas = Canvas::from_surface(surface)?;

    renderer.render_map(&canvas, (0, 0), &map_state, false)?;

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

    surface.save(format!("./{}.png", Utc::now().timestamp()))?;
    println!("Successfully saved image");
    Ok(())
}
