pub struct Engine {
    pub fps: u8,
    pub game: Game
}

impl Engine {
    pub fn new(fps: u8) -> Self {
        Engine { fps, game: Game {  } }
    }

    pub fn run<G>(&mut self, mut game: G) where G: FnMut(&mut Game) {
        println!("Run Engine");
        game(&mut self.game);
        println!("After Run Engine");
    }
}

pub struct Game {
    
}