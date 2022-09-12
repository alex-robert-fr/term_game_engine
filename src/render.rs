use std::io::{stdout, Write};

use crossterm::terminal::size;

use crate::spacial::Vector2D;

pub struct Window {
    size: (u16, u16),
    screen: Vec<Vec<char>>
}

impl Window {
    pub fn new() -> Self {
        let size = size().unwrap();
        let mut screen = Vec::new();
        for _ in 0..size.1 {
            let mut x_line = Vec::new();
            for _ in 0..size.0 {
                x_line.push(' ');
            }
            screen.push(x_line);
        }

        Window {
            size,
            screen,
        }
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


pub struct Pencil<'a> {
    window: &'a mut Window
}

impl<'a> Pencil<'a> {
    pub fn new(window: &'a mut Window) -> Self {
        Pencil { window }
    }

    pub fn draw_text(&mut self, text: &str, pos: Vector2D) {
        for i in pos.x..(pos.x + text.len() as i32) {
            self.window.screen[pos.y as usize][i as usize] = text.as_bytes()[i as usize - pos.x as usize] as char;
        }
    }

    pub fn draw_item(&mut self, ch: char, pos: Vector2D) {
        self.window.screen[pos.y as usize][pos.x as usize] = ch;
    }
}