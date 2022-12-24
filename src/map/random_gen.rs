use crate::map::{Environment, Map, MapGenerator};
use rand::{thread_rng, Rng};
use std::collections::HashMap;

use once_cell::sync::OnceCell;
use std::slice::Iter;
use std::sync::{Arc, RwLock};

pub struct RandomGenerator {}

#[derive(Copy, Clone, Eq, Hash, PartialEq)]
struct RandomGeneratorHexType {
    environment: Environment,
    // chance to generate a tile of this type in percent
    base_chance: u16,
    // chance in percent to transform into this `HexType` based on the tile's current `HexType` when
    // smoothing the map
    transform_chance: fn(Environment) -> u16,
}

impl MapGenerator for RandomGenerator {
    fn populate(&self, map: Arc<RwLock<Map>>, dimensions: (u16, u16)) {
        let (max_x, max_y) = ((dimensions.0 - 1) as usize, (dimensions.1 - 1) as usize);

        for y in 0..=max_y {
            for x in 0..=max_x {
                RandomGenerator::generate_hex(map.clone(), x, y, max_x, max_y)
            }
        }
    }

    fn smooth(&self, map: Arc<RwLock<Map>>, dimensions: (u16, u16)) {
        let mut rng = rand::thread_rng();

        let (max_x, max_y) = ((dimensions.0 - 1) as usize, (dimensions.1 - 1) as usize);

        for y in 0..=max_y {
            for x in 0..=max_x {
                let op = rng.gen_range(0..100);
                if op < 45 {
                    // do nothing
                    continue;
                } else if op < 55 {
                    // transform the hex
                    RandomGenerator::set_hex(
                        map.clone(),
                        RandomGenerator::transform_hex(RandomGenerator::read_hex(
                            map.clone(),
                            x,
                            y,
                        )),
                        x,
                        y,
                    );
                } else {
                    // determine tile's type by averaging the surroundings
                    let y_above = if y > 0 { y - 1 } else { max_y };
                    let y_below = if y < max_y { y + 1 } else { 0 };
                    let x_left = if x > 0 { x - 1 } else { max_x };
                    let x_right = if x < max_x { x + 1 } else { 0 };

                    let surrounding_environments: [Environment; 8] = [
                        RandomGenerator::read_hex(map.clone(), x_left, y_above),
                        RandomGenerator::read_hex(map.clone(), x, y_above),
                        RandomGenerator::read_hex(map.clone(), x_right, y_above),
                        RandomGenerator::read_hex(map.clone(), x_left, y),
                        RandomGenerator::read_hex(map.clone(), x_right, y),
                        RandomGenerator::read_hex(map.clone(), x_left, y_below),
                        RandomGenerator::read_hex(map.clone(), x, y_below),
                        RandomGenerator::read_hex(map.clone(), x_right, y_below),
                    ];

                    let most_frequent_environment = surrounding_environments
                        .into_iter()
                        .fold(HashMap::<Environment, usize>::new(), |mut map, env| {
                            *map.entry(env).or_default() += 1;
                            map
                        })
                        .into_iter()
                        .max_by_key(|(_, cnt)| *cnt)
                        .map(|(k, _)| k)
                        .unwrap();
                    RandomGenerator::set_hex(map.clone(), most_frequent_environment, x, y);
                }
            }
        }
    }
}

impl RandomGenerator {
    fn generate_hex(map: Arc<RwLock<Map>>, x: usize, y: usize, _max_x: usize, _max_y: usize) {
        let n = thread_rng().gen_range(0..RandomGeneratorHexType::summed_terrain_base_chance());
        let (_, hex_type) = RandomGeneratorHexType::iterator().fold(
            (0, RandomGeneratorHexType::NONE),
            |(summed_percentage, hex_type), &t| {
                if summed_percentage > n {
                    (summed_percentage, hex_type)
                } else {
                    (summed_percentage + t.base_chance, t)
                }
            },
        );

        RandomGenerator::set_hex(map.clone(), hex_type.environment, x, y);
    }

    fn set_hex(map: Arc<RwLock<Map>>, env: Environment, x: usize, y: usize) {
        let mut map = match map.write() {
            Ok(m) => m,
            Err(e) => {
                println!("failed to set hex: {e}");
                return;
            }
        };

        map.tiles[y][x].environment = env;
    }

    fn read_hex(map: Arc<RwLock<Map>>, x: usize, y: usize) -> Environment {
        let map = match map.read() {
            Ok(m) => m,
            Err(e) => {
                println!("Failed to read hex: {e}");
                return Environment::NONE;
            }
        };

        map.tiles[y][x].environment
    }

