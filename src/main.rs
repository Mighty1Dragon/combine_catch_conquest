use crate::map::HexMap;
use macroquad::prelude::*;
use crate::game::*;
use crate::ui::*;
use crate::figure::*;

mod game;
mod map;
mod figure;
mod ui;

#[macroquad::main("combine catch and conquer")]
async fn main() {
    set_fullscreen(true);
    println!("Hello, world!");
    let mut h = HexMap::new("../maps/river");
    let mut g = Game::new( h , GameMode::Debug);
    let ui = UI::new().await;
    g.map.figures.insert((6,3), Figure::new(Player::Player(0), FigureType::Pawn));
    loop{
        //clear_background(RED);
        ui.draw(&g);
        next_frame().await
    }
    
}
