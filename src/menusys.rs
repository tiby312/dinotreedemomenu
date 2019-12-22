
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


pub struct Menu{
    bots: Vec<Bot>,
    buttons:[Button;3],
    color_button:Button,
    color_clicker:Clicker,
    col_counter:usize,
    numberthing:NumberThing
}

impl Menu{

    pub fn new(aspect_ratio:axgeom::AspectRatio,symbols:&Symbols)->(Menu,[f32;3],axgeom::Vec2AspectRatio,f32){
        
        let num_bots=5_000;
        
        let startx=500.0;
        let starty=500.0;

        let borderf32= vec2(1000.0,1000.0);

        let aa=if aspect_ratio.width_over_height()<1.0{
            Vec2AspectRatio{ratio:aspect_ratio,width:1000.0}
        }else{

            let k=Vec2AspectRatio{ratio:aspect_ratio,width:1000.0*aspect_ratio.width_over_height()};
            dbg!(k);
            k

        };
        
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



        let kk=vec2(200.0,100.0);
        let color_button=Button::new(kk,3,unit*2.0,&symbols.game_table.0);


        let buttons={
            let mut v=vec2(200.0,400.0);
            let b1=Button::new(v,0,unit*2.0,&symbols.game_table.0);
            v.x+=unit*20.0;
            let b2=Button::new(v,1,unit*2.0,&symbols.game_table.0);
            v.x+=unit*20.0;
            let b3=Button::new(v,2,unit*2.0,&symbols.game_table.0);
            v.x+=unit*20.0;
            [b1,b2,b3]
        };

        let numberthing={
            let x=700.0;
            let y=500.0;
            NumberThing::new(40_000,unit*15.0,unit*2.0,vec2(x,y))
        };

        let col=COLS[0];

        (Menu{
            bots,
            buttons,
            color_button,
            col_counter:0 , //TODO hack
            color_clicker:Clicker::new(),
            numberthing,
        },col,aa ,10.0)
    }
}


impl MenuTrait for Menu{
    fn step(&mut self,poses:&[Vec2<f32>],_border:&Vec2<f32>,symbols:&Symbols,aspect_ratio:axgeom::AspectRatio)->(Option<Box<dyn MenuTrait>>,GameResponse){
        
        let bots=&mut self.bots;
        
        for i in poses.iter(){
            let curr=self.numberthing.get_number();

            dbg!(self.buttons[0].get_dim(),*i);

            //up arrow
            if self.buttons[0].get_dim().contains_point(*i){
                self.numberthing.update_number(curr+50);
            }
            if self.buttons[1].get_dim().contains_point(*i){
                self.numberthing.update_number((curr as isize-50).max(1) as usize); 
            }
            if self.buttons[2].get_dim().contains_point(*i){
                let (game,rect,radius)=BotSystem::new(aspect_ratio,curr);
                return (Some(Box::new(Game{game})),GameResponse{color:None,is_game:true,new_game_world:Some((rect,radius))})
            }
        }

        if self.color_clicker.update(self.color_button.get_dim(),poses){
            self.col_counter=(self.col_counter+1) % COLS.len();
        }

        {
            let mut bb=bots.iter_mut();

            for digit in self.numberthing.iter(&symbols.digit_table){
                for pos in digit{
                    bb.next().unwrap().pos=pos;
                }
            }
        
            for i in self.buttons.iter(){
                for pos in i.iter(&symbols.game_table.0){
                    bb.next().unwrap().pos=pos;
                }
            }

            for pos in self.color_button.iter(&symbols.game_table.0){
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
    fn step(&mut self,poses:&[Vec2<f32>],border:&Vec2<f32>,symbols:&Symbols,aspect_ratio:axgeom::AspectRatio)->(Option<Box<dyn MenuTrait>>,GameResponse){
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




