
use crate::map::HexMap;
use crate::figure::Player;

#[derive(Debug)]
pub enum GameMode{
    TwoPlayer,
    Party
}

#[derive(Debug)]
pub struct Game {
    pub map: HexMap,
    player: Vec<Player>,
    gamemode: GameMode
}
