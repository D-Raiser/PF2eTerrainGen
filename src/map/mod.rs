mod environments;
mod random_gen;

use crate::map::environments::Environment;
use crate::map::random_gen::RandomGenerator;

trait MapGenerator {
    fn generate(&self, dimensions: (i16, i16), iterations: u16) -> Result<Map, String>;
    fn populate(&self, map: &mut Map);
    fn smooth(&self, map: &mut Map);
}

pub struct MapState {
    pub map: Map,
    pub map_size: (i16, i16),
    pub iterations: u16,
}

pub struct Map {
    pub tiles: Vec<Vec<Hex>>,
}

#[derive(Copy, Clone, Eq, Hash, PartialEq)]
pub struct Hex {
    pub environment: Environment,
}

impl MapState {
    const GENERATOR: RandomGenerator = RandomGenerator {};

    pub fn new(width: i16, height: i16, iterations: u16) -> Result<MapState, String> {
        let map = MapState::GENERATOR.generate((width, height), iterations)?;
        Ok(MapState {
            map,
            map_size: (width, height),
            iterations,
        })
    }

    pub fn regenerate_map(&mut self) -> Result<(), String> {
        self.map = MapState::GENERATOR.generate(self.map_size, self.iterations)?;
        Ok(())
    }
}
