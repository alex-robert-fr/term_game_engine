use crossterm::terminal::size;

pub struct Window {
    size: (u16, u16),
    // screen: Vec<Vec<char>>
}

impl Window {
    pub fn new() -> Self {
        let size = size().unwrap();

        Window { size }
    }

    pub fn to_fill_screen(&mut self) {
        for _ in 0..self.size.1 {
            for _ in 0..self.size.0 {
                print!("#");
            }
        }
    }

    pub fn clear(&self) {
        print!("\x1b[2j");
    }

    pub fn cursor_move_home(&self) {
        print!("\x1b[0;0H");
    }
}
