use crate::{track::TrackState, Train};
use std::sync::{Arc, Mutex};

mod blue_circuit;
mod green_circuit;
mod purple_circuit;
mod red_circuit;

pub use blue_circuit::BlueCircuit;
pub use green_circuit::GreenCircuit;
pub use purple_circuit::PurpleCircuit;
pub use red_circuit::RedCircuit;

pub trait Circuit {
    fn run(&self, ui_state: Arc<Mutex<TrackState>>, train: &Train);

    fn initial_track_state(&self) -> TrackState;
}


