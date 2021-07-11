mod paths;

use crate::loading::paths::PATHS;
use crate::GameState;
use bevy::asset::{Asset, HandleId, LoadState};
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
    items: Vec<HandleUntyped>,
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

fn start_loading(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let mut items: Vec<HandleUntyped> = vec![];

    fn track_asset<T: Asset>(
        asset_server: &Res<AssetServer>,
        items: &mut Vec<HandleUntyped>,
        asset_path: &str,
    ) -> Handle<T> {
        let handle = asset_server.load::<T, &str>(asset_path);
        items.push(handle.clone_untyped());
        handle
    }

    commands.insert_resource(FontAssets {
        fira_sans: track_asset(&asset_server, &mut items, PATHS.fira_sans),
    });

    commands.insert_resource(AudioAssets {
        birds: track_asset(&asset_server, &mut items, PATHS.audio_birds),
    });

    commands.insert_resource(Materials {
        tile: materials.add(Color::rgb(0.5, 0.78, 0.52).into()),
        man: materials.add(
            track_asset(&asset_server, &mut items, PATHS.texture_man)
                .clone()
                .into(),
        ),
        house: materials.add(
            track_asset(&asset_server, &mut items, PATHS.texture_house)
                .clone()
                .into(),
        ),
        tree: materials.add(
            track_asset(&asset_server, &mut items, PATHS.texture_tree)
                .clone()
                .into(),
        ),
        wood_logs: materials.add(
            track_asset(&asset_server, &mut items, PATHS.texture_wood_logs)
                .clone()
                .into(),
        ),
        shadow: materials.add(
            track_asset(&asset_server, &mut items, PATHS.texture_grad_shadow)
                .clone()
                .into(),
        ),
        stockpile: materials.add(
            track_asset(&asset_server, &mut items, PATHS.texture_stockpile)
                .clone()
                .into(),
        ),
    });

    commands.insert_resource(LoadingState { items });
}

fn check_state(
    mut state: ResMut<State<GameState>>,
    asset_server: Res<AssetServer>,
    loading_state: Res<LoadingState>,
) {
    if LoadState::Loaded
        != asset_server.get_group_load_state(loading_state.items.iter().map(|x| x.id))
    {
        return;
    }

    state.set(GameState::Menu).unwrap();
}
