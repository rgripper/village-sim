// use crate::loading::TextureAssets;
// use crate::GameState;
// use crate::{actions::Actions, hexagon::HexagonBuilder};
// use bevy::prelude::*;
// use rand::{prelude::ThreadRng, Rng};

// pub struct LandGridPlugin;

pub struct LandTile {
    pub column: i32,
    pub row: i32,
}

pub struct LandGrid {
    pub tiles: Vec<LandTile>,
}

// impl Plugin for LandGridPlugin {
//     fn build(&self, app: &mut AppBuilder) {
//         app.add_system_set(
//             SystemSet::on_enter(GameState::Playing).with_system(spawn_land_grid.system()),
//         )
//         .add_system_set(
//             SystemSet::on_exit(GameState::Playing).with_system(remove_land_grid.system()),
//         );
//     }
// }

// fn spawn_land_grid(
//     mut commands: Commands,
//     textures: Res<TextureAssets>,
//     mut materials: ResMut<Assets<ColorMaterial>>,
// ) {
//     let hexagon_size = 30.0;
//     let world_columns = 9;
//     let world_rows = 27;

//     let hexagon_builder = HexagonBuilder::new(hexagon_size);
//     let mut rng = rand::thread_rng();
//     let world_rect = hexagon_builder.get_world_rect(world_columns, world_rows);
//     let generate_tree = |rng: &mut ThreadRng| {
//         (
//             rng.gen_range(0f32..world_rect.width),
//             rng.gen_range(0f32..world_rect.height),
//         )
//     };

//     let land_grid = LandGrid {
//         tiles: (0..world_columns * world_rows)
//             .map(|i| {
//                 LandTile {
//                     column: i.rem_euclid(world_columns),
//                     row: i / world_columns,
//                 }
//                 // hexagon_builder.get_hexagon_at(i.rem_euclid(world_columns), i / world_columns)
//             })
//             .collect(),
//     };

//     commands
//         .spawn_bundle(SpriteBundle {
//             material: materials.add(textures.texture_bevy.clone().into()),
//             transform: Transform::from_translation(Vec3::new(0., 0., 1.)),
//             ..Default::default()
//         })
//         .insert(land_grid);
// }

// fn remove_land_grid(mut commands: Commands, land_grid_query: Query<Entity, With<LandGrid>>) {
//     for land_grid in land_grid_query.iter() {
//         commands.entity(land_grid).despawn();
//     }
// }
