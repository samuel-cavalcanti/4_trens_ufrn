use crate::{Track, TrackState};
use std::sync::{Mutex, Arc};

mod green_circuit;
mod purple_circuit;
mod red_circuit;
mod blue_circuit;

pub use red_circuit::RedCircuit;
pub use purple_circuit::PurpleCircuit;
pub use green_circuit::GreenCircuit;
pub use blue_circuit::BlueCircuit;


fn print_waiting(train_id: u64, state: TrackState) {
    let number = state as usize + 1;
    println!("train {} waiting L{}", train_id, number);
}