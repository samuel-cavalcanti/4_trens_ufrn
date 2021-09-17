use bevy::prelude::*;
use std::sync::{Arc, Mutex};
use tracks::{
    circuit::{BlueCircuit, GreenCircuit, PurpleCircuit, RedCircuit},
    Train,
};
use tracks::{Track, TrackState};

pub struct TrainMaterials {
    green_train_material: Handle<ColorMaterial>,
    purple_train_material: Handle<ColorMaterial>,
    red_train_material: Handle<ColorMaterial>,
    blue_train_material: Handle<ColorMaterial>,
    train_block_size: f32,
}

pub struct TrackMaterials {
    green_track_material: Handle<ColorMaterial>,
    purple_track_material: Handle<ColorMaterial>,
    red_track_material: Handle<ColorMaterial>,
    blue_track_material: Handle<ColorMaterial>,
    background_material: Handle<ColorMaterial>,
}

pub struct UiTrackPos {
    track_pos: Vec<(Vec3, String)>,
    center_blue_track: Vec2,
}

pub struct HalfWindowSize {
    width: f32,
    height: f32,
}

pub enum ButtonAction {
    INCREMENT,
    DECREMENT,
}

#[derive(Clone)]
pub enum TrainID {
    GREEN,
    PURPLE,
    RED,
    BLUE,
}

pub struct GreenTrainID;

pub struct PurpleTrainID;

pub struct RedTrainID;

pub struct BlueTrainID;

pub struct TrainState {
    state: Arc<Mutex<TrackState>>,
}

const APP_NAME: &str = "Visualização da dinâmica dos trens";

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .insert_resource(WindowDescriptor {
            title: APP_NAME.to_string(),
            width: 600.0,
            height: 800.0,
            vsync: true,
            ..Default::default()
        })
        .add_startup_system(setup.system())
        .add_startup_stage(
            "spawn_track_entities",
            SystemStage::single(spawn_track_entities.system()),
        )
        .add_startup_stage(
            "spawn_train_entities",
            SystemStage::single(train_block_spawn.system()),
        )
        .add_startup_stage(
            "spawn_text_entities",
            SystemStage::single(spawn_text_entities.system()),
        )
        .add_startup_stage(
            "spawn_button_entities",
            SystemStage::single(spawn_button_entities.system()),
        )
        .add_system(green_train_update.system())
        .add_system(purple_train_update.system())
        .add_system(red_train_update.system())
        .add_system(blue_train_update.system())
        .add_system(press_button.system())
        .run();
}

fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut windows: ResMut<Windows>,
) {
    // camera
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());

    let window = windows.get_primary_mut().unwrap();

    window.set_position(IVec2::new(3870, 4830));

    let gray_scale = 90.2 / 100.0;
    let green_train_color = Color::rgb(7.1 / 100.0, 71.8 / 100.0, 58.8 / 100.0);
    let green_track_color = Color::rgb(56.5 / 100.0, 94.1 / 100.0, 85.1 / 100.0);

    let purple_train_color = Color::rgb(51.1 / 100.0, 55.3 / 100.0, 86.3 / 100.0);
    let purple_track_color = Color::rgb(85.1 / 100.0, 83.9 / 100.0, 92.9 / 100.0);

    let red_train_color = Color::rgb(1.0, 14.9 / 100.0, 8.2 / 100.0);
    let red_track_color = Color::rgb(96.5 / 100.0, 72.5 / 100.0, 16.1 / 100.0);

    let blue_train_color = Color::rgb(2.0 / 100.0, 42.7 / 100.0, 74.5 / 100.0);
    let blue_track_color = Color::rgb(6.7 / 100.0, 65.1 / 100.0, 94.5 / 100.0);

    let background_color = Color::rgb(gray_scale, gray_scale, gray_scale);

    commands.insert_resource(ClearColor(background_color.into()));

    commands.insert_resource(TrainMaterials {
        green_train_material: materials.add(green_train_color.into()),
        purple_train_material: materials.add(purple_train_color.into()),
        red_train_material: materials.add(red_train_color.into()),
        blue_train_material: materials.add(blue_train_color.into()),
        train_block_size: 50.0,
    });

    commands.insert_resource(TrackMaterials {
        green_track_material: materials.add(green_track_color.into()),
        purple_track_material: materials.add(purple_track_color.into()),
        red_track_material: materials.add(red_track_color.into()),
        blue_track_material: materials.add(blue_track_color.into()),
        background_material: materials.add(background_color.into()),
    });

    commands.insert_resource(HalfWindowSize {
        width: window.width() / 2.0,
        height: window.height() / 2.0,
    })
}

