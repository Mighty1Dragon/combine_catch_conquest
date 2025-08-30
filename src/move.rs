use crate::map::*;
use crate::game::*;
use crate::figure::*;
use crate::MoveType::*;

pub fn calc_possible_moves( pos: (usize, usize), game: &mut Game){
    
        let mut possible = Vec::new();
        let mut moves = Vec::new();
        if let None = game.map.figures.get(&pos){
            return;
        }
        let my_fig = game.map.figures.get(&pos).unwrap();
        let my_field = game.map.map.get(pos.0 + pos.1 * game.map.x_map_bound);

        match my_fig.typ{

            FigureType::Pawn => {
                calc_king_move(pos.0, pos.1, &mut possible, &mut moves, &game);
            }
            FigureType::King => {
                calc_king_move(pos.0, pos.1, &mut possible, &mut moves, &game);
            }
        }
        game.possible = possible;
        game.move_types = moves;

}

pub fn calc_king_move(x: usize, y: usize, possible: &mut Vec<(usize,usize)>, moves: &mut Vec<MoveType>,  game: &Game){
    for i in 0..6 {
        match try_direction(x, y, i, &game) {
            Some(target) => {
                if not_same_color(&(x, y), &target, &game){

                    possible.push(target);
                    if has_figure(&target, &game) {
                        moves.push(Melee);
                    }else{
                        moves.push(Movement);
                    }
                }
            },
            None => {},
        }
    }
}

pub fn not_same_color(pos1: &(usize, usize), pos2: &(usize, usize), game: &Game) -> bool{
    let color1 = game.map.figures.get(pos1).unwrap().color;
    if game.map.figures.contains_key(pos2) {
        return color1 != game.map.figures.get(pos2).unwrap().color;
    }else{
        return true;
    }
}

pub fn has_figure(pos: &(usize, usize), game: &Game) -> bool {
    game.map.figures.contains_key(pos)
}

pub fn try_direction(x: usize, y: usize, direction: u8, game: &Game) -> Option<(usize, usize)>{
    match direction {
        0 => return game.map.up(x, y),
        1 => return game.map.upper_right(x, y),
        2 => return game.map.bottom_right(x, y),
        3 => return game.map.bottom(x, y),
        4 => return game.map.bottom_left(x, y),
        5 => return game.map.upper_left(x, y),
        _ => panic!("direction doesn't exist")
    }

}
