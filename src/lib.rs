pub struct Engine {
    pub fps: u8,
    pub game: Game,
    pub state: State
}

impl Engine {
    pub fn new(fps: u8) -> Self {
        Engine { fps, game: Game {  }, state: State::new() }
    }

    pub fn run<G>(&mut self, mut game: G) where G: FnMut(&mut Game) {
        while !self.state.exit {
            println!("Run Engine");
            game(&mut self.game);
            println!("After Run Engine");
        }
    }
}

pub struct Game {
    
}

pub struct State {
    exit: bool
}

impl State {
    pub fn new() -> Self {
        State { exit: false }
    }
}