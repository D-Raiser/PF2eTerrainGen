use sdl2::pixels::Color;

#[derive(Copy, Clone, Eq, Hash, PartialEq, Debug)]
pub enum Environment {
    NONE,
    // Common Environments
    AQUATIC,
    ARCTIC,
    DESERT,
    FOREST,
    MOUNTAIN,
    PLAINS,
    // TODO: Urban ?
    SWAMP,
    // Extreme Environments
    AERIAL,
    GLACIER,
    VOLCANIC,
    // TODO: Undersea?
    // TODO: Underground?
}

impl Environment {
    pub fn color(self) -> Color {
        match self {
            Environment::NONE => Color::RGB(40, 40, 40),
            Environment::AQUATIC => Color::RGB(0, 130, 220),
            Environment::ARCTIC => Color::RGB(145, 230, 230),
            Environment::DESERT => Color::RGB(230, 230, 30),
            Environment::FOREST => Color::RGB(10, 105, 15),
            Environment::MOUNTAIN => Color::RGB(88, 97, 96),
            Environment::PLAINS => Color::RGB(0, 205, 12),
            Environment::SWAMP => Color::RGB(50, 80, 10),
            Environment::AERIAL => Color::RGB(202, 216, 214),
            Environment::GLACIER => Color::RGB(216, 255, 255),
            Environment::VOLCANIC => Color::RGB(154, 5, 3),
        }
    }
}
