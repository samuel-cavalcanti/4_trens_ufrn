use crate::Train;
use std::time::Duration;
use std::thread::sleep;

pub struct Track {
    pub name: String,
    pub distance: u64,
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