fn train_block_spawn(
    mut commands: Commands,
    train_materials: Res<TrainMaterials>,
    ui_tracks: Res<UiTrackPos>,
) {
    println!("train_block_spawn!!");

    let size = Vec2::new(
        train_materials.train_block_size,
        train_materials.train_block_size,
    );

    let track_distance = 5;
    let all_tracks: Vec<Arc<Mutex<Track>>> = (1..14)
        .map(|i| {
            Arc::new(Mutex::new(Track::new(
                format!("L{}", i).to_string(),
                track_distance,
            )))
        })
        .collect();

    let tracks = all_tracks.clone();
    let green_state_mutex = Arc::new(Mutex::new(TrackState::L2));
    let purple_state_mutex = Arc::new(Mutex::new(TrackState::L7));
    let red_state_mutex = Arc::new(Mutex::new(TrackState::L8));
    let blue_state_mutex = Arc::new(Mutex::new(TrackState::L13));

    let trains = vec![
        Arc::new(Mutex::new(Train::new(0, 2))),
        Arc::new(Mutex::new(Train::new(1, 3))),
        Arc::new(Mutex::new(Train::new(2, 3))),
        Arc::new(Mutex::new(Train::new(3, 4))),
    ];

    let green_train = trains[0].clone();
    commands
        .spawn_bundle(SpriteBundle {
            material: train_materials.green_train_material.clone(),
            transform: Transform {
                translation: ui_tracks.track_pos[TrackState::L2 as usize].0.clone(),
                ..Default::default()
            },
            sprite: Sprite::new(size),
            ..Default::default()
        })
        .insert(GreenTrainID)
        .insert(TrainState {
            state: green_state_mutex.clone(),
        })
        .insert(std::thread::spawn(move || {
            let circuit = GreenCircuit::new(tracks);

            loop {
                let train;
                {
                    train = green_train.lock().unwrap().clone();
                }

                circuit.run(green_state_mutex.clone(), &train);
            }
        }));

    let tracks = all_tracks.clone();
    let purple_train = trains[1].clone();
    commands
        .spawn_bundle(SpriteBundle {
            material: train_materials.purple_train_material.clone(),
            transform: Transform {
                translation: ui_tracks.track_pos[TrackState::L7 as usize].0.clone(),
                ..Default::default()
            },
            sprite: Sprite::new(size),
            ..Default::default()
        })
        .insert(PurpleTrainID)
        .insert(TrainState {
            state: purple_state_mutex.clone(),
        })
        .insert(std::thread::spawn(move || {
            let circuit = PurpleCircuit::new(tracks);
            loop {
                let train;
                {
                    train = purple_train.lock().unwrap().clone();
                }
                circuit.run(purple_state_mutex.clone(), &train);
            }
        }));

    let tracks = all_tracks.clone();
    let red_train = trains[2].clone();
    commands
        .spawn_bundle(SpriteBundle {
            material: train_materials.red_train_material.clone(),
            transform: Transform {
                translation: ui_tracks.track_pos[TrackState::L8 as usize].0.clone(),
                ..Default::default()
            },
            sprite: Sprite::new(size),
            ..Default::default()
        })
        .insert(RedTrainID)
        .insert(TrainState {
            state: red_state_mutex.clone(),
        })
        .insert(std::thread::spawn(move || {
            let circuit = RedCircuit::new(tracks);

            loop {
                let train;
                {
                    train = red_train.lock().unwrap().clone();
                }
                circuit.run(red_state_mutex.clone(), &train)
            }
        }));

    let tracks = all_tracks.clone();
    let blue_train = trains[3].clone();
    commands
        .spawn_bundle(SpriteBundle {
            material: train_materials.blue_train_material.clone(),
            transform: Transform {
                translation: ui_tracks.track_pos[TrackState::L12 as usize].0.clone(),
                ..Default::default()
            },
            sprite: Sprite::new(size),
            ..Default::default()
        })
        .insert(BlueTrainID)
        .insert(TrainState {
            state: blue_state_mutex.clone(),
        })
        .insert(std::thread::spawn(move || {
            let circuit = BlueCircuit::new(tracks);

            loop {
                let train;
                {
                    train = blue_train.lock().unwrap().clone();
                }

                circuit.run(blue_state_mutex.clone(), &train);
            }
        }));

    commands.insert_resource(trains);
}

