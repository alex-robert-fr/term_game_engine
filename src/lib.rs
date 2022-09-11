use crossterm::terminal::{enable_raw_mode, disable_raw_mode};
use input::Keyboard;

pub mod input;

pub struct Engine {
    pub fps: u8,
    pub game: Game,
    pub state: State
}

impl Engine {
    pub fn new(fps: u8) -> Self {
        Engine { fps, game: Game {  }, state: State::new() }
    }

    pub fn run<G>(&mut self, mut game: G) where G: FnMut(&mut Game, &mut State) {
        enable_raw_mode().unwrap();
        while !self.state.exit {
            println!("Run Engine");
            game(&mut self.game, &mut self.state);
            println!("After Run Engine");
        }
        disable_raw_mode().unwrap();
    }
}

pub struct Game {
    
}

pub struct State {
    pub keyboard: Keyboard,
    pub exit: bool
}

impl State {
    pub fn new() -> Self {
        State { exit: false, keyboard: Keyboard {  } }
    }
}