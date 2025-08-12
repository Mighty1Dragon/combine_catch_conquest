use crate::map::HexMap;

mod game;
mod map;
mod figure;

fn main() {
    println!("Hello, world!");
    let h = HexMap::new("maps/river");
}
