use crate::Train;
use std::time::Duration;
use std::thread::sleep;

pub struct Track {
    pub name: String,
    pub distance: u64,
}

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

impl Track {
    pub fn new(name: String, distance: u64) -> Track {
        Track {
            name,
            distance,
        }
    }

    pub fn run(&self, train: &Train) {
        // println!("train id: {} vel: {} in {}", train.id, train.velocity, self.name);
        // v = d/t
        // t = d/v
        let time = self.distance / train.velocity;
        sleep(Duration::new(time, 0));
    }
}
