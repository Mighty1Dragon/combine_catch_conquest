use crate::map::HexMap;
use macroquad::prelude::*;
use crate::game::*;
use crate::ui::*;
use crate::figure::*;

mod game;
mod map;
mod figure;
mod ui;
mod r#move;

#[macroquad::main("combine catch and conquer")]
async fn main() {
    set_fullscreen(true);
    println!("Hello, world!");
    let mut h = HexMap::new("../maps/river");
    let mut g = Game::new( h , GameMode::Debug);
    let ui = UI::new().await;
    g.map.figures.insert((6,3), Figure::new(1, FigureType::Pawn));
    g.map.figures.insert((6,2), Figure::new(1, FigureType::King));
    g.map.figures.insert((6,6), Figure::new(2, FigureType::Pawn));
    g.map.figures.insert((6,7), Figure::new(2, FigureType::King));
    loop{
        //clear_background(RED);
        ui.select(&mut g);
        ui.draw(&g);
        next_frame().await
    }
    
}
