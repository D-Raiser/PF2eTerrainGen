use once_cell::sync::Lazy;
use sdl2::gfx::primitives::DrawRenderer;
use sdl2::pixels::Color;
use sdl2::render::{Canvas, RenderTarget};

static SQRT_3: Lazy<f32> = Lazy::new(|| 3f32.sqrt());
static TANGENT_LENGTH_FACTOR: Lazy<f32> = Lazy::new(|| *SQRT_3 / 2f32);

// to only compute numbers identical across all hexes once
pub struct HexRenderer {
    pub width: f32,
    // the vertical distance between hexagons of two connected rows, NOT the true height of the hex
    pub tiling_height: f32,
    skip_offscreen: bool,
    pub offset_x: f32,
    pub offset_y: f32,
    // the distance from the middle point to the middle of an edge of the hex
    x_radius: f32,
    pub y_radius: f32,
    // half the length of any of the sides of all six triangles that make up the hexagon
    half_radius: f32,
}

impl HexRenderer {
    pub fn new(skip_offscreen: bool, hex_radius: i16, viewport_offset: (i16, i16)) -> HexRenderer {
        let y_radius = hex_radius as f32;
        let width = *SQRT_3 * y_radius;
        let height = width * *TANGENT_LENGTH_FACTOR;
        let x_radius = width / 2f32;
        let r_half = y_radius / 2f32;

        HexRenderer {
            half_radius: r_half,
            offset_x: viewport_offset.0 as f32,
            offset_y: viewport_offset.1 as f32,
            width,
            tiling_height: height,
            y_radius,
            x_radius,
            skip_offscreen,
        }
    }

    pub fn render_hex_indexed<T: RenderTarget>(
        &self,
        canvas: &Canvas<T>,
        index: (i16, i16),
        // the distance from the middle point to a corner of the hex
        color: Color,
    ) -> Result<(), String> {
        let (x_i, y_i) = index;
        let (x, y) = (x_i as f32, y_i as f32);

        // every 2nd row needs to be shifted by half a hex for a continuous pattern
        let row_offset = ((y_i % 2) as f32) * self.x_radius;

        let center_x = x * self.width + self.x_radius - self.offset_x + row_offset;
        let center_y = y * self.tiling_height + self.y_radius - self.offset_y;

        let p1 = round_to_pixel_precision((center_x, center_y - self.y_radius)); // top
        let p2 = round_to_pixel_precision((center_x + self.x_radius, center_y - self.half_radius)); // top-right
        let p3 = round_to_pixel_precision((center_x + self.x_radius, center_y + self.half_radius)); // bottom-right
        let p4 = round_to_pixel_precision((center_x, center_y + self.y_radius)); // bottom
        let p5 = round_to_pixel_precision((center_x - self.x_radius, center_y + self.half_radius)); // bottom-left
        let p6 = round_to_pixel_precision((center_x - self.x_radius, center_y - self.half_radius)); // top-left

        let x_coordinates = &[p1.0, p2.0, p3.0, p4.0, p5.0, p6.0];
        let y_coordinates = &[p1.1, p2.1, p3.1, p4.1, p5.1, p6.1];
        let size = canvas.output_size()?;

        canvas.filled_polygon(x_coordinates, y_coordinates, color)
    }
}
//
// pub fn render_hex_indexed<T: RenderTarget>(
//     canvas: &Canvas<T>,
//     offset: (i16, i16),
//     index: (i16, i16),
//     // the distance from the middle point to a corner of the hex
//     radius: i16,
//     color: Color,
//     skip_offscreen: bool,
// ) -> Result<(), String> {
//     let (x_i, y_i) = index;
//     let (x, y) = (x_i as f32, y_i as f32);
//
//     let width = *SQRT_3 * (radius as f32);
//     // the distance from the middle point to the middle of an edge of the hex
//     let x_radius = width / 2f32;
//     let y_radius = radius as f32;
//     let r_half = y_radius / 2f32;
//
//     // every 2nd row needs to be shifted by half a hex for a continuous pattern
//     let row_offset = ((y_i % 2) as f32) * (width / 2f32);
//     let (offset_x, offset_y) = (offset.0 as f32, offset.1 as f32);
//
//     let center_x = (x * width) + row_offset + (width / 2f32) - offset_x;
//     let center_y = y * width * *TANGENT_LENGTH_FACTOR + y_radius - offset_y;
//
//     let p1 = round_to_pixel_precision((center_x, center_y - y_radius)); // top
//     let p2 = round_to_pixel_precision((center_x + x_radius, center_y - r_half)); // top-right
//     let p3 = round_to_pixel_precision((center_x + x_radius, center_y + r_half)); // bottom-right
//     let p4 = round_to_pixel_precision((center_x, center_y + y_radius)); // bottom
//     let p5 = round_to_pixel_precision((center_x - x_radius, center_y + r_half)); // bottom-left
//     let p6 = round_to_pixel_precision((center_x - x_radius, center_y - r_half)); // top-left
//
//     let x_coords = &[p1.0, p2.0, p3.0, p4.0, p5.0, p6.0];
//     let y_coords = &[p1.1, p2.1, p3.1, p4.1, p5.1, p6.1];
//     let size = canvas.output_size()?;
//     let is_x_completely_offscreen = x_coords
//         .iter()
//         .all(|&v| is_out_of_bounds(v, 0, size.0 as i16));
//     let is_y_completely_offscreen = y_coords
//         .iter()
//         .all(|&v| is_out_of_bounds(v, 0, size.1 as i16));
//
//     if skip_offscreen && (is_x_completely_offscreen || is_y_completely_offscreen) {
//         // TODO: Calculate which indexes to show outside the loop calling this function
//         //  for better performance
//         return Ok(());
//     }
//
//     canvas.filled_polygon(x_coords, y_coords, color)
// }

fn is_out_of_bounds(p: i16, min: i16, max: i16) -> bool {
    p < min || p > max
}

fn round_to_pixel_precision(p: (f32, f32)) -> (i16, i16) {
    (p.0.round() as i16, p.1.round() as i16)
}
