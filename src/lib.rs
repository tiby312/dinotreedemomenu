pub use dinotreedemo;
pub use dinotreedemo::dinotree;
pub use dinotree::axgeom;

use axgeom::Vec2;


use axgeom::Rect;

trait MenuTrait:Send+Sync{
    fn step(&mut self,poses:&[Vec2<f32>],border:&Rect<f32>)->(Option<Box<dyn MenuTrait>>,GameResponse);
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
pub struct MenuGame{
    _symbols:Box<Symbols>,
    state:Box<dyn MenuTrait>
}
impl MenuGame{
    pub fn new()->(MenuGame,GameResponse){
        let symbols=Box::new(Symbols::new());
    
        let (a,col,rect,radius)=menusys::Menu::new(unsafe{&*(symbols.as_ref() as *const _)});
        (MenuGame{_symbols:symbols,state:Box::new(a)},GameResponse{color:Some(col),is_game:false,new_game_world:Some((rect,radius))})
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
