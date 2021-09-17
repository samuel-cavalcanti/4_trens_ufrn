pub mod circuit;
pub mod track;

use crate::circuit::{BlueCircuit, GreenCircuit, PurpleCircuit, RedCircuit};
use std::sync::{Arc, Mutex};

pub use crate::track::Track;
use std::thread::JoinHandle;

// pub use crate::circuit::green_circuit;
// pub use crate::circuit::purple_circuit;

#[derive(Clone)]
pub enum TrackState {
    L1 = 0,
    L2,
    L3,
    L4,
    L5,
    L6,
    L7,
    L8,
    L9,
    L10,
    L11,
    L12,
    L13,
}

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
        if self.velocity > 2 {
            self.velocity -= 1;
        }

       
    }
}
