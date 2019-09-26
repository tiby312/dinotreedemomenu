
use dinotreedemo::*;
use axgeom::*;

use laid_dot::*;
use super::*;

use axgeom::Vec2;


pub static COLS:&'static [[f32;3]]=
    &[
        [0.9,0.9,0.9],
        [1.0,0.2,0.2],
        [0.6,0.5,1.0],
        [1.0,1.0,0.0],
        [1.0,0.0,1.0],
        [0.0,1.0,1.0],
    ];


pub struct Menu<'a>{
    bots: Vec<Bot>,
    buttons:[Button<'a>;3],
    color_button:Button<'a>,
    color_clicker:Clicker,
    col_counter:usize,
    numberthing:NumberThing<'a>
}

impl<'a> Menu<'a>{

    pub fn new(symbols:&'a Symbols)->(Menu,[f32;3],Rect<f32>,f32){
        
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
        let z=vec2(0.0,0.0);
        let bots:Vec<Bot>=(0..num_bots).map(|_|Bot{pos:z,vel:z,acc:z}).collect();



        let kk=vec2(-200.0,-100.0);
        let color_button=Button::new(kk,symbols.game_table.lookup(3),unit*2.0);


        let buttons={
            let mut v=vec2(-200.0,100.0);
            let b1=Button::new(v,symbols.game_table.lookup(0),unit*2.0);
            v.x+=unit*20.0;
            let b2=Button::new(v,symbols.game_table.lookup(1),unit*2.0);
            v.x+=unit*20.0;
            let b3=Button::new(v,symbols.game_table.lookup(2),unit*2.0);
            v.x+=unit*20.0;
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
            NumberThing::new(symbols.digit_table.lookup_number(40_000),unit*15.0,unit*2.0,vec2(x,y))
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


impl<'a> MenuTrait for Menu<'a>{
    fn step(&mut self,poses:&[Vec2<f32>],_border:&Rect<f32>)->(Option<Box<dyn MenuTrait>>,GameResponse){
        
        let bots=&mut self.bots;
        
        for i in poses.iter(){
            let curr=self.numberthing.get_number();

            //up arrow
            if self.buttons[0].get_dim().contains_point(*i){
                self.numberthing.update_number(curr+50);
            }
            if self.buttons[1].get_dim().contains_point(*i){
                self.numberthing.update_number((curr as isize-50).max(1) as usize); 
            }
            if self.buttons[2].get_dim().contains_point(*i){

                let (game,rect,radius)=BotSystem::new(curr);
                return (Some(Box::new(Game{game})),GameResponse{color:None,is_game:true,new_game_world:Some((rect,radius))})
            }
        }

        if self.color_clicker.update(self.color_button.get_dim(),poses){
            self.col_counter=(self.col_counter+1) % COLS.len();
        }

        {
            let mut bb=bots.iter_mut();

            
            for digit in self.numberthing.iter(){
                for pos in digit{
                    bb.next().unwrap().pos=pos;
                }
            }

        
            for i in self.buttons.iter(){
                for pos in i.iter(){

                    bb.next().unwrap().pos=pos;
                }
            }


            for pos in self.color_button.iter(){
                bb.next().unwrap().pos=pos;
            };
            
            for b in bb{
                b.pos=vec2(-10000.0,-10000.0);
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
    fn step(&mut self,poses:&[Vec2<f32>],border:&Rect<f32>)->(Option<Box<dyn MenuTrait>>,GameResponse){
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




