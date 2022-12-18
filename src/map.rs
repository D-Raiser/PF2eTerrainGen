use rand::Rng;

pub struct Map {
    pub tiles: Vec<Vec<Hex>>,
}

#[derive(Clone)]
pub struct Hex {
    pub hex_type: HexType,
}

impl Hex {
    const NONE_HEX: Hex = Hex {
        hex_type: HexType::None,
    };
}

#[derive(Clone)]
pub enum HexType {
    None,
    Water,
    Forest,
}

impl Map {
    pub fn generate(height: i16, width: i16) -> Result<Map, String> {
        if height % 2 != 0 || width % 2 != 0 || height < 2 || width < 2 {
            return Err(String::from("Map dimensions must be even positive numbers"));
        }

        let tiles: Vec<Vec<Hex>> = vec![vec![Hex::NONE_HEX; width as usize]; height as usize];
        let iterations = 3;
        let mut map = Map { tiles }.populate();

        for _ in 1..iterations {
            map = map.smooth()
        }

        Ok(map)
    }

    fn populate(self) -> Map {
        self.populate_randomly()
    }

    fn populate_randomly(mut self) -> Map {
        let mut rng = rand::thread_rng();

        for (y, row) in self.tiles.iter_mut().enumerate() {
            for (x, hex) in row.iter_mut().enumerate() {
                *hex = Hex {
                    hex_type: if rng.gen_range(0..100) > 50 {
                        HexType::Water
                    } else {
                        HexType::Forest
                    },
                }
            }
        }
        self
    }

    fn smooth(self) -> Map {
        self
    }
}

// 0,0 - 1,0 - 0,1 - 2,0 - 1,2 - 0,2
