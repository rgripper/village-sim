use crate::actions::Actions;
use crate::loading::TextureAssets;
use crate::GameState;
use bevy::prelude::*;

pub struct LandTileGridPlugin;

pub struct LandTile;

pub struct LandTileGrid {
    tiles: Vec<LandTile>,
}

impl Plugin for LandTileGridPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_set(
            SystemSet::on_enter(GameState::Playing).with_system(spawn_tile_grid.system()),
        )
        .add_system_set(
            SystemSet::on_exit(GameState::Playing).with_system(remove_tile_grid.system()),
        );
    }
}

fn spawn_tile_grid(
    mut commands: Commands,
    textures: Res<TextureAssets>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands
        .spawn_bundle(SpriteBundle {
            material: materials.add(textures.texture_bevy.clone().into()),
            transform: Transform::from_translation(Vec3::new(0., 0., 1.)),
            ..Default::default()
        })
        .insert(LandTileGrid);
}

fn remove_tile_grid(
    mut commands: Commands,
    land_tile_grid_query: Query<Entity, With<LandTileGrid>>,
) {
    for land_tile_grid in land_tile_grid_query.iter() {
        commands.entity(land_tile_grid).despawn();
    }
}
