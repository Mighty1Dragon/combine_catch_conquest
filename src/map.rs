use crate::figure::{Figure, FigureType, Player};
use crate::game::Game;
use std::collections::HashMap;
use std::env;
use std::fs;

#[derive(Debug)]
pub enum Terrain {
    Plain,
    Water,
    Forrest,
    Desert,
    Hill,
    Mountain

}

#[derive(Debug)]
pub enum Modifier {
    Wall,
    Mine,
    None
    //Stall
}

#[derive(Debug)]
struct Field {

    terrain: Terrain,
    modi: Modifier,
    player: Player,

}

#[derive(Debug)]
pub struct HexMap {
    pub map: Vec<Field>,
    pub x_map_bound: usize,
    pub y_map_bound: usize,
    pub figures: HashMap<(usize,usize), Figure>,
}

impl HexMap {
    pub fn new(map: &str) -> HexMap {

        let file = fs::read_to_string(map).expect("upsi file");
        let mut newmap = Vec::new();

        for line in file.lines(){

            for digit in line.split_whitespace(){

                let terrain = match digit.parse().unwrap(){
                    0 => Terrain::Plain,
                    1 => Terrain::Water,
                    _ => Terrain::Water
                };

                newmap.push(Field{
                     terrain,
                     modi: Modifier::None,
                     player: Player::None
                });
            }
        }

        HexMap { map: newmap,x_map_bound: 14, y_map_bound: 14, figures: HashMap::new() }
    }

    pub fn get_possible_moves(&self, pos: (usize, usize)) -> Vec<(usize, usize)>{

        let mut possible = Vec::new();

        match self.figures.get(&pos).unwrap().typ{

            FigureType::Pawn => {
                self.king_move(pos.0, pos.1, &mut possible);
            }
            FigureType::King => {
                self.king_move(pos.0, pos.1, &mut possible);
            }
        }
        possible
    }

    pub fn king_move(&self, x: usize, y: usize, possible: &mut Vec<(usize,usize)>){
        let mut options = Vec::new();
        options.push(self.up(x, y));
        options.push(self.upper_left(x, y));
        options.push(self.upper_right(x, y));
        options.push(self.bottom(x, y));
        options.push(self.bottom_left(x, y));
        options.push(self.bottom_right(x, y));
        
        for i in options.into_iter().flatten() {
           possible.push(i);
        }
    }

    pub fn bottom (&self, x: usize, y: usize) -> Option<(usize, usize)>{
        if y < self.y_map_bound {
            Some((x, y +1))
        }else{
            None
        }
    }

    pub fn bottom_right(&self, x: usize, y: usize) -> Option<(usize, usize)>{
        if x %2 == 0{
            if y < self.y_map_bound && x < self.x_map_bound{
                return Some((x+1, y));
            }
            return None;
        }
        if y <= self.y_map_bound && x < self.x_map_bound{
            return Some((x+1,y+1));
        }
        None
    }
 
    pub fn bottom_left(&self, x: usize, y: usize) -> Option<(usize, usize)>{
        if x %2 == 0{
            if y < self.y_map_bound && x > 0{
                return Some((x-1, y));
            }
            return None;
        }
        if y <= self.y_map_bound && x > 0{
            return Some((x-1,y+1));
        }
        None
    }   

    pub fn up(&self, x: usize, y: usize) -> Option<(usize,usize)>{
        if y > 0 {
            Some((x, y-1))
        }else{
            None
        }
    }

    pub fn upper_left(&self, x: usize, y: usize) -> Option<(usize,usize)>{
        if x %2 == 0{
            if y > 0 && x > 0{
                return Some((x-1, y-1));
            }
            return None;
        }
        if x > 0{
            return Some((x-1,y));
        }
        None
    }

    pub fn upper_right(&self, x: usize, y: usize) -> Option<(usize, usize)>{
        if x %2 == 0{
            if y > 0 && x < self.x_map_bound{
                return Some((x+1, y-1));
            }
            return None;
        }
        if x< self.x_map_bound{
            return Some((x+1,y));
        }
        None
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn type_test(){

        let h = HexMap::new("maps/river");
        let mut vec = Vec::new();
        h.king_move(1,0,&mut vec);
        //assert!(vec.len() == 5);
        assert!(vec.contains(&(0,0)));
        assert!(vec.contains(&(2,0)));
        assert!(vec.contains(&(0,1)));
        assert!(vec.contains(&(1,1)));
        assert!(vec.contains(&(2,1)));

        let mut vec2 = Vec::new();
        h.king_move(2,1, &mut vec2);
        //assert!(vec2.len() == 6);
        assert!(vec2.contains(&(1,0)));
        assert!(vec2.contains(&(2,0)));
        assert!(vec2.contains(&(3,0)));
        assert!(vec2.contains(&(1,1)));
        assert!(vec2.contains(&(3,1)));
        assert!(vec2.contains(&(2,2)));
    }
}