    fn transform_hex(environment: Environment) -> Environment {
        let total_chance = RandomGeneratorHexType::total_transform_chance(environment);
        if total_chance == 0 {
            return environment;
        }

        let n = thread_rng().gen_range(0..total_chance);
        let (_, ret) = RandomGeneratorHexType::iterator().fold(
            (0, RandomGeneratorHexType::NONE),
            |(summed_percentage, ret), &t| {
                if summed_percentage > n {
                    (summed_percentage, ret)
                } else {
                    (summed_percentage + (t.transform_chance)(environment), t)
                }
            },
        );
        ret.environment
    }
}

impl RandomGeneratorHexType {
    const NONE: RandomGeneratorHexType = RandomGeneratorHexType {
        environment: Environment::NONE,
        // never generate this
        base_chance: 0,
        transform_chance: |_| 0,
    };

    const AQUATIC: RandomGeneratorHexType = RandomGeneratorHexType {
        environment: Environment::AQUATIC,
        // starting out with 70% (which could be considered realistic) leads to water
        // spreading even further during smoothing since it's the most prevalent element already
        base_chance: 14,
        transform_chance: |_| 0,
    };
    const ARCTIC: RandomGeneratorHexType = RandomGeneratorHexType {
        environment: Environment::ARCTIC,
        base_chance: 0,
        transform_chance: |t| if t == Environment::AQUATIC { 5 } else { 0 },
    };
    const DESERT: RandomGeneratorHexType = RandomGeneratorHexType {
        environment: Environment::DESERT,
        base_chance: 4,
        transform_chance: |t| if t == Environment::PLAINS { 5 } else { 0 },
    };
    const FOREST: RandomGeneratorHexType = RandomGeneratorHexType {
        environment: Environment::FOREST,
        base_chance: 12,
        transform_chance: |_t| 0,
    };
    const MOUNTAIN: RandomGeneratorHexType = RandomGeneratorHexType {
        environment: Environment::MOUNTAIN,
        base_chance: 6,
        transform_chance: |_| 0,
    };
    const PLAINS: RandomGeneratorHexType = RandomGeneratorHexType {
        environment: Environment::PLAINS,
        base_chance: 8,
        transform_chance: |_| 0,
    };
    const SWAMP: RandomGeneratorHexType = RandomGeneratorHexType {
        environment: Environment::SWAMP,
        base_chance: 0,
        transform_chance: |t| {
            if t == Environment::FOREST || t == Environment::PLAINS {
                5
            } else {
                0
            }
        },
    };
    const AERIAL: RandomGeneratorHexType = RandomGeneratorHexType {
        environment: Environment::AERIAL,
        base_chance: 0,
        transform_chance: |t| if t == Environment::MOUNTAIN { 3 } else { 0 },
    };
    const GLACIER: RandomGeneratorHexType = RandomGeneratorHexType {
        environment: Environment::GLACIER,
        base_chance: 0,
        // requires at least 2 smoothing iterations, since `ARCTIC` isn't generated by default
        transform_chance: |t| if t == Environment::ARCTIC { 40 } else { 0 },
    };
    const VOLCANIC: RandomGeneratorHexType = RandomGeneratorHexType {
        environment: Environment::VOLCANIC,
        base_chance: 0,
        transform_chance: |t| if t == Environment::MOUNTAIN { 10 } else { 0 },
    };

    fn iterator() -> Iter<'static, RandomGeneratorHexType> {
        static HEX_TYPES: [RandomGeneratorHexType; 11] = [
            RandomGeneratorHexType::NONE,
            RandomGeneratorHexType::AQUATIC,
            RandomGeneratorHexType::ARCTIC,
            RandomGeneratorHexType::DESERT,
            RandomGeneratorHexType::FOREST,
            RandomGeneratorHexType::MOUNTAIN,
            RandomGeneratorHexType::PLAINS,
            RandomGeneratorHexType::SWAMP,
            RandomGeneratorHexType::AERIAL,
            RandomGeneratorHexType::GLACIER,
            RandomGeneratorHexType::VOLCANIC,
        ];
        HEX_TYPES.iter()
    }

    fn total_transform_chance(environment: Environment) -> u16 {
        RandomGeneratorHexType::iterator()
            .map(|&t| (t.transform_chance)(environment))
            .sum()
    }

    fn summed_terrain_base_chance() -> u16 {
        static INSTANCE: OnceCell<u16> = OnceCell::new();
        *INSTANCE.get_or_init(|| {
            RandomGeneratorHexType::iterator()
                .map(|t| t.base_chance)
                .sum()
        })
    }
}
