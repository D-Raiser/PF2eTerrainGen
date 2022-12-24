use crate::map::environments::Environment;
use crate::map::{Map, MapGenerator};
use rand::Rng;
use std::cmp::{max, min};

pub struct ProceduralGenerator {}

// TODO: doesn't feel much better than the random generator yet
//  probably needs some smoothing for bigger clusters of the same environment
impl MapGenerator for ProceduralGenerator {
    fn populate(&self, map: &mut Map) {
        let max_y = map.tiles.len() - 1;
        let max_x = map.tiles[0].len() - 1;

        println!("{max_x} {max_y}");

        for y in 0..=max_y {
            for x in 0..=max_x {
                ProceduralGenerator::generate_hex(map, x, y, max_x, max_y)
            }
        }
    }

    fn smooth(&self, _map: &mut Map) {
        todo!() //TODO: Not sure yet if this needs to do anything
    }
}

impl ProceduralGenerator {
    fn generate_hex(map: &mut Map, x: usize, y: usize, max_x: usize, max_y: usize) {
        let water_odds: u32 = if ProceduralGenerator::is_mostly_land(
            ProceduralGenerator::surrounding_environments(map, x, y, max_x, max_y),
        ) {
            4
        } else {
            7
        };

        let polar_distance = min(y, (max_y - y));
        // only the top/bottom ~12% are covered in ice (so 24% total at most)
        let close_to_pole = polar_distance < (max_y / 7);
        println!("{close_to_pole} + {water_odds}");

        if rand::thread_rng().gen_ratio(water_odds + if close_to_pole { 3 } else { 1 }, 10) {
            println!("WATER");
            ProceduralGenerator::generate_water(map, x, y, max_y);
        } else {
            println!("LAND");
            ProceduralGenerator::generate_land(map, x, y, max_x, max_y);
        }
    }

    fn generate_water(map: &mut Map, x: usize, y: usize, max_y: usize) {
        let polar_distance = min(y as u32, (max_y - y) as u32);
        // only the top/bottom ~12% are covered in ice (so 24% total at most)
        let max_dist_for_ice = (max_y / 8) as u32;

        if polar_distance >= max_dist_for_ice {
            return ProceduralGenerator::set_hex(map, Environment::AQUATIC, x, y);
        }

        // (8/10) * ((max_dist_for_ice-polar_distance) / max_dist_for_ice)
        // so up to 80% chance for ice the closer to the poles we get
        if !rand::thread_rng().gen_ratio(
            8 * (max_dist_for_ice - polar_distance),
            10 * max_dist_for_ice,
        ) {
            return ProceduralGenerator::set_hex(map, Environment::AQUATIC, x, y);
        }

        // up to 20% chance for glaciers the closer to the poles we get
        if !rand::thread_rng().gen_ratio(
            2 * (max_dist_for_ice - polar_distance),
            10 * max_dist_for_ice,
        ) {
            return ProceduralGenerator::set_hex(map, Environment::GLACIER, x, y);
        }
        return ProceduralGenerator::set_hex(map, Environment::ARCTIC, x, y);
    }

    fn generate_land(map: &mut Map, x: usize, y: usize, max_x: usize, max_y: usize) {
        let equatorial_distance = (max_y / 2).abs_diff(y) as u32;

        // only the middle ~25% can generate deserts
        let max_dist_for_desert = (max_y / 8) as u32;

        // up to 30% chance for deserts the closer to the equator we get
        if max_dist_for_desert > equatorial_distance
            && rand::thread_rng().gen_ratio(
                30 * (max_dist_for_desert - equatorial_distance),
                100 * max_dist_for_desert,
            )
        {
            return ProceduralGenerator::set_hex(map, Environment::DESERT, x, y);
        }

        let surroundings = ProceduralGenerator::surrounding_environments(map, x, y, max_x, max_y);
        let aerial_count = ProceduralGenerator::count_in_surroundings(
            &surroundings,
            &mut [Environment::AERIAL].iter(),
        );
        // make it less likely for more aerial terrain the more there already is in the surrounding area
        // maximum 5% if nothing around it is aerial
        if aerial_count <= 2 && rand::thread_rng().gen_ratio(max(0, 3 - aerial_count), 100) {
            return ProceduralGenerator::set_hex(map, Environment::AERIAL, x, y);
        }

        // zones that have higher potential for volcanic activity
        let volcanic_zone_count = ProceduralGenerator::count_in_surroundings(
            &surroundings,
            &mut [
                Environment::MOUNTAIN,
                Environment::DESERT,
                Environment::VOLCANIC,
            ]
            .iter(),
        );
        if rand::thread_rng().gen_ratio(max(0, 3 + 2 * volcanic_zone_count), 100) {
            return ProceduralGenerator::set_hex(map, Environment::VOLCANIC, x, y);
        }

        // up to 40% chance for plains the further away we are from the equator (since
        // equatorial_distance can at most be max_y/2)
        if rand::thread_rng().gen_ratio(4 * equatorial_distance, 5 * max_y as u32) {
            return ProceduralGenerator::set_hex(map, Environment::PLAINS, x, y);
        }

        // 45% chance for forests
        if rand::thread_rng().gen_ratio(9, 20) {
            return ProceduralGenerator::set_hex(map, Environment::FOREST, x, y);
        }

        // 65% chance for mountains
        if rand::thread_rng().gen_ratio(13, 20) {
            return ProceduralGenerator::set_hex(map, Environment::MOUNTAIN, x, y);
        }

        return ProceduralGenerator::set_hex(map, Environment::SWAMP, x, y);
    }

    fn count_in_surroundings<'a, I>(surroundings: &Vec<Environment>, envs: &mut I) -> u32
    where
        I: Iterator<Item = &'a Environment>,
    {
        surroundings.iter().fold(0, |cnt, &env| {
            if envs.any(|&e| e == env) {
                cnt + 1
            } else {
                cnt
            }
        })
    }

    fn set_hex(map: &mut Map, env: Environment, x: usize, y: usize) {
        map.tiles[y][x].environment = env;
    }

    fn is_mostly_land(surroundings: Vec<Environment>) -> bool {
        let cnt = ProceduralGenerator::count_in_surroundings(
            &surroundings,
            &mut [
                Environment::AQUATIC,
                Environment::ARCTIC,
                Environment::GLACIER,
            ]
            .iter(),
        );
        let none_cnt = ProceduralGenerator::count_in_surroundings(
            &surroundings,
            &mut [Environment::NONE].iter(),
        );
        let size = surroundings.len() as u32;
        println!("{:?}", surroundings);
        return (cnt) * 2 < (size - none_cnt);
    }

    // assumes that iterating over the map left->right first and top->bottom second
    fn surrounding_environments(
        map: &mut Map,
        x: usize,
        y: usize,
        max_x: usize,
        _max_y: usize,
    ) -> Vec<Environment> {
        let mut surrounding_environments: Vec<Environment> = vec![];

        if y > 0 {
            surrounding_environments.push(map.tiles[y - 1][x].environment);
            if x > 0 {
                surrounding_environments.push(map.tiles[y - 1][x - 1].environment);
            }
            if x < max_x {
                surrounding_environments.push(map.tiles[y - 1][x + 1].environment);
            }
        }
        if x > 0 {
            let _ = &map.tiles[y];
            let _ = &map.tiles[y][x - 1];
            surrounding_environments.push(map.tiles[y][x - 1].environment);
        }

        surrounding_environments
    }
}
