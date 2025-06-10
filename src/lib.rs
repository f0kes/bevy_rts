#![allow(clippy::type_complexity)]

pub mod box_select;
pub mod dudliq;
pub mod loading;
pub mod particles;
pub mod player;
pub mod recolor;
pub mod scene;
pub mod navigation;

use crate::loading::LoadingPlugin;
use crate::player::PlayerPlugin;

use bevy::app::App;
#[cfg(debug_assertions)]
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;
use player::Mode;

#[derive(States, Default, Clone, Eq, PartialEq, Debug, Hash)]
enum GameState {
    // During the loading State the LoadingPlugin will load our assets
    #[default]
    Loading,
    Loaded,
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((LoadingPlugin, PlayerPlugin));

        #[cfg(debug_assertions)]
        {
            app.add_plugins((
                FrameTimeDiagnosticsPlugin,
                LogDiagnosticsPlugin::default(),
            ));
        }
    }
}
