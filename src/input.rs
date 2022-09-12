use std::time::Duration;

use crossterm::event::{
    poll, read, Event, KeyCode, KeyEvent, KeyEventKind, KeyEventState, KeyModifiers,
};

pub struct Keyboard {}

impl Keyboard {
    pub fn get_last_key_press(&self) -> Option<Key> {
        match poll(Duration::from_millis(0)) {
            Ok(val) => {
                if val {
                    match read().unwrap() {
                        Event::Key(event) => Some(self.translate_key(event)),
                        _ => None,
                    }
                } else {
                    Some(Key::Unknown)
                }
            },
            Err(_) => None,
        }
    }

    fn translate_key(&self, key: KeyEvent) -> Key {
        match key {
            KeyEvent {
                code: KeyCode::Char('q'),
                modifiers: KeyModifiers::NONE,
                kind: KeyEventKind::Press,
                state: KeyEventState::NONE,
            }
            | KeyEvent {
                code: KeyCode::Char('Q'),
                modifiers: KeyModifiers::SHIFT,
                kind: KeyEventKind::Press,
                state: KeyEventState::NONE,
            } => Key::Q,
            _ => Key::Unknown,
        }
    }
}

pub enum Key {
    Q,
    Unknown,
}
