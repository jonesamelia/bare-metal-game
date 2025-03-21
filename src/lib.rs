#![no_std]

use num::Integer;
use pc_keyboard::{DecodedKey, KeyCode};
use pluggable_interrupt_os::vga_buffer::{
    clear_screen, is_drawable, plot, plot_num, plot_str, Color, ColorCode, BUFFER_HEIGHT, BUFFER_WIDTH
};

use core::{
    clone::Clone,
    cmp::{min, Eq, PartialEq},
    iter::Iterator,
    marker::Copy,
    prelude::rust_2024::derive,
};


pub fn safe_add<const LIMIT: usize>(a: usize, b: usize) -> usize {
    (a + b).mod_floor(&LIMIT)
}

pub fn add2<const LIMIT: usize>(value: usize) -> usize {
    safe_add::<LIMIT>(value, 2)
}

pub fn sub2<const LIMIT: usize>(value: usize) -> usize {
    safe_add::<LIMIT>(value, LIMIT - 2)
}

pub struct Player {
    px: usize,
    py: usize,
}

#[derive(Copy, Clone)]
pub struct Obstacle {
    ox: usize,
    oy: usize,
    color: Color,
    speed: usize,
}
pub struct Frogger{
    player: Player,
    obstacles: [Obstacle; 10],
    lilypads: [LilyPad; 5],
    frogs: usize,
    living: bool,
    init: bool,
    score: usize,
    hiscore: usize,
    ddelay: usize,
    currenty: usize,
    openl: usize,
    started: bool
}

#[derive(Copy, Clone)]
pub struct LilyPad{
    lx: usize,
    ly: usize,
    taken: bool
}

impl Default for Player{
    fn default() -> Self {
        Self {
            px: 37,
            py: BUFFER_HEIGHT - 2,
        }
    }
}

impl Default for Obstacle{
    fn default() -> Self {
        Self {
           ox: BUFFER_WIDTH - 8,
           oy: 20,
            color: Color::Blue, 
            speed: 1
        }
    }
}

impl Default for Frogger{
    fn default() -> Self {
        Self {
            player: Player::default(),
            frogs: 7,
            obstacles: [Obstacle::default(); 10],
            lilypads: [LilyPad:: default(); 5],
            living: true,
            score: 0,
            hiscore: 0,
            ddelay: 0,
            init: false,
            currenty: BUFFER_HEIGHT - 4,
            openl: 5,
            started: true,

        }
    }
}

impl Default for LilyPad{
    fn default() -> Self {
        Self {
            lx: 7,
            ly: 3,
            taken: false 
        }
    }
}


impl Player {
    fn draw(&mut self){
        let mut clr= ColorCode::new(Color::LightGreen, Color::Black);
        if self.py == BUFFER_HEIGHT-4 {
            clr = ColorCode::new(Color::LightGreen, Color::Pink);
        }

        plot('-', self.px, self.py, clr);
        plot('0', self.px-1, self.py, clr);
        plot('0', self.px+1, self.py, clr);
        //plot('_', self.px-3, self.py +1, ColorCode::new(Color::Green, Color::Black));
        plot('^', self.px-2, self.py +1, clr);
        plot('(', self.px-1, self.py +1, clr);
        plot('_', self.px, self.py +1, clr);
        plot(')', self.px + 1, self.py +1, clr);
        plot('^', self.px + 2, self.py +1, clr);
        //plot('_', self.px + 3, self.py +1, ColorCode::new(Color::Green, Color::Black));
    }
    fn clear(&self){
        let mut clr= ColorCode::new(Color::Black, Color::Black);
        if self.py == BUFFER_HEIGHT-4 {
            clr = ColorCode::new(Color::Pink, Color::Pink);
        }
        for x in 0..3{
            plot(' ', x + self.px -1, self.py, clr);
        }
        for x2 in 0..5{
            plot(' ', x2 + self.px - 2, self.py + 1, clr);
        }
    }
}
impl Obstacle {
    fn draw(&mut self){
       // let h: &str = "{}"; BUFFER_HEIGHT;
        //plot_str(h, self.oy - 2, self.ox, ColorCode::new(self.color, Color::Black));
        
        plot('(', self.ox, self.oy, ColorCode::new(self.color, Color::Black));
        plot('(', self.ox, self.oy - 1, ColorCode::new(self.color, Color::Black));
        
        
        if self.ox + 1 < BUFFER_WIDTH{
            plot('0', self.ox + 1, self.oy, ColorCode::new(self.color, Color::Black));
            plot('0', self.ox + 1, self.oy -1, ColorCode::new(self.color, Color::Black));
        }
        for bx in 2..6{
            if self.ox + bx < BUFFER_WIDTH{
                plot('_', self.ox + bx, self.oy - 1, ColorCode::new(self.color, self.color)); 
                plot('_', self.ox + bx, self.oy, ColorCode::new(self.color, self.color));    
            }
            
        }
        if self.ox + 6 < BUFFER_WIDTH{
            plot(']', self.ox + 6, self.oy, ColorCode::new(self.color, Color::Black));
            plot(']', self.ox + 6, self.oy -1, ColorCode::new(self.color, Color::Black));
        }
        //plot('_', self.ox + 1, self.oy -1, ColorCode::new(self.color, Color::Black));
        
    }

