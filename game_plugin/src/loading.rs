mod paths;

use crate::loading::paths::PATHS;
use crate::GameState;
use bevy::asset::LoadState;
use bevy::prelude::*;
use bevy_kira_audio::AudioSource;

// TODO: refactor this file. This is error prone, you must remember to start loading the textures in one method and insert them in another (here)

pub struct LoadingPlugin;

impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_set(
            SystemSet::on_enter(GameState::Loading).with_system(start_loading.system()),
        )
        .add_system_set(SystemSet::on_update(GameState::Loading).with_system(check_state.system()));
    }
}

pub struct LoadingState {
    textures: Vec<HandleUntyped>,
    fonts: Vec<HandleUntyped>,
    audio: Vec<HandleUntyped>,
}

pub struct FontAssets {
    pub fira_sans: Handle<Font>,
}

pub struct AudioAssets {
    pub birds: Handle<AudioSource>,
}

pub struct Materials {
    pub tile: Handle<ColorMaterial>,
    pub tree: Handle<ColorMaterial>,
    pub wood_logs: Handle<ColorMaterial>,
    pub stockpile: Handle<ColorMaterial>,
    pub man: Handle<ColorMaterial>,
    pub shadow: Handle<ColorMaterial>,
    pub house: Handle<ColorMaterial>,
}

fn start_loading(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut fonts: Vec<HandleUntyped> = vec![];
    fonts.push(asset_server.load_untyped(PATHS.fira_sans));

    let mut audio: Vec<HandleUntyped> = vec![];
    audio.push(asset_server.load_untyped(PATHS.audio_birds));

    let mut textures: Vec<HandleUntyped> = vec![];
    textures.push(asset_server.load_untyped(PATHS.texture_tree));
    textures.push(asset_server.load_untyped(PATHS.texture_wood_logs));
    textures.push(asset_server.load_untyped(PATHS.texture_house));
    textures.push(asset_server.load_untyped(PATHS.texture_man));
    textures.push(asset_server.load_untyped(PATHS.texture_grad_shadow));
    textures.push(asset_server.load_untyped(PATHS.texture_stockpile));

    commands.insert_resource(LoadingState {
        textures,
        fonts,
        audio,
    });
}

fn check_state(
    mut commands: Commands,
    mut state: ResMut<State<GameState>>,
    asset_server: Res<AssetServer>,
    loading_state: Res<LoadingState>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    if LoadState::Loaded
        != asset_server.get_group_load_state(loading_state.fonts.iter().map(|handle| handle.id))
    {
        return;
    }
    if LoadState::Loaded
        != asset_server.get_group_load_state(loading_state.textures.iter().map(|handle| handle.id))
    {
        return;
    }
    if LoadState::Loaded
        != asset_server.get_group_load_state(loading_state.audio.iter().map(|handle| handle.id))
    {
        return;
    }

    commands.insert_resource(FontAssets {
        fira_sans: asset_server.get_handle(PATHS.fira_sans),
    });

    commands.insert_resource(AudioAssets {
        birds: asset_server.get_handle(PATHS.audio_birds),
    });

    commands.insert_resource(Materials {
        tile: materials.add(Color::rgb(0.5, 0.78, 0.52).into()),
        man: materials.add(asset_server.get_handle(PATHS.texture_man).clone().into()),
        house: materials.add(asset_server.get_handle(PATHS.texture_house).clone().into()),
        tree: materials.add(asset_server.get_handle(PATHS.texture_tree).clone().into()),
        wood_logs: materials.add(
            asset_server
                .get_handle(PATHS.texture_wood_logs)
                .clone()
                .into(),
        ),
        shadow: materials.add(
            asset_server
                .get_handle(PATHS.texture_grad_shadow)
                .clone()
                .into(),
        ),
        stockpile: materials.add(
            asset_server
                .get_handle(PATHS.texture_stockpile)
                .clone()
                .into(),
        ),
    });

    state.set(GameState::Menu).unwrap();
}
