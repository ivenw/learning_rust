use rand::{self, Rng};
use std::fmt::{Display, Formatter, Result};

struct Num(u8);

impl Num {
    fn new() -> Self {
        Self(rand::thread_rng().gen_range(1..=10))
    }
}

impl Display for Num {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self)
    }
}
enum Tile {
    Player,
    Num,
    Empty,
}

struct TileMap {
    width: u16,
    height: u16,
    map: Vec<Vec<Tile>>,
}
impl TileMap {
    fn new(width: u16, height: u16) -> Self {
        let map = (0..height)
            .into_iter()
            .map(|_| (0..width).into_iter().map(|_| Tile::Num).collect())
            .collect();
        Self { width, height, map }
    }
}

impl Display for TileMap {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let mut buffer = String::new();
        for line in self.map.iter() {
            buffer.push_str("\n");
            for tile in line.iter() {
                buffer.push_str(tile.to_string().as_str());
            }
        }
        write!(f, "{}", buffer)
    }
}

fn main() {
    let map = TileMap::new(16, 8);
    println!("{}", map)
}
