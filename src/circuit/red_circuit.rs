use crate::{Track, Train, track::TrackState};
use std::sync::{Arc, Mutex};

use super::Circuit;

pub struct RedCircuit {
    tracks: Vec<(Arc<Mutex<Track>>, TrackState)>,
}

impl RedCircuit {
    pub fn new(tracks: Vec<Arc<Mutex<Track>>>) -> RedCircuit {
        RedCircuit {
            tracks: vec![
                (tracks[TrackState::L8 as usize].clone(), TrackState::L8),
                (tracks[TrackState::L9 as usize].clone(), TrackState::L9),
                (tracks[TrackState::L10 as usize].clone(), TrackState::L10),
                (tracks[TrackState::L5 as usize].clone(), TrackState::L5),
            ],
        }
    }
}

impl Circuit for RedCircuit {
    fn run(&self, ui_state: Arc<Mutex<TrackState>>, train: &Train) {
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

    fn initial_track_state(&self) -> TrackState {
        self.tracks[0].1.clone()
    }
}
