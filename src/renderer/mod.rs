use sdl2::gfx::primitives::DrawRenderer;
use sdl2::pixels::Color;
use sdl2::render::WindowCanvas;

lazy_static! {
    static ref TANGENT_LENGTH_FACTOR: f32 = 3f32.sqrt() / 2f32;
}

pub fn render_hex(
    canvas: &WindowCanvas,
    center: (i16, i16),
    radius: i16,
    color: Color,
) -> Result<(), String> {
    let (x, y) = center;
    let tangent_length = (*TANGENT_LENGTH_FACTOR * (radius as f32)).round() as i16;
    let r_half = (radius / 2) as i16;

    let p1 = (x, y - radius);
    let p2 = (x + tangent_length, y - r_half);
    let p3 = (x + tangent_length, y + r_half);
    let p4 = (x, y + radius);
    let p5 = (x - tangent_length, y + r_half);
    let p6 = (x - tangent_length, y - r_half);

    canvas.filled_polygon(
        &[p1.0, p2.0, p3.0, p4.0, p5.0, p6.0],
        &[p1.1, p2.1, p3.1, p4.1, p5.1, p6.1],
        color,
    )
}
