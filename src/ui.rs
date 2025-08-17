
use macroquad::prelude::*;
use crate::game::*;
use std::collections::HashMap;

pub struct UI{
    shift_x: f32,
    shift_y: f32,
    field_size: f32,
    textures: HashMap<i32,Texture2D> 
}

impl UI {
    pub async fn new() -> UI{

        let mut textures = HashMap::new();
        textures.insert(101,  load_texture("textures/little_green_pawn.png").await.unwrap());

        UI{
            shift_x: 350.,
            shift_y: 125.,
            field_size: 55.,
            textures
        }
    }

    pub fn draw(&self, game: &Game){

        let shift_x = self.shift_x;
        let shift_y = self.shift_y;
        let pawn: Texture2D = self.textures.get(&101).unwrap().clone();
        

        let field_size = self.field_size;

        clear_background(RED);

        for x in 0..game.map.x_map_bound {
           

            for y in 0..game.map.y_map_bound {
                let current_y = (y as f32) + shift_y + (y as f32) * field_size + {
                    if x % 2 == 1 { field_size/2.0 } else { 0.0 }
                };
                let current_x = (x as f32) + shift_x + (x as f32) * field_size;
                draw_rectangle(current_x, current_y, field_size, field_size, GRAY );
                draw_rectangle_lines( current_x, current_y, field_size, field_size, 5.0, BLACK);

                if game.possible.contains(&(x, y)){
                    draw_rectangle_lines(current_x, current_y, field_size, field_size, 10.0, GREEN);
                }

                if game.map.figures.contains_key(&(x, y)){
                    draw_texture(&pawn, current_x, current_y, GRAY);

                }
            }
        }
    }


    pub fn calc_mouse_field(&self, game: &Game) -> Option<(usize, usize)>{

        let (mouse_x, mouse_y) = mouse_position();
        let shift_y = self.shift_y;
        let shift_x = self.shift_x;
        let field_size = self.field_size;
        
        for x in 0..game.map.x_map_bound {
           
            for y in 0..game.map.y_map_bound {
                let current_y = (y as f32) + shift_y + (y as f32) * field_size + {
                    if x % 2 == 1 { field_size/2.0 } else { 0.0 }
                };
                let current_x = (x as f32) + shift_x + (x as f32) * field_size;
                if  current_x < mouse_x 
                    && 
                    current_x + field_size > mouse_x 
                    &&
                    current_y < mouse_y
                    &&
                    current_y + field_size > mouse_y
                {
                    return Some((x, y));
                }
            }
        }

        None
    }

    pub fn select(&self, game: &mut Game){
        if is_mouse_button_pressed(MouseButton::Left){
            let mouse_field  = self.calc_mouse_field(&game);
            if let None = mouse_field{
                game.selected = None;
                //print!("i was here and nowhere else!!!");
                return;
            }
            match game.selected {
                Some(start) => {
                    game.selected = None;
                    move_figure(start, mouse_field.unwrap(), game);
                    game.possible = Vec::new();
                },
                None => {
                    game.selected = mouse_field;
                    game.possible = game.map.get_possible_moves(mouse_field.unwrap());
                    if game.possible.len() == 0 {
                        game.selected = None;
                    }
                }
            }
        }
    }
}


