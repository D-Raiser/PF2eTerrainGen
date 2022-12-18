use sdl2::gfx::primitives::DrawRenderer;
use sdl2::pixels::Color;
use sdl2::render::WindowCanvas;

lazy_static! {
    static ref SQRT_3: f32 = 3f32.sqrt();
    static ref TANGENT_LENGTH_FACTOR: f32 = *SQRT_3 / 2f32;
}

pub fn render_hex_indexed(
    canvas: &WindowCanvas,
    offset: (i16, i16),
    index: (i16, i16),
    // the distance from the middle point to a corner of the hex
    radius: i16,
    color: Color,
) -> Result<(), String> {
    let (x_i, y_i) = index;
    let (x, y) = (x_i as f32, y_i as f32);

    let width = *SQRT_3 * (radius as f32);
    // the distance from the middle point to the middle of an edge of the hex
    let x_radius = width / 2f32;
    let y_radius = radius as f32;
    let r_half = y_radius / 2f32;

    // every 2nd row needs to be shifted by half a hex for a continuous pattern
    let row_offset = ((y_i % 2) as f32) * (width / 2f32);
    let (offset_x, offset_y) = (offset.0 as f32, offset.1 as f32);

    let center_x = (x * width) + row_offset + (width / 2f32) - offset_x;
    let center_y = y * width * *TANGENT_LENGTH_FACTOR + y_radius - offset_y;

    let p1 = round_to_pixel_precision((center_x, center_y - y_radius)); // top
    let p2 = round_to_pixel_precision((center_x + x_radius, center_y - r_half)); // top-right
    let p3 = round_to_pixel_precision((center_x + x_radius, center_y + r_half)); // bottom-right
    let p4 = round_to_pixel_precision((center_x, center_y + y_radius)); // bottom
    let p5 = round_to_pixel_precision((center_x - x_radius, center_y + r_half)); // bottom-left
    let p6 = round_to_pixel_precision((center_x - x_radius, center_y - r_half)); // top-left

    canvas.filled_polygon(
        &[p1.0, p2.0, p3.0, p4.0, p5.0, p6.0],
        &[p1.1, p2.1, p3.1, p4.1, p5.1, p6.1],
        color,
    )
}

fn round_to_pixel_precision(p: (f32, f32)) -> (i16, i16) {
    (p.0.round() as i16, p.1.round() as i16)
}
