use crate::loading::TextureAssets;
use crate::{hexagon::HexagonBuilder, tree::Tree};
use crate::{hexagon::Rectangle, land_grid::LandTile, GameState};
use bevy::prelude::*;
use rand::{prelude::ThreadRng, Rng};

pub struct WorldGenPlugin;

impl Plugin for WorldGenPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_set(
            SystemSet::on_enter(GameState::Playing).with_system(generate_world.system()),
        );
        // .add_system_set(
        //     SystemSet::on_exit(GameState::Playing).with_system(remove_world.system()),
        // );
    }
}

fn generate_world(
    mut commands: Commands,
    textures: Res<TextureAssets>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let hexagon_size = 30.0;
    let world_columns = 9;
    let world_rows = 27;

    let hexagon_builder = HexagonBuilder::new(hexagon_size);
    let world_rect = hexagon_builder.get_world_rect(world_columns, world_rows);

    create_land_grid(
        &mut commands,
        &textures,
        &mut materials,
        &hexagon_builder,
        &world_rect,
        world_columns,
        world_rows,
    );

    let mut rng = rand::thread_rng();
    let generate_tree = |rng: &mut ThreadRng, world_rect: &Rectangle| {
        (
            rng.gen_range(world_rect.position.x..world_rect.position.x.abs()),
            rng.gen_range(world_rect.position.y..world_rect.position.y.abs()),
        )
    };

    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    for (x, y) in (0..36).map(|_| generate_tree(&mut rng, &world_rect)) {
        commands
            .spawn_bundle(SpriteBundle {
                material: materials.add(textures.texture_tree.clone().into()),
                transform: Transform::from_translation(Vec3::new(x, y, 1.)),
                sprite: Sprite::new(Vec2::new(12., 24.)),
                ..Default::default()
            })
            .insert(Tree);
    }
}

fn create_land_grid(
    commands: &mut Commands,
    textures: &Res<TextureAssets>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    hexagon_builder: &HexagonBuilder,
    world_rect: &Rectangle,
    world_columns: i32,
    world_rows: i32,
) {
    (0..world_columns * world_rows)
        .map(|i| LandTile {
            column: i.rem_euclid(world_columns),
            row: i / world_columns,
        })
        .for_each(|tile| {
            let hexagon =
                hexagon_builder.get_hexagon_at(world_rect.position, tile.column, tile.row);
            let rect = hexagon.get_bounding_rectangle();
            commands
                .spawn_bundle(SpriteBundle {
                    material: materials.add(Color::rgb(0.5, 0.78, 0.52).into()),
                    transform: Transform::from_translation(rect.position.extend(1.)),
                    sprite: Sprite::new(rect.size),
                    ..Default::default()
                })
                .insert(tile);
        })
}

// fn remove_land_grid(mut commands: Commands, land_grid_query: Query<Entity, With<LandGrid>>) {
//     for land_grid in land_grid_query.iter() {
//         commands.entity(land_grid).despawn();
//     }
// }