    fn checkcollide(&mut self, px: usize, py: usize) -> bool{
        for x in self.ox..self.ox+6{
            for p in px-2..px+2{
                if x == p  && self.oy-1 == py{
                    return true;
                }
            }
        }
        return false;
    }

    fn clear(&mut self){
        for bx in 0..7{
            if self.ox + bx < BUFFER_WIDTH{
                plot(' ', self.ox + bx, self.oy - 1, ColorCode::new(Color::Black, Color::Black)); 
                plot(' ', self.ox + bx, self.oy, ColorCode::new(Color::Black, Color::Black));   
            }
        } 
    }

}

impl LilyPad{
    fn drawreg(&mut self){
        for nx in self.lx -1..self.lx +2{
            plot(' ', nx, self.ly, ColorCode::new(Color::Black, Color::Green));
            plot(' ', nx, self.ly + 1, ColorCode::new(Color::Black, Color::Green));
            plot(' ', nx, self.ly - 1, ColorCode::new(Color::Black, Color::Blue));
        }
        for by in self.ly..self.ly+2{
            plot(' ', self.lx -2, by, ColorCode::new(Color::Black, Color::Blue));
            plot(' ', self.lx +2, by, ColorCode::new(Color::Black, Color::Blue));
        }
    }

    fn froghome(&mut self){
        self.taken = true;
        plot('-', self.lx, self.ly+1, ColorCode::new(Color::LightGreen, Color::Green));
        plot('0', self.lx-1, self.ly+1, ColorCode::new(Color::LightGreen, Color::Green));
        plot('0', self.lx+1, self.ly+1, ColorCode::new(Color::LightGreen, Color::Green));
        plot('(', self.lx-1, self.ly, ColorCode::new(Color::LightGreen, Color::Green));
        plot(')', self.lx+1, self.ly, ColorCode::new(Color::LightGreen, Color::Green));    
        plot('v', self.lx+2, self.ly, ColorCode::new(Color::LightGreen, Color::Blue));    
        plot('v', self.lx-2, self.ly, ColorCode::new(Color::LightGreen, Color::Blue));                  
    }
}

impl Frogger {
    pub fn tick(&mut self){
        if self.init == false && self.started == true{
            self.draw();
            self.initialize();
        }
        if self.living{
                
            self.move_obstacles();
        }
        else if self.frogs == 0{
            
                    self.player.clear();
                    self.started = false;
                    self.game_over();    
                    self.ddelay = 0;
            
        }
        else{
            if self.ddelay == 0{
                self.player.draw();
                plot('x', self.player.px-1, self.player.py, ColorCode::new(Color::LightGreen, Color::Black));
                plot('x', self.player.px+1, self.player.py, ColorCode::new(Color::LightGreen, Color::Black));
                plot(' ', 25 +self.frogs, 1, ColorCode::new(Color::Black, Color::Black));
                self.ddelay += 1;
            }
            else if self.ddelay == 3{
                self.player.clear();
                self.player.px = Player::default().px;
                self.player.py = Player::default().py; 
                self.player.draw();
                self.living = true;
                self.currenty = Frogger::default().currenty;
                self.ddelay = 0;
            }
            else{
                self.ddelay += 1;
            }
        }
    }
    
