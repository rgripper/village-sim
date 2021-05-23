mod actions;
mod audio;
mod hexagon;
mod land_grid;
mod layers;
mod loading;
mod menu;
mod plants;
mod sprite_helpers;
mod village;
mod world_gen;

use crate::actions::ActionsPlugin;
use crate::audio::InternalAudioPlugin;
use crate::loading::LoadingPlugin;
use crate::menu::MenuPlugin;

use bevy::app::AppBuilder;
// use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;
use plants::PlantLifePlugin;
use world_gen::WorldGenPlugin;

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
enum GameState {
    Loading,
    Playing,
    Menu,
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_state(GameState::Loading)
            .add_plugin(LoadingPlugin)
            .add_plugin(ActionsPlugin)
            .add_plugin(MenuPlugin)
            .add_plugin(InternalAudioPlugin)
            .add_plugin(WorldGenPlugin)
            .add_plugin(PlantLifePlugin)

            // .add_plugin(FrameTimeDiagnosticsPlugin::default())
            // .add_plugin(LogDiagnosticsPlugin::default())
            ;
    }
}
