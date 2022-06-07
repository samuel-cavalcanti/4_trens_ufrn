use std::sync::{Arc, Mutex};

use crate::{ButtonAction, TrainID, TrainMaterials};
use bevy::prelude::*;
use tracks::Train;

pub struct ButtonUiPlugin;

impl Plugin for ButtonUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_stage(
            "spawn_button_entities",
            SystemStage::single(spawn_button_entities),
        )
        .add_system(press_button);
    }
}

fn spawn_button_entities(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    materials: Res<Assets<ColorMaterial>>,
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
            visibility: Visibility { is_visible: false },
            // visible: Visible {
            //     is_visible: false,
            //     is_transparent: true,
            // },
            ..Default::default()
        })
        .with_children(|root| {
            for (color, train_id) in train_colors.iter().zip([
                TrainID::GREEN,
                TrainID::PURPLE,
                TrainID::RED,
                TrainID::BLUE,
            ]) {
                let color_material = materials.get(color.clone()).unwrap();

                for (text, button_action) in velocity_texts
                    .iter()
                    .zip([ButtonAction::INCREMENT, ButtonAction::DECREMENT])
                {
                    root.spawn_bundle(ButtonBundle {
                        style: Style {
                            size: Size::new(Val::Px(150.0), Val::Px(65.0)),
                            margin: Rect::all(Val::Percent(1.0)),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,

                            ..Default::default()
                        },
                        color: UiColor(color_material.color),
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

fn press_button(
    mut query: Query<
        (&Interaction, &mut UiColor, &ButtonAction, &TrainID),
        (Changed<Interaction>, With<Button>),
    >,
    trains: Res<Vec<Arc<Mutex<Train>>>>,
    train_materials: Res<TrainMaterials>,
    materials: ResMut<Assets<ColorMaterial>>,
) {
    query.for_each_mut(|result| {
        let (interaction, mut material, button_action, train_id) = result;

        match *interaction {
            Interaction::Clicked => {
                *material = UiColor(Color::rgb(0.35, 0.75, 0.35));
            }
            Interaction::None => {
                let color_material = match train_id {
                    TrainID::GREEN => train_materials.green_train_material.clone(),
                    TrainID::PURPLE => train_materials.purple_train_material.clone(),
                    TrainID::RED => train_materials.red_train_material.clone(),
                    TrainID::BLUE => train_materials.blue_train_material.clone(),
                };

                let color_material = materials.get(color_material.clone()).unwrap();
                *material = UiColor(color_material.color);
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
