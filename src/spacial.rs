#[derive(PartialEq, Eq, Clone, Copy)]
pub struct Vector2D {
    pub x: i32,
    pub y: i32,
}

impl Vector2D {
    pub fn new(x: i32, y: i32) -> Self {
        Vector2D { x, y }
    }
}