    fn reset(&mut self){
        if self.player.py != 3{
            self.player.clear();
        }
        self.player.px = Player::default().px;
        self.player.py = Player::default().py;
        self.player.draw();
        self.living = true;
        self.currenty = Frogger::default().currenty; 
    }

    fn initialize(&mut self){
        for i in 0..self.lilypads.len(){
            self.lilypads[i].lx = 7 + i*15;
        }
        self.openl = self.lilypads.len();

        self.obstacles[1].ox = 30;
        self.obstacles[1].color = Color:: LightGray;
        self.obstacles[2].oy = 18;
        self.obstacles[2].ox = 10;
        self.obstacles[2].color = Color::LightBlue;
        self.obstacles[3].oy = 18;
        self.obstacles[3].ox = BUFFER_WIDTH - 30;
        self.obstacles[3].color = Color::LightCyan;
        self.obstacles[4].oy = 16;
        self.obstacles[4].ox = 22;
        self.obstacles[4].color = Color::Red;
        self.obstacles[4].speed = 2;
        self.obstacles[5].oy = 16;
        self.obstacles[5].color = Color:: LightRed;
        self.obstacles[5].speed = 2;
        self.obstacles[5].ox = BUFFER_WIDTH - 30;
        self.obstacles[6].oy = 12;
        self.obstacles[6].ox = 20;
        self.obstacles[6].color = Color::Yellow;
        self.obstacles[7].oy = 12;
        self.obstacles[7].color = Color::White;
        self.obstacles[7].ox = BUFFER_WIDTH - 41;
        self.obstacles[8].oy = 10;
        self.obstacles[8].speed = 2;
        self.obstacles[8].color = Color::Brown;
        self.obstacles[9].oy = 10;
        self.obstacles[9].speed = 2;
        self.obstacles[9].color = Color:: Cyan;
        self.obstacles[9].ox = BUFFER_WIDTH - 52;

        self.makelilypads();
        self.draw_ui();
        self.init = true;
        self.living = true;
        self.currenty = Frogger::default().currenty;
        self.ddelay = 0;
        self.reset();
        self.frogs = Frogger::default().frogs;

    }
    
    fn makelilypads(&mut self){
        for i in 0..self.lilypads.len(){
            self.lilypads[i].drawreg();
            self.lilypads[i].taken = false;
        }
        self.frogs = Frogger::default().frogs;
        self.openl = self.lilypads.len();
    }

    fn clear(&mut self){
        self.player.clear();  
    }

    fn move_obstacles(&mut self){
        let bw = BUFFER_WIDTH - 3;
        for i in 0..self.obstacles.len(){
            self.obstacles[i].clear();
            self.obstacles[i].ox = safe_add::<{BUFFER_WIDTH -3}>(self.obstacles[i].ox, bw -self.obstacles[i].speed);
            self.obstacles[i].draw();
            if self.obstacles[i].checkcollide(self.player.px, self.player.py){
                self.living = false;
                self.frogs = self.frogs-1;
                plot(' ', 25 +self.frogs, 1, ColorCode::new(Color::Black, Color::Black));
                self.draw_ui();
            }
        }
    }

    fn draw(&mut self){
        self.draw_ground();
        //self.obstacles[0].draw();
        self.player.draw();
        self.draw_ui();
    }
    fn draw_ground(&mut self){
        for x in 0..BUFFER_WIDTH{
            for y in 2..5{
                plot(' ', x, y, ColorCode::new(Color::Black, Color::LightGreen));
            }
        
        }
        for ry in BUFFER_HEIGHT-4..BUFFER_HEIGHT-2{
            for rx in 0..BUFFER_WIDTH{
                plot(' ', rx, ry, ColorCode::new(Color::Black, Color::Pink));
            }
            
        }
    }
    fn draw_ui(&mut self){
        plot_num(self.score.try_into().unwrap(), 3, 1, ColorCode::new(Color::White, Color::Black));

        for f in 0..self.frogs{
            plot('@', 25 +f, 1, ColorCode::new(Color::LightGreen, Color::Black));
        }
       // plot_str(s, 3, 1, ColorCode::new(Color::White, Color::Blue));
    }

