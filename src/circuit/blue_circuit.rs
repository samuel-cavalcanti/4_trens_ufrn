use crate::circuit::print_waiting;
use crate::{Track, TrackState, Train};
use std::sync::{Arc, Mutex};

pub struct BlueCircuit {
    train: Train,
    state: TrackState,
    tracks_1: Vec<(Arc<Mutex<Track>>, TrackState)>,
    tracks_2: Vec<Arc<Mutex<Track>>>,
}

impl BlueCircuit {
    pub fn new(tracks: Vec<Arc<Mutex<Track>>>) -> BlueCircuit {
        BlueCircuit {
            train: Train::new(4, 5),
            state: TrackState::L13,
            tracks_1: vec![
                (tracks[TrackState::L12 as usize].clone(), TrackState::L12),
                (tracks[TrackState::L13 as usize].clone(), TrackState::L13),
                (tracks[TrackState::L11 as usize].clone(), TrackState::L11),
            ],
            tracks_2: vec![
                tracks[TrackState::L4 as usize].clone(),
                tracks[TrackState::L6 as usize].clone(),
                tracks[TrackState::L10 as usize].clone(),
            ],
        }
    }

    pub fn run(&self, ui_state: Arc<Mutex<TrackState>>, train: &Train) {
        for (track, track_state) in self.tracks_1.clone() {
            {
                let result_lock = track.lock().unwrap();
                if let Ok(mut mutex_state) = ui_state.lock() {
                    *mutex_state = track_state;
                }
                result_lock.run(&train);
            }
        }

        {
            let l4 = self.tracks_2[0].lock().unwrap();
            let l6 = self.tracks_2[1].lock().unwrap();
            let l10 = self.tracks_2[2].lock().unwrap();

            if let Ok(mut mutex_state) = ui_state.lock() {
                *mutex_state = TrackState::L4;
            }

            l4.run(&train);

            drop(l4); // libera o lock

            if let Ok(mut mutex_state) = ui_state.lock() {
                *mutex_state = TrackState::L6;
            }

            l6.run(&train);

            drop(l6); // libera o lock

            if let Ok(mut mutex_state) = ui_state.lock() {
                *mutex_state = TrackState::L10;
            }
            l10.run(&train); // ao sair de contexto o lock é liberado
        }
    }
}
