use crate::map::MapState;
use crate::viewport::ViewPortState;

pub struct AppState {
    pub viewport_state: ViewPortState,
    pub map_state: MapState,
}
