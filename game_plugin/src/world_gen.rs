use crate::{creature::Creature, loading::TextureAssets};
use crate::{hexagon::HexagonBuilder, tree::Tree};
use crate::{hexagon::Rectangle, land_grid::LandTile, GameState};
use bevy::prelude::*;
use rand::{prelude::ThreadRng, Rng};

pub struct WorldGenPlugin;

pub struct SimParams {
    pub start_pos: Vec2,
    pub size: Vec2,
}

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
    sim_params: Res<SimParams>,
) {
    let hexagon_size = 10.0; // probably will be a const

    let hexagon_builder = HexagonBuilder::new(hexagon_size);
    let (world_columns, world_rows) =
        hexagon_builder.get_world_columns_rows(sim_params.size.x, sim_params.size.y);
    let world_rect = hexagon_builder.get_world_rect(world_columns, world_rows);

    create_land_grid(
        &mut commands,
        &mut materials,
        &hexagon_builder,
        &world_rect,
        world_columns,
        world_rows,
    );

    let mut rng = rand::thread_rng();
    let generate_in_rect = |rng: &mut ThreadRng, rect: &Rectangle| {
        (
            rng.gen_range(rect.position.x - rect.size.x / 2.0..rect.position.x + rect.size.x / 2.0),
            rng.gen_range(rect.position.y - rect.size.y / 2.0..rect.position.y + rect.size.y / 2.0),
        )
    };

    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    let tree_material = materials.add(textures.texture_tree.clone().into());
    let man_material = materials.add(textures.texture_man.clone().into());

    for (x, y) in (0..36).map(|_| generate_in_rect(&mut rng, &world_rect)) {
        commands
            .spawn_bundle(SpriteBundle {
                material: tree_material.clone(),
                transform: Transform::from_translation(Vec3::new(x, y, 1.)),
                sprite: Sprite::new(Vec2::new(12., 24.)),
                ..Default::default()
            })
            .insert(Tree);
    }

    let villager_start_rect = Rectangle {
        position: sim_params.start_pos,
        size: Vec2::new(100., 100.),
    };
    for (x, y) in (0..8).map(|_| generate_in_rect(&mut rng, &villager_start_rect)) {
        commands
            .spawn_bundle(SpriteBundle {
                material: man_material.clone(),
                transform: Transform::from_translation(Vec3::new(x, y, 1.)),
                sprite: Sprite::new(Vec2::new(16., 16.)),
                ..Default::default()
            })
            .insert(Creature);
    }
}

fn create_land_grid(
    commands: &mut Commands,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    hexagon_builder: &HexagonBuilder,
    world_rect: &Rectangle,
    world_columns: i32,
    world_rows: i32,
) {
    let origin = world_rect.size / 2.0;
    let tile_material = materials.add(Color::rgb(0.5, 0.78, 0.52).into());
    (0..world_columns * world_rows)
        .map(|i| LandTile {
            column: i.rem_euclid(world_columns),
            row: i / world_columns,
        })
        .for_each(|tile| {
            let hexagon = hexagon_builder.get_hexagon_at(origin, tile.column, tile.row);
            let rect = hexagon.get_bounding_rectangle();
            commands
                .spawn_bundle(SpriteBundle {
                    material: tile_material.clone(),
                    transform: Transform::from_translation((rect.position).extend(1.)),
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
