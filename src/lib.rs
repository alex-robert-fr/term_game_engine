pub struct Engine {
    pub fps: u8
}

impl Engine {
    pub fn new(fps: u8) -> Self {
        Engine { fps }
    }
}