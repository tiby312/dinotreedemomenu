pub use dinotreedemo;
pub use dinotreedemo::dinotree;
//extern crate axgeom;
pub use dinotree::axgeom;

use axgeom::Vec2;


use axgeom::Rect;

trait MenuTrait:Send+Sync{
    fn step(&mut self,poses:&[Vec2<f32>],border:&Rect<f32>)->(Option<Box<MenuTrait>>,GameResponse);
    fn get_bots(&self)->&[Bot];
}


pub use dinotreedemo::compute_border;
pub use duckduckgeo::bot::*;


pub struct GameResponse
{
    pub color:          Option<[f32;3]>,
    pub is_game:        bool,
    pub new_game_world: Option<(Rect<f32>,f32)>
}

pub struct Symbols{
    digit_table:ascii_num::digit::DigitSymbolTable,
    game_table:ascii_num::GameSymbolTable,
}
impl Symbols{
    pub fn new()->Symbols{
        Symbols{
            digit_table:ascii_num::digit::DigitSymbolTable::new_default(),
            game_table:ascii_num::GameSymbolTable::new()
        }
    }
}
pub struct MenuGame<'a>{
    symbols:&'a Symbols,
    state:Box<MenuTrait + 'a>
}
impl<'a> MenuGame<'a>{
    pub fn new(symbols:&'a Symbols)->(MenuGame<'a>,GameResponse){
        let (a,col,rect,radius)=menusys::Menu::new(symbols);
        (MenuGame{symbols,state:Box::new(a)},GameResponse{color:Some(col),is_game:false,new_game_world:Some((rect,radius))})
    }

    pub fn step(&mut self,poses:&[Vec2<f32>],border:&Rect<f32>)->GameResponse{
        let (a,b)=self.state.step(poses,border);
        match a{
            Some(x)=>{
                self.state=x;
            },
            None=>{

            }
        }
        b
    }

    pub fn get_bots(&self)->&[Bot]{
        self.state.get_bots()
    }
}


mod menusys;
mod menu_primitives;