fn spawn_track_entities(
    mut commands: Commands,
    track_materials: Res<TrackMaterials>,
    train_materials: Res<TrainMaterials>,
    half_window_size: Res<HalfWindowSize>,
) {
    println!("spawn_track_entities!!");

    let train_block_size = train_materials.train_block_size;
    let contour_size = 30.0;
    let blue_track_size = Vec2::new(
        7.0 * half_window_size.width / 5.0,
        2.0 * half_window_size.height / 3.0,
    );
    let blue_track_pos = Vec3::new(
        -train_block_size - blue_track_size.x / 2.0 + half_window_size.width,
        -half_window_size.height + blue_track_size.y / 2.0 + half_window_size.height / 5.0,
        10.0,
    );
    commands
        .spawn_bundle(SpriteBundle {
            material: track_materials.background_material.clone(),
            transform: Transform {
                translation: blue_track_pos,
                ..Default::default()
            },
            sprite: Sprite::new(Vec2::new(
                blue_track_size.x - contour_size,
                blue_track_size.y - contour_size,
            )),
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn_bundle(SpriteBundle {
                material: track_materials.blue_track_material.clone(),
                sprite: Sprite::new(blue_track_size),
                ..Default::default()
            });
        });

    let material_tracks = vec![
        track_materials.green_track_material.clone(),
        track_materials.purple_track_material.clone(),
        track_materials.red_track_material.clone(),
    ];

    let off_set = 50.0;
    let small_track_size = Vec2::new((blue_track_size.x - 2.0 * off_set) / 3.0, blue_track_size.y);
    let mut pos = Vec3::new(-blue_track_pos.x - contour_size - 1.0, 100.0, 10.0);
    let ui_l1_pos = Vec3::new(pos.x - 0.75 * small_track_size.x / 2.0, pos.y, 100.0);
    let ui_l2_pos = Vec3::new(pos.x, pos.y + 1.10 * small_track_size.y / 2.0, 100.0);
    let ui_l3_pos = Vec3::new(ui_l1_pos.x + 0.97 * small_track_size.x, pos.y, 100.0); //
    let ui_l4_pos = Vec3::new(pos.x, pos.y - 1.10 * small_track_size.y / 2.0, 100.0); //
    let ui_l5_pos = Vec3::new(ui_l3_pos.x + small_track_size.x + off_set, pos.y, 100.0);
    let ui_l6_pos = Vec3::new(
        ui_l2_pos.x + small_track_size.x + off_set,
        pos.y - 1.10 * small_track_size.y / 2.0,
        100.0,
    );
    let ui_l7_pos = Vec3::new(
        ui_l2_pos.x + small_track_size.x + off_set,
        pos.y + 1.10 * small_track_size.y / 2.0,
        100.0,
    );
    let ui_l8_pos = Vec3::new(
        ui_l7_pos.x + small_track_size.x + off_set,
        pos.y + 1.10 * small_track_size.y / 2.0,
        100.0,
    );
    let ui_l9_pos = Vec3::new(ui_l5_pos.x + small_track_size.x + off_set, pos.y, 100.0);
    let ui_l10_pos = Vec3::new(
        ui_l7_pos.x + small_track_size.x + off_set,
        pos.y - 1.10 * small_track_size.y / 2.0,
        100.0,
    );
    let ui_l11_pos = Vec3::new(
        blue_track_pos.x - 0.90 * blue_track_size.x / 2.0,
        blue_track_pos.y,
        100.0,
    );
    let ui_l12_pos = Vec3::new(
        blue_track_pos.x + 0.90 * blue_track_size.x / 2.0,
        blue_track_pos.y,
        100.0,
    );
    let ui_l13_pos = Vec3::new(
        blue_track_pos.x,
        blue_track_pos.y - 0.75 * blue_track_size.y / 2.0,
        100.0,
    );

    for material in material_tracks {
        commands
            .spawn_bundle(SpriteBundle {
                material: track_materials.background_material.clone(),
                transform: Transform {
                    translation: pos,
                    ..Default::default()
                },
                sprite: Sprite::new(Vec2::new(
                    small_track_size.x - contour_size,
                    small_track_size.y - contour_size,
                )),
                ..Default::default()
            })
            .with_children(|parent| {
                parent.spawn_bundle(SpriteBundle {
                    material,
                    sprite: Sprite::new(small_track_size),
                    ..Default::default()
                });
            });
        pos = Vec3::new(pos.x + small_track_size.x + off_set, pos.y, 10.0);
    }

    let ui_tracks_pos = vec![
        (ui_l1_pos, "L1".to_string()),
        (ui_l2_pos, "L2".to_string()),
        (ui_l3_pos, "L3".to_string()),
        (ui_l4_pos, "L4".to_string()),
        (ui_l5_pos, "L5".to_string()),
        (ui_l6_pos, "L6".to_string()),
        (ui_l7_pos, "L7".to_string()),
        (ui_l8_pos, "L8".to_string()),
        (ui_l9_pos, "L9".to_string()),
        (ui_l10_pos, "L10".to_string()),
        (ui_l11_pos, "L11".to_string()),
        (ui_l12_pos, "L12".to_string()),
        (ui_l13_pos, "L13".to_string()),
    ];

    commands.insert_resource(UiTrackPos {
        track_pos: ui_tracks_pos,
        center_blue_track: Vec2::new(blue_track_pos.x, blue_track_pos.y),
    });
}

