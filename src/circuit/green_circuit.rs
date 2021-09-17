
use crate::{Track, TrackState, Train};
use std::sync::{Arc, Mutex};

pub struct GreenCircuit {

    tracks: Vec<(Arc<Mutex<Track>>, TrackState)>,
}

impl GreenCircuit {
    pub fn new(tracks: Vec<Arc<Mutex<Track>>>) -> GreenCircuit {
        GreenCircuit {
            tracks: vec![
                (tracks[0].clone(), TrackState::L1),
                (tracks[1].clone(), TrackState::L2),
                (tracks[2].clone(), TrackState::L3),
                (tracks[3].clone(), TrackState::L4),
            ],
        }
    }

    pub fn run(&self, ui_state: Arc<Mutex<TrackState>>, train: &Train) {
        for (track, track_state) in self.tracks.clone() {
            {
                let result_lock = track.lock().unwrap();
                if let Ok(mut mutex_state) = ui_state.lock() {
                    *mutex_state = track_state.clone();
                }
                result_lock.run(train);
            }
        }
    }
}
