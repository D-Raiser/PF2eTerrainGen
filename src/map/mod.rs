mod environments;
mod procedural_gen;
mod random_gen;

use crate::map::environments::Environment;
use crate::map::procedural_gen::ProceduralGenerator;
use std::sync::{Arc, RwLock};
use std::thread;

trait MapGenerator {
    fn populate(&self, map: Arc<RwLock<Map>>, dimensions: (u16, u16));
    fn smooth(&self, map: Arc<RwLock<Map>>, dimensions: (u16, u16));
}

pub struct MapState {
    pub map: Arc<RwLock<Map>>,
    pub map_size: (u16, u16),
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

    fn create_empty_map(dimensions: (u16, u16)) -> Result<Map, String> {
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

    pub fn new(dimensions: (u16, u16), iterations: u16) -> Result<MapState, String> {
        let mut state = MapState {
            map: Arc::new(RwLock::new(MapState::create_empty_map(dimensions)?)),
            map_size: dimensions,
            iterations,
        };
        state.generate_map()?;
        Ok(state)
    }

    pub fn generate_map(&mut self) -> Result<(), String> {
        let mut map = self.map.write().map_err(|e| e.to_string())?;
        *map = MapState::create_empty_map(self.map_size)?;

        let local_self = self.map.clone();
        let iterations = self.iterations;
        let dimensions = self.map_size;

        thread::spawn(move || MapState::generate(local_self, iterations, dimensions));

        Ok(())
    }

    fn generate(map: Arc<RwLock<Map>>, iterations: u16, dimensions: (u16, u16)) {
        MapState::GENERATOR.populate(map.clone(), dimensions);
        for _ in 0..iterations {
            MapState::GENERATOR.smooth(map.clone(), dimensions);
        }
    }
}
