use std::sync::{Arc, Mutex};

use bevy::prelude::*;
use tracks::Track;

use crate::{HalfWindowSize, TrackMaterials, TrainMaterials, UiTrackPos};
pub struct SetupPlugin;

impl Plugin for SetupPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup).add_startup_stage(
            "spawn_track_entities",
            SystemStage::single(spawn_track_entities),
        );
    }
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

fn spawn_track_entities(
    mut commands: Commands,
    materials: Res<Assets<ColorMaterial>>,
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

    let background_color = materials
        .get(track_materials.background_material.clone())
        .clone()
        .unwrap()
        .color;
    // let color = ;
    commands
        .spawn_bundle(SpriteBundle {
            // material: track_materials.background_material.clone(),
            transform: Transform {
                translation: blue_track_pos,
                ..Default::default()
            },
            sprite: Sprite {
                custom_size: Some(Vec2::new(
                    blue_track_size.x - contour_size,
                    blue_track_size.y - contour_size,
                )),
                color: background_color.clone(),
                ..Default::default()
            },
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn_bundle(SpriteBundle {
                // material: track_materials.blue_track_material.clone(),
                sprite: Sprite {
                    custom_size: Some(blue_track_size),
                    color: materials
                        .get(track_materials.blue_track_material.clone())
                        .clone()
                        .unwrap()
                        .color,
                    ..Default::default()
                },
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
                transform: Transform {
                    translation: pos,
                    ..Default::default()
                },
                sprite: Sprite {
                    custom_size: Some(Vec2::new(
                        small_track_size.x - contour_size,
                        small_track_size.y - contour_size,
                    )),
                    color: background_color.clone(),
                    ..Default::default()
                },
                ..Default::default()
            })
            .with_children(|parent| {
                parent.spawn_bundle(SpriteBundle {
                    sprite: Sprite {
                        custom_size: Some(small_track_size),
                        color: materials.get(material.clone()).unwrap().color,
                        ..Default::default()
                    },
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

    let tracks: Vec<Arc<Mutex<Track>>> = (1..14)
        .map(|i| Arc::new(Mutex::new(Track::new(format!("L{}", i).to_string(), 5))))
        .collect();

    commands.insert_resource(tracks);
}
