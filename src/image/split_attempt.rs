// Note(DR): I'll leave this as a half-finished attempt to export bigger images by first generating
// smaller blocks and joining them afterwards

// use crate::map::MapState;
// use crate::rendering::HexRenderer;
// use chrono::Utc;
// use image::io::Reader as ImageReader;
// use image::{GenericImage, ImageBuffer, Rgba};
// use sdl2::image::SaveSurface;
// use sdl2::pixels::PixelFormatEnum;
// use sdl2::render::Canvas;
// use sdl2::surface::Surface;
// use std::cmp::min;

// const MAX_IMAGE_DIMENSIONS: (u16, u16) = (8192, 8192);

// pub fn save_as_png(map_state: &MapState) -> Result<(), String> {
//     let renderer = HexRenderer::new(10);
//     let (width, height) = renderer.get_bounds(map_state.map_size);
//     println!(
//         "Exporting image with dimensions {:?}, this might take a while and the application might also be marked as not responding!",
//         renderer.get_bounds(map_state.map_size)
//     );
//
//     let block_width = min(MAX_IMAGE_DIMENSIONS.0, width) as usize;
//     let block_height = min(MAX_IMAGE_DIMENSIONS.1, height) as usize;
//
//     let image_id = format!("{}", Utc::now().timestamp());
//
//     // export partial images (saving one big image via SDL leads to OOM crashes)
//     for (i, x) in (0..width).step_by(block_width).enumerate() {
//         for (j, y) in (0..height).step_by(block_height).enumerate() {
//             let remaining_width = width as i32 - x as i32;
//             let remaining_height = height as i32 - y as i32;
//             if remaining_height <= 0 || remaining_width <= 0 {
//                 continue;
//             }
//             let sub_image_width = min(block_width as u32, remaining_width as u32);
//             let sub_image_height = min(block_height as u32, remaining_height as u32);
//             save_sub_image(
//                 &renderer,
//                 map_state,
//                 image_id.as_str(),
//                 (i, j),
//                 (x as i16, y as i16),
//                 sub_image_width,
//                 sub_image_height,
//             )?;
//             println!(
//                 "Exported sub-image ({i}x{j}) with dimensions {sub_image_width}x{sub_image_height}"
//             );
//         }
//     }
//
//     let mut whole_image = <ImageBuffer<Rgba<u8>, _>>::new(width as u32, height as u32);
//     // join images to one big image
//     for (i, x) in (0..width).step_by(block_width).enumerate() {
//         for (j, y) in (0..height).step_by(block_height).enumerate() {
//             let sub_image = ImageReader::open(sub_image_file_name(image_id.as_str(), (i, j)))
//                 .map_err(|e| e.to_string())?
//                 .decode()
//                 .map_err(|e| e.to_string())?;
//
//             whole_image
//                 .copy_from(&sub_image, x as u32, y as u32)
//                 .map_err(|e| e.to_string())?;
//
//             println!("Concatenated sub-image {i}x{j}");
//         }
//     }
//
//     whole_image
//         .save(format!("./{}.png", image_id))
//         .map_err(|e| e.to_string())?;
//
//     println!("Successfully saved image");
//     Ok(())
// }
//
// // renders & saves one (sub-)block of the image
// fn save_sub_image(
//     renderer: &HexRenderer,
//     map_state: &MapState,
//     image_id: &str,
//     index: (usize, usize),
//     offset: (i16, i16),
//     width: u32,
//     height: u32,
// ) -> Result<(), String> {
//     let pixel_format = PixelFormatEnum::RGBA8888;
//     let surface = Surface::new(width, height, pixel_format)?;
//     let canvas = Canvas::from_surface(surface)?;
//
//     renderer.render_map(&canvas, offset, &map_state, false)?;
//
//     let mut pixels = canvas.read_pixels(None, pixel_format)?;
//     let (width, height) = canvas.output_size()?;
//     let pitch = pixel_format.byte_size_of_pixels(width as usize);
//     let surface = Surface::from_data(
//         pixels.as_mut_slice(),
//         width,
//         height,
//         pitch as u32,
//         pixel_format,
//     )?;
//
//     surface.save(sub_image_file_name(image_id, index))?;
//     Ok(())
// }
//
// fn sub_image_file_name(image_id: &str, index: (usize, usize)) -> String {
//     format!("./{}_tmp_{}x{}.png", image_id, index.0, index.1)
// }
