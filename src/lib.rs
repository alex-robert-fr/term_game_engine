use std::{thread, time::Duration, io::stdout};

use crossterm::{terminal::{enable_raw_mode, disable_raw_mode}, execute, event::{EnableMouseCapture, DisableMouseCapture}};
use input::Keyboard;
use render::Window;

pub mod input;
pub mod render;

pub struct Engine {
    pub fps: u8,
    pub game: Game,
    pub state: State,
    pub window: Window
}

impl Engine {
    pub fn new(fps: u8) -> Self {
        Engine { fps, game: Game {  }, state: State::new(), window: Window::new() }
    }

    pub fn run<G>(&mut self, mut app: G) where G: FnMut(&mut Game, &mut State, &mut Window) {
        enable_raw_mode().unwrap();
        execute!(stdout(), EnableMouseCapture).unwrap();
        while !self.state.exit {
            self.window.cursor_origin();
            app(&mut self.game, &mut self.state, &mut self.window);
            self.window.draw_screen();
            thread::sleep(Duration::from_millis(33));
        }
        execute!(stdout(), DisableMouseCapture).unwrap();
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