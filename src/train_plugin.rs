use std::sync::{Arc, Mutex};

use bevy::prelude::*;
use tracks::{
    circuit::{BlueCircuit, Circuit, GreenCircuit, PurpleCircuit, RedCircuit},
    Track, Train,
};

use crate::{TrainMaterials, TrainState, UiTrackPos};

pub struct TrainPlugin;

impl Plugin for TrainPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_stage(
            "spawn_train_entities",
            SystemStage::single(train_block_spawn.system()),
        )
        .add_system(train_update.system());
    }
}

fn train_block_spawn(
    mut commands: Commands,
    train_materials: Res<TrainMaterials>,
    ui_tracks: Res<UiTrackPos>,
    tracks: Res<Vec<Arc<Mutex<Track>>>>,
) {
    println!("train_block_spawn!!");

    let size = Vec2::new(
        train_materials.train_block_size,
        train_materials.train_block_size,
    );

    let green_circuit = GreenCircuit::new(tracks.clone());
    let purple_circuit = PurpleCircuit::new(tracks.clone());
    let red_circuit = RedCircuit::new(tracks.clone());
    let blue_circuit = BlueCircuit::new(tracks.clone());

    let circuits: Vec<Arc<dyn Circuit + Send + Sync>> = vec![
        Arc::new(green_circuit),
        Arc::new(purple_circuit),
        Arc::new(red_circuit),
        Arc::new(blue_circuit),
    ];

    let trains = vec![
        Arc::new(Mutex::new(Train::new(0, 2))),
        Arc::new(Mutex::new(Train::new(1, 3))),
        Arc::new(Mutex::new(Train::new(2, 3))),
        Arc::new(Mutex::new(Train::new(3, 4))),
    ];

    let materials = vec![
        train_materials.green_train_material.clone(),
        train_materials.purple_train_material.clone(),
        train_materials.red_train_material.clone(),
        train_materials.blue_train_material.clone(),
    ];
    commands.insert_resource(trains.clone());

    for ((circuit, train), material) in circuits.iter().zip(trains).zip(materials) {
        let state = Arc::new(Mutex::new(circuit.initial_track_state()));
        let translation = ui_tracks.track_pos[circuit.initial_track_state() as usize]
            .0
            .clone();

        let cloned_circuit: Arc<dyn Circuit + Send + Sync> = circuit.clone();
        commands
            .spawn_bundle(SpriteBundle {
                material,
                transform: Transform {
                    translation,
                    ..Default::default()
                },
                sprite: Sprite::new(size),
                ..Default::default()
            })
            .insert(TrainState {
                state: state.clone(),
            })
            .insert(std::thread::spawn(move || loop {
                let circuit_train;
                {
                    circuit_train = train.lock().unwrap().clone();
                }

                cloned_circuit.run(state.clone(), &circuit_train);
            }));
    }
}

fn train_update(query: Query<(&TrainState, &mut Transform)>, ui_tracks: Res<UiTrackPos>) {
    query.for_each_mut(|(train_state, mut transform)| {
        if let Ok(mutex) = train_state.state.try_lock() {
            let state = mutex.clone();
            transform.translation = ui_tracks.track_pos[state as usize].0.clone();
        }
    });
}