fn spawn_text_entities(
    mut commands: Commands,
    ui_tracks: Res<UiTrackPos>,
    asset_server: Res<AssetServer>,
) {
    println!("spawn_text !!");

    let track_pos = &ui_tracks.track_pos;

    for (pos, text_value) in track_pos {
        commands.spawn_bundle(Text2dBundle {
            transform: Transform {
                translation: pos.clone(),
                ..Default::default()
            },
            text: Text::with_section(
                text_value.clone(),
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 45.0,
                    color: Color::BLACK,
                },
                TextAlignment {
                    vertical: VerticalAlign::Center,
                    horizontal: HorizontalAlign::Center,
                },
            ),
            ..Default::default()
        });
    }

    commands.spawn_bundle(Text2dBundle {
        transform: Transform {
            translation: Vec3::new(
                ui_tracks.center_blue_track.x,
                -ui_tracks.center_blue_track.y + 150.0,
                100.0,
            ),
            ..Default::default()
        },
        text: Text::with_section(
            "Visualização dinâmica dos trens",
            TextStyle {
                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                font_size: 60.0,
                color: Color::BLACK,
            },
            TextAlignment {
                vertical: VerticalAlign::Center,
                horizontal: HorizontalAlign::Center,
            },
        ),
        ..Default::default()
    });
}

