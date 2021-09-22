pub mod circuit;
pub mod track;


pub use crate::track::Track;

// pub use crate::circuit::green_circuit;
// pub use crate::circuit::purple_circuit;



pub enum Color {
    GREEN,
    PURPLE,
    RED,
    BLUE,
}

#[derive(Clone)]
pub struct Train {
    pub id: u64,
    pub velocity: u64,
}

impl Train {
    pub fn new(id: u64, velocity: u64) -> Self {
        Train { velocity, id }
    }

    pub fn increment(&mut self) {
        if self.velocity < 6 {
            self.velocity += 1;
        }
    }

    pub fn decrement(&mut self) {
        if self.velocity > 1 {
            self.velocity -= 1;
        }
    }
}

