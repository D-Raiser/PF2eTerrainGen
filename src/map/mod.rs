mod environments;
mod procedural_gen;
mod random_gen;

use crate::map::environments::Environment;
use crate::map::procedural_gen::ProceduralGenerator;
use crate::map::random_gen::RandomGenerator;

trait MapGenerator {
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
    //const GENERATOR: RandomGenerator = RandomGenerator {};
    const GENERATOR: ProceduralGenerator = ProceduralGenerator {};

    fn create_empty_map(dimensions: (i16, i16)) -> Result<Map, String> {
        let (width, height) = dimensions;
        if height % 2 != 0 || width % 2 != 0 || height < 2 || width < 2 {
            // With uneven numbers the map cannot be tiled infinitely without gaps
            return Err(String::from("Map dimensions must be even positive numbers"));
        }

        Ok(Map {
            tiles: vec![
                vec![
                    Hex {
                        environment: Environment::NONE
                    };
                    width as usize
                ];
                height as usize
            ],
        })
    }

    pub fn new(dimensions: (i16, i16), iterations: u16) -> Result<MapState, String> {
        let mut map = MapState::create_empty_map(dimensions)?;

        MapState::GENERATOR.populate(&mut map);
        for _ in 0..iterations {
            MapState::GENERATOR.smooth(&mut map);
        }

        Ok(MapState {
            map,
            map_size: dimensions,
            iterations,
        })
    }

    pub fn regenerate_map(&mut self) -> Result<(), String> {
        self.map = MapState::new(self.map_size, self.iterations)?.map;
        Ok(())
    }
}
