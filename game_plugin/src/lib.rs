mod actions;
mod audio;
mod behaviour;
mod buildings;
mod creatures;
mod hexagon;
mod land_grid;
mod layers;
mod loading;
mod menu;
mod physics;
mod plants;
mod random_names;
mod residence;
mod sprite_helpers;
mod time_cycle;
mod tree_cutting;
mod village;
mod world_gen;

use crate::actions::ActionsPlugin;
use crate::audio::InternalAudioPlugin;
use crate::loading::LoadingPlugin;
use crate::menu::MenuPlugin;

use behaviour::MovementPlugin;
use bevy::app::AppBuilder;
// use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;
use plants::PlantLifePlugin;
use residence::ResidencePlugin;
use time_cycle::TimeCyclePlugin;
use tree_cutting::TaskQuePlugin;
use village::VillagePlugin;
use world_gen::{ExperimentalPlugin, WorldGenPlugin};

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
            .add_plugin(TimeCyclePlugin)
            .add_plugin(LoadingPlugin)
            .add_plugin(ActionsPlugin)
            .add_plugin(MenuPlugin)
            .add_plugin(InternalAudioPlugin)
            .add_plugin(VillagePlugin)
            .add_plugin(ResidencePlugin)
            .add_plugin(PlantLifePlugin)
            .add_plugin(WorldGenPlugin)
            .add_plugin(MovementPlugin)
            .add_plugin(TaskQuePlugin)
            .add_plugin(ExperimentalPlugin)
            // .add_plugin(FrameTimeDiagnosticsPlugin::default())
            // .add_plugin(LogDiagnosticsPlugin::default())
            ;
    }
}
