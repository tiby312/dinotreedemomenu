#![feature(try_from)]
extern crate axgeom;
extern crate compt;
extern crate ordered_float;
extern crate dinotree_alg;
extern crate dinotree;
extern crate dinotree_measure;
extern crate dists;
extern crate num;



use dinotreedemo::*;
use ascii_num::*;
use ordered_float::*;
use axgeom::*;

use crate::menu_primitives::*;

trait MenuTrait:Send+Sync{
    fn step(&mut self,poses:&[Vec2],border:&Rect<f32>)->(Option<Box<MenuTrait>>,GameResponse);
    fn get_bots(&self)->&[Bot];
}


pub use dinotreedemo::compute_border;
pub use dinotreedemo::Bot;
pub use dinotreedemo::Vec2;


pub struct GameResponse
{
    pub color:          Option<[f32;3]>,
    pub is_game:        bool,
    pub new_game_world: Option<(Rect<f32>,f32)>
}

pub struct MenuGame{
    state:Box<MenuTrait>
}
impl MenuGame{
    pub fn new()->(MenuGame,GameResponse){
        let (a,col,rect,radius)=menusys::Menu::new();
        (MenuGame{state:Box::new(a)},GameResponse{color:Some(col),is_game:false,new_game_world:Some((rect,radius))})
    }

    pub fn step(&mut self,poses:&[Vec2],border:&Rect<f32>)->GameResponse{
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
