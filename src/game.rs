
use crate::map::HexMap;
use crate::figure::Player;
use crate::ui::*;

#[derive(Debug)]
pub enum GameMode{
    Debug,
    TwoPlayer,
    Party
}

#[derive(Debug)]
pub struct Game {
    pub map: HexMap,
    pub player: Vec<Player>,
    pub gamemode: GameMode,
    pub selected: Option<(usize, usize)>,
    pub possible: Vec<(usize,usize)>,
}
pub fn move_figure( start: (usize, usize), target: (usize, usize), game: &mut Game){
    if game.possible.contains(&target){
        let fig = game.map.figures.remove(&start);
        game.map.figures.insert(target, fig.expect("selected field not allowed"));
    }
}

impl Game {

    pub fn new(map: HexMap, gamemode: GameMode) -> Game{
        Game{
            map, player: Vec::new(), gamemode, selected: None, possible: Vec::new()
        }
    }

    
}
