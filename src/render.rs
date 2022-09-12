use std::io::{stdout, Write};

use crossterm::terminal::size;

pub struct Window {
    size: (u16, u16),
    screen: Vec<Vec<char>>,
}

impl Window {
    pub fn new() -> Self {
        let size = size().unwrap();
        let mut screen = Vec::new();
        for _ in 0..size.1 {
            let mut x_line = Vec::new();
            for _ in 0..size.0 {
                x_line.push('#');
            }
            screen.push(x_line);
        }

        Window {
            size,
            screen,
        }
    }

    pub fn clear(&self) {
        print!("\x1b[2J");
        stdout().flush().unwrap();
    }

    pub fn cursor_origin(&self) {
        print!("\x1b[0;0H");
        stdout().flush().unwrap();
    }
    pub fn draw_screen(&self) {
        // let mut screen = String::new();
        for y in self.screen.iter() {
            for x in y.iter() {
                // screen.push(*x);
                print!("{x}");
            }
        }
        // print!("{screen}");
        stdout().flush().unwrap();
    }
}