fn spawn_button_entities(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    train_materials: Res<TrainMaterials>,
) {
    println!("spawn_button_entities");

    let velocity_texts = vec!["Velocidade ++".to_string(), "Velocidade --".to_string()];

    let train_colors = vec![
        train_materials.green_train_material.clone(),
        train_materials.purple_train_material.clone(),
        train_materials.red_train_material.clone(),
        train_materials.blue_train_material.clone(),
    ];

    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                margin: Rect::all(Val::Percent(0.5)),
                size: Size::new(Val::Percent(25.0), Val::Percent(100.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,

                display: Display::Flex,
                flex_wrap: FlexWrap::Wrap,
                ..Default::default()
            },
            visible: Visible {
                is_visible: false,
                is_transparent: true,
            },
            ..Default::default()
        })
        .with_children(|root| {
            for (color, train_id) in train_colors.iter().zip([
                TrainID::GREEN,
                TrainID::PURPLE,
                TrainID::RED,
                TrainID::BLUE,
            ]) {
                for (text, button_action) in velocity_texts
                    .iter()
                    .zip([ButtonAction::INCREMENT, ButtonAction::DECREMENT])
                {
                    root.spawn_bundle(ButtonBundle {
                        style: Style {
                            size: Size::new(Val::Px(150.0), Val::Px(65.0)),
                            // center button
                            margin: Rect::all(Val::Percent(1.0)),
                            // align_self: AlignSelf::Center,
                            // horizontally center child text
                            justify_content: JustifyContent::Center,
                            // vertically center child text
                            align_items: AlignItems::Center,
                            ..Default::default()
                        },
                        material: color.clone(),
                        ..Default::default()
                    })
                    .insert(train_id.clone())
                    .insert(button_action)
                    .with_children(|parent| {
                        parent.spawn_bundle(TextBundle {
                            text: Text::with_section(
                                text,
                                TextStyle {
                                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                    font_size: 20.0,
                                    color: Color::rgb(0.9, 0.9, 0.9),
                                },
                                Default::default(),
                            ),
                            ..Default::default()
                        });
                    });
                }
            }
        });
}
fn green_train_update(
    mut green_query: Query<(&TrainState, &mut Transform, With<GreenTrainID>)>,

    ui_tracks: Res<UiTrackPos>,
) {
    let result = green_query.single_mut();

    if let Ok((train_state, mut transform, _)) = result {
        if let Ok(mutex) = train_state.state.try_lock() {
            let state = mutex.clone();
            transform.translation = ui_tracks.track_pos[state as usize].0.clone();
        }
    }
}

fn purple_train_update(
    mut query: Query<(&TrainState, &mut Transform, With<PurpleTrainID>)>,
    ui_tracks: Res<UiTrackPos>,
) {
    let result = query.single_mut();

    if let Ok((train_state, mut transform, _)) = result {
        if let Ok(mutex) = train_state.state.try_lock() {
            let state = mutex.clone();
            transform.translation = ui_tracks.track_pos[state as usize].0.clone();
        }
    }
}

fn red_train_update(
    mut query: Query<(&TrainState, &mut Transform, With<RedTrainID>)>,
    ui_tracks: Res<UiTrackPos>,
) {
    let result = query.single_mut();

    if let Ok((train_state, mut transform, _)) = result {
        if let Ok(mutex) = train_state.state.try_lock() {
            let state = mutex.clone();
            transform.translation = ui_tracks.track_pos[state as usize].0.clone();
        }
    }
}

fn blue_train_update(
    mut query: Query<(&TrainState, &mut Transform, With<BlueTrainID>)>,
    ui_tracks: Res<UiTrackPos>,
) {
    let result = query.single_mut();

    if let Ok((train_state, mut transform, _)) = result {
        if let Ok(mutex) = train_state.state.try_lock() {
            let state = mutex.clone();
            transform.translation = ui_tracks.track_pos[state as usize].0.clone();
        }
    }
}


fn press_button(
    mut query: Query<
        (
            &Interaction,
            &mut Handle<ColorMaterial>,
            &Children,
            &ButtonAction,
            &TrainID,
        ),
        (Changed<Interaction>, With<Button>),
    >,
    trains: Res<Vec<Arc<Mutex<Train>>>>,
    train_materials: Res<TrainMaterials>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    query.for_each_mut(|result| {

        let (interaction, mut material, childred, button_action, train_id) = result;

    match *interaction {
        Interaction::Clicked => {
            *material = materials.add(Color::rgb(0.35, 0.75, 0.35).into());
        }
        // Interaction::Hovered => todo!(),
        // Interaction::None => todo!(),
        Interaction::None => {
            match train_id {
                TrainID::GREEN => *material = train_materials.green_train_material.clone(),
                TrainID::PURPLE => *material = train_materials.purple_train_material.clone(),
                TrainID::RED => *material = train_materials.red_train_material.clone(),
                TrainID::BLUE => *material = train_materials.blue_train_material.clone(),
            }
            return;
        }
        _ => {
            return;
        }
    }

    let mut train = match train_id {
        TrainID::GREEN => trains[0].lock().unwrap(),
        TrainID::PURPLE => trains[1].lock().unwrap(),
        TrainID::RED => trains[2].lock().unwrap(),
        TrainID::BLUE => trains[3].lock().unwrap(),
    };

    match button_action {
        ButtonAction::INCREMENT => train.increment(),
        ButtonAction::DECREMENT => train.decrement(),
    };
       
    });

    
}
