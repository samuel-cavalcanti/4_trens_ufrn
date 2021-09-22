use bevy::prelude::*;

use crate::UiTrackPos;

pub struct TextPlugin;

impl Plugin for TextPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_stage(
            "spawn_text_entities",
            SystemStage::single(spawn_text_entities.system()),
        );
    }
}


fn spawn_text_entities(
    mut commands: Commands,
    ui_tracks: Res<UiTrackPos>,
    asset_server: Res<AssetServer>,
) {
    println!("spawn_text !!");

    let track_pos = &ui_tracks.track_pos;

    for (pos, text_value) in track_pos {
        //  let translation = Vec3::new(pos.x,pos.y,100.0);
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