    fn game_over(&mut self){
        self.player.clear();
        self.started = false;
        self.init = false;
        self.living = false;
        clear_screen();
        if self.score > self.hiscore{
            self.hiscore = self.score;
        }
        plot_str("GAME OVER", BUFFER_WIDTH/2 -4, 5, ColorCode::new(Color::White, Color::Black));
        plot_str("SCORE", 25, 10, ColorCode::new(Color::White, Color::Black));
        plot_num(self.score.try_into().unwrap(), 25, 11, ColorCode::new(Color::White, Color::Black));

        plot_str("HI-SCORE", 40, 10, ColorCode::new(Color::White, Color::Black));
        plot_num(self.hiscore.try_into().unwrap(), 40, 11, ColorCode::new(Color::White, Color::Black));

        plot_str("Press S to play again!", 20, 20, ColorCode::new(Color::White, Color::Black)); 

    }

    fn check_lilypads(&mut self){
        if self.player.py == 3{
            for i in 0..self.lilypads.len(){
                if self.lilypads[i].lx == self.player.px && self.lilypads[i].taken == false{
                    self.score += 50;
                    self.draw_ui();
                    self.lilypads[i].froghome();
                    self.openl -= 1;
                    if self.openl == 0{
                      self.score += 1000;
                      self.makelilypads();  
                      self.openl = 5;
                      self.draw_ui();
                    }
                    self.frogs -= 1;
                    plot(' ', 25 +self.frogs, 1, ColorCode::new(Color::Black, Color::Black));
                    self.reset(); 
                    if self.frogs == 0{
                        self.player.clear();
                        self.game_over();
                    }
                    break;
                    
                }
            }
        }
        
    }

    pub fn key(&mut self, key: DecodedKey) {
        match key {
            DecodedKey::RawKey(code) => self.handle_raw(code),
            DecodedKey::Unicode(c) => self.handle_unicode(c),
        }
    }

    fn handle_raw(&mut self, key: KeyCode) {
        if self.living{
          match key {
            KeyCode::ArrowLeft => {
                self.player.clear();
                self.player.px = safe_add::<BUFFER_WIDTH>(self.player.px, BUFFER_WIDTH - 5);
                self.player.draw();
            }
            KeyCode::ArrowRight => {
                self.player.clear();
                self.player.px = safe_add::<BUFFER_WIDTH>(self.player.px, 5);
                self.player.draw();
            }
            KeyCode::ArrowUp => {
                self.player.clear();
                self.player.py = sub2::<BUFFER_HEIGHT>(self.player.py);
                if self.player.py == 3{
                    let l = self.openl;
                    self.check_lilypads();
                    if self.openl == l{
                        self.player.py = add2::<BUFFER_HEIGHT>(self.player.py);
                        if self.started{
                            self.player.draw();
                        }
                        
                    }
                }
                
                if self.player.py < self.currenty{
                    self.score += 10;
                    if self.started{
                        self.draw_ui();
                    }
                    
                    self.currenty = self.player.py;
                }
                self.check_lilypads();
                self.player.draw();
            }
            KeyCode::ArrowDown => {
                if self.player.py < BUFFER_HEIGHT - 2{
                    self.player.clear();
                    self.player.py = add2::<BUFFER_HEIGHT>(self.player.py);
                    self.player.draw();
                }
            }
            _ => {}
            }  
        }
        
    }

    fn handle_unicode(&mut self, c: char){
        match c {
           /*  'a' => {
                if self.player.px > 0{
                    self.clear();
                    self.player.px -= 2;    
                }
                
            }
            'd' => {
                self.clear();
                self.player.px = safe_add::<BUFFER_WIDTH>(self.player.px, 2);
            }*/
            's' => {
                if self.started == false{
                    self.started = true;
                    clear_screen();
                    self.score = 0;
                    self.obstacles = Frogger::default().obstacles;
                }
            }
            _ => {}
        }
    }

}



    
