use crate::circuit::print_waiting;
use crate::{Track, TrackState, Train};
use std::{
    convert::TryInto,
    sync::{Arc, Mutex},
};

pub struct PurpleCircuit {
    train: Train,
    state: TrackState,
    tracks: Vec<(Arc<Mutex<Track>>, TrackState)>,
}

impl PurpleCircuit {
    pub fn new(tracks: Vec<Arc<Mutex<Track>>>) -> PurpleCircuit {
        PurpleCircuit {
            train: Train::new(2, 2),
            state: TrackState::L7,
            tracks: vec![
                (tracks[TrackState::L7 as usize].clone(), TrackState::L7),
                (tracks[TrackState::L5 as usize].clone(), TrackState::L5),
                (tracks[TrackState::L6 as usize].clone(), TrackState::L6),
                (tracks[TrackState::L3 as usize].clone(), TrackState::L3),
            ],
        }
    }

    pub fn run(&self, ui_state: Arc<Mutex<TrackState>>, train: &Train) {
        for (track, track_state) in self.tracks.clone() {
            {
                let result_lock = track.lock().unwrap();
                if let Ok(mut mutex_state) = ui_state.lock() {
                    *mutex_state = track_state;
                }
                result_lock.run(&train);
            }
        }
    }
}
