extern crate dinotreedemo;
extern crate axgeom;
extern crate ascii_num;
extern crate ordered_float;

use dinotreedemo::*;
use axgeom::*;

use crate::menu_primitives::*;
use super::*;



pub static COLS:&'static [[f32;3]]=
    &[
        [0.0,1.0,0.0],
        [0.9,0.9,0.9],
        [1.0,0.2,0.2],
        [0.6,0.5,1.0],
        [1.0,1.0,0.0],
        [1.0,0.0,1.0],
        [0.0,1.0,1.0],
    ];


pub struct Menu{
    bots: Vec<Bot>,
    buttons:[Button;3],
    color_button:Button,
    color_clicker:Clicker,
    col_counter:usize,
    numberthing:NumberThing
}

impl Menu{

    pub fn new()->(Menu,[f32;3],Rect<f32>,f32){
        
        let num_bots=5_000;
        
        let startx=500.0;
        let starty=500.0;

        //let border= axgeom::Rect::new(NotNaN::new(-startx).unwrap(),NotNaN::new(startx).unwrap(),NotNaN::new(-starty).unwrap(),NotNaN::new(starty).unwrap());
        let borderf32= axgeom::Rect::new(-startx,startx ,-starty,starty);

        //used as the building block for all positions
        let unit=8.0;//bot::get_unit(startx,starty);
        
        //let br=unit*1.0;
        //let mr=unit*10.0;

        //let (bot_prop,mouse_prop)=bot::create_from_radius(br,mr);
        //let bots=bot::create_bots(num_bots,&border,&bot_prop);
        //let s=dists::spiral::Spiral::new([0.0,0.0],12.0,1.0);
        //let bots:Vec<Bot>=s.take(num_bot).map(|pos|Bot::new(&Vec2::new(pos[0] as f32,pos[1] as f32))).collect();
        let z=Vec2::new(0.0,0.0);
        let bots:Vec<Bot>=(0..num_bots).map(|_|Bot{pos:z,vel:z,acc:z}).collect();



        let kk=Vec2::new(-200.0,-100.0);
        let color_button=Button::new(kk,ascii_num::get_misc(3),unit*2.0);


        let buttons={
            let mut v=Vec2::new(-200.0,100.0);
            let b1=Button::new(v,ascii_num::get_misc(0),unit*2.0);
            v.0[0]+=unit*20.0;
            let b2=Button::new(v,ascii_num::get_misc(1),unit*2.0);
            v.0[0]+=unit*20.0;
            let b3=Button::new(v,ascii_num::get_misc(2),unit*2.0);
            v.0[0]+=unit*20.0;
            [b1,b2,b3]
        };

        /*
        let kk=Vec2::new(unit*5.0,starty as f32-unit*70.0);    
        let debug_button=OnOffButton::new(kk,
                ascii_num::get_misc(4),
                ascii_num::get_misc(5),
                unit*2.0);
        */

        let numberthing={
            let x=startx as f32-100.0;
            let y=starty as f32-200.0;
            NumberThing::new(unit*15.0,unit*2.0,40_000,Vec2::new(x,y))
        };

        let col=COLS[0];

        (Menu{
            bots,
            buttons,
            color_button,
            col_counter:0 , //TODO hack
            color_clicker:Clicker::new(),
            numberthing,
        },col,borderf32,10.0)
    }
}


impl MenuTrait for Menu{
    fn step(&mut self,poses:&[Vec2],_border:&Rect<f32>)->(Option<Box<MenuTrait>>,GameResponse){
        
        let bots=&mut self.bots;
        
        for i in poses.iter(){
            let curr=self.numberthing.get_number();

            //up arrow
            if self.buttons[0].get_dim().contains_point(i.0){
                self.numberthing.update_number(curr+50);
            }
            if self.buttons[1].get_dim().contains_point(i.0){
                self.numberthing.update_number((curr as isize-50).max(1) as usize); 
            }
            if self.buttons[2].get_dim().contains_point(i.0){

                let (game,rect,radius)=BotSystem::new(curr);
                return (Some(Box::new(Game{game})),GameResponse{color:None,is_game:true,new_game_world:Some((rect,radius))})
            }
        }

        if self.color_clicker.update(self.color_button.get_dim(),poses){
            self.col_counter=(self.col_counter+1) % COLS.len();
        }

        {
            let mut bb=IteratorCounter::new(bots.iter_mut());
         
            self.numberthing.draw(&mut bb);

            for i in self.buttons.iter(){
                i.draw(&mut bb);
            }

            self.color_button.draw(&mut bb);
            
            for b in bb{
                b.pos=Vec2::new(-10000.0,-10000.0);
            }
        }

        let col=COLS[self.col_counter]; //TODO only show when it changes?
        let g=GameResponse{new_game_world:None,color:Some(col),is_game:false};
        (None,g)
    }
    
    fn get_bots(&self)->&[Bot]{
        &self.bots
    }
}


struct Game{
    game:dinotreedemo::BotSystem
}
impl MenuTrait for Game{
    fn step(&mut self,poses:&[Vec2],border:&Rect<f32>)->(Option<Box<MenuTrait>>,GameResponse){
        self.game.step(poses,border);
        (None,GameResponse{
            color:None,
            is_game:true,
            new_game_world:None
        })
    }
    fn get_bots(&self)->&[Bot]{
        self.game.get_bots()
    }
}




