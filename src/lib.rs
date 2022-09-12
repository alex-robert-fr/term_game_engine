use std::{io::stdout, thread, time::Duration};

use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode}, cursor,
};
use render::Window;

pub mod render;
pub mod spacial;

pub struct Engine {
    pub fps: u8,
    pub state: State,
    pub window: Window,
}

impl Engine {
    pub fn new(fps: u8) -> Self {
        Engine {
            fps,
            state: State::new(),
            window: Window::new(),
        }
    }

    pub fn run<G>(&mut self, mut app: G)
    where
        G: FnMut(&mut State, &mut Window),
    {
        enable_raw_mode().unwrap();
        execute!(stdout(), EnableMouseCapture).unwrap();
        execute!(stdout(), cursor::Hide).unwrap();

        while !self.state.exit {
            self.window.cursor_origin();
            app(&mut self.state, &mut self.window);
            self.window.draw_screen();
            thread::sleep(Duration::from_millis(1_000 / 80));
        }
        execute!(stdout(), DisableMouseCapture).unwrap();
        disable_raw_mode().unwrap();
    }
}

pub struct State {
    pub exit: bool,
}

impl State {
    pub fn new() -> Self {
        State { exit: false }
    }
}
