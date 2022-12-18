use sdl2::event::Event;
use sdl2::mouse::MouseWheelDirection;
use std::cmp::{max, min};

const DEFAULT_ZOOM_LEVEL: i16 = 40;
const MIN_ZOOM_LEVEL: i16 = 10;
const MAX_ZOOM_LEVEL: i16 = 100;

pub struct ViewPortState {
    // offset from (0,0) to the the pixel that is currently supposed to be displayed in the top-left corner
    pub offset: (i16, i16),
    // effectively the radius of a single hex
    pub zoom_level: i16,
}

impl ViewPortState {
    pub fn new() -> ViewPortState {
        ViewPortState {
            offset: (0, 0),
            zoom_level: DEFAULT_ZOOM_LEVEL,
        }
    }

    pub fn handle_events(&mut self, event: Event) {
        match event {
            Event::MouseMotion {
                mousestate,
                xrel,
                yrel,
                ..
            } => {
                if !mousestate.left() {
                    return;
                }
                self.offset = (self.offset.0 - (xrel as i16), self.offset.1 - (yrel as i16));
            }
            Event::MouseWheel { y, direction, .. } => {
                let dir = 2 * if direction == MouseWheelDirection::Normal {
                    1i16
                } else {
                    -1i16
                };
                self.zoom_level = max(
                    min(self.zoom_level + dir * (y as i16), MAX_ZOOM_LEVEL),
                    MIN_ZOOM_LEVEL,
                );
            }
            _ => {}
        }
    }
}
