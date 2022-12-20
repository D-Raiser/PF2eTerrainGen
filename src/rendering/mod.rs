use crate::map::MapState;
use once_cell::sync::Lazy;
use sdl2::gfx::primitives::DrawRenderer;
use sdl2::pixels::Color;
use sdl2::render::{Canvas, RenderTarget};
use std::cmp::{max, min};

static SQRT_3: Lazy<f32> = Lazy::new(|| 3f32.sqrt());
static TANGENT_LENGTH_FACTOR: Lazy<f32> = Lazy::new(|| *SQRT_3 / 2f32);

// to only compute numbers identical across all hexes once
pub struct HexRenderer {
    pub hex_width: f32,
    // the vertical distance between hexagons of two connected rows, NOT the true height of the hex
    pub tiling_height: f32,
    // the distance from the middle point to the middle of an edge of the hex
    x_radius: f32,
    pub y_radius: f32,
    // half the length of any of the sides of all six triangles that make up the hexagon
    half_radius: f32,
}

impl HexRenderer {
    pub fn new(hex_radius: i16) -> HexRenderer {
        let y_radius = hex_radius as f32;
        let width = *SQRT_3 * y_radius;
        let height = width * *TANGENT_LENGTH_FACTOR;
        let x_radius = width / 2f32;
        let r_half = y_radius / 2f32;

        HexRenderer {
            half_radius: r_half,
            hex_width: width,
            tiling_height: height,
            y_radius,
            x_radius,
        }
    }

    // returns the dimensions required to render a map of the provided size
    pub fn get_bounds(&self, map_size: (i16, i16)) -> (u16, u16) {
        let (x, y) = map_size;
        // every second row is horizontally offset by half a tile
        let total_width = self.hex_width * (x as f32) + 0.5 * self.hex_width;
        let total_height = self.tiling_height * (y as f32) + 0.5 * self.y_radius;

        (total_width.round() as u16, total_height.round() as u16)
    }

    pub fn render_map<T: RenderTarget>(
        &self,
        canvas: &Canvas<T>,
        viewport_offset: (i16, i16),
        map_state: &MapState,
        skip_offscreen: bool,
    ) -> Result<(), String> {
        let viewport_size = canvas.output_size()?;
        let ((min_idx_x, min_idx_y), (max_idx_x, max_idx_y)) = self.get_index_range(
            map_state.map_size,
            viewport_offset,
            viewport_size,
            skip_offscreen,
        );

        for x in min_idx_x..=max_idx_x {
            for y in min_idx_y..=max_idx_y {
                let hex = &map_state.map.tiles[x][y];
                self.render_hex_indexed(
                    &canvas,
                    (x as i16, y as i16),
                    viewport_offset,
                    hex.environment.color(),
                )?;
            }
        }

        Ok(())
    }

    fn render_hex_indexed<T: RenderTarget>(
        &self,
        canvas: &Canvas<T>,
        index: (i16, i16),
        viewport_offset: (i16, i16),
        // the distance from the middle point to a corner of the hex
        color: Color,
    ) -> Result<(), String> {
        let (x_i, y_i) = index;
        let (x, y) = (x_i as f32, y_i as f32);
        let (offset_x, offset_y) = (viewport_offset.0 as f32, viewport_offset.1 as f32);

        // every 2nd row needs to be shifted by half a hex for a continuous pattern
        let row_offset = ((y_i % 2) as f32) * self.x_radius;

        let center_x = x * self.hex_width + self.x_radius - offset_x + row_offset;
        let center_y = y * self.tiling_height + self.y_radius - offset_y;

        let p1 = round_to_pixel_precision((center_x, center_y - self.y_radius)); // top
        let p2 = round_to_pixel_precision((center_x + self.x_radius, center_y - self.half_radius)); // top-right
        let p3 = round_to_pixel_precision((center_x + self.x_radius, center_y + self.half_radius)); // bottom-right
        let p4 = round_to_pixel_precision((center_x, center_y + self.y_radius)); // bottom
        let p5 = round_to_pixel_precision((center_x - self.x_radius, center_y + self.half_radius)); // bottom-left
        let p6 = round_to_pixel_precision((center_x - self.x_radius, center_y - self.half_radius)); // top-left

        let x_coordinates = &[p1.0, p2.0, p3.0, p4.0, p5.0, p6.0];
        let y_coordinates = &[p1.1, p2.1, p3.1, p4.1, p5.1, p6.1];

        canvas.filled_polygon(x_coordinates, y_coordinates, color)
    }

    // returns minimum and maximum index of tiles to be rendered
    fn get_index_range(
        &self,
        map_size: (i16, i16),
        viewport_offset: (i16, i16),
        viewport_size: (u32, u32),
        skip_offscreen: bool,
    ) -> ((usize, usize), (usize, usize)) {
        let (mut min_idx_x, mut min_idx_y) = (0 as usize, 0 as usize);
        let (mut max_idx_x, mut max_idx_y) = (map_size.0 as usize - 1, map_size.1 as usize - 1);

        if skip_offscreen {
            // pre-calculate which tiles will be visible to skip anything other than these tiles
            // for better performance
            let (min_x, min_y) = (viewport_offset.0 as f32, viewport_offset.1 as f32);
            let (max_x, max_y) = (
                min_x + viewport_size.0 as f32,
                min_y + viewport_size.1 as f32,
            );

            let padding = 1f32;
            // subtract/add 1 to the min/max just to be sure that enough is always rendered to not have
            // any clipping at the screen borders
            min_idx_x = max(
                (((min_x / self.hex_width) - 0.5).round() - padding) as usize,
                0,
            );
            min_idx_y = max(
                (((min_y - self.y_radius) / self.tiling_height).round() - padding) as usize,
                0,
            );
            max_idx_x = min(
                (((max_x / self.hex_width) - 0.5).round() + padding) as usize,
                map_size.0 as usize - 1,
            );
            max_idx_y = min(
                (((max_y - self.y_radius) / self.tiling_height).round() + padding) as usize,
                map_size.1 as usize - 1,
            );
        }

        ((min_idx_x, min_idx_y), (max_idx_x, max_idx_y))
    }
}

fn round_to_pixel_precision(p: (f32, f32)) -> (i16, i16) {
    (p.0.round() as i16, p.1.round() as i16)
}
