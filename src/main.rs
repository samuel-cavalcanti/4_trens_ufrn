use bevy::{prelude::*, window::PresentMode};
use buttons_ui_plugin::ButtonUiPlugin;
use setup_plugin::SetupPlugin;
use std::sync::{Arc, Mutex};
use text_plugin::TextPlugin;

use tracks::track::TrackState;
use train_plugin::TrainPlugin;

pub mod buttons_ui_plugin;
pub mod setup_plugin;
pub mod text_plugin;
pub mod train_plugin;

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
#[derive(Component)]
pub enum ButtonAction {
    INCREMENT,
    DECREMENT,
}

#[derive(Clone,Component)]
pub enum TrainID {
    GREEN,
    PURPLE,
    RED,
    BLUE,
}
#[derive(Component)]
pub struct TrainState {
    state: Arc<Mutex<TrackState>>,
}

const APP_NAME: &str = "Visualização da dinâmica dos trens";

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(WindowDescriptor {
            title: APP_NAME.to_string(),
            width: 600.0,
            height: 800.0,
            present_mode:PresentMode::Fifo,
            ..Default::default()
        })
        .add_plugin(SetupPlugin)
        .add_plugin(TrainPlugin)
        .add_plugin(TextPlugin)
        .add_plugin(ButtonUiPlugin)
      
        .run();
}
