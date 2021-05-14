use std::ops::Range;

use crate::{
    creature::Creature,
    loading::TextureAssets,
    plants::{PlantSize, Seeder},
};
use crate::{hexagon::HexagonBuilder, plants::Tree};
use crate::{hexagon::Rectangle, land_grid::LandTile, GameState};
use bevy::prelude::*;
use rand::{prelude::ThreadRng, Rng};

pub struct WorldGenPlugin;

pub struct SimParams {
    pub start_pos: Vec2,
    pub hexagon_builder: HexagonBuilder,
    pub world_rect: Rectangle,
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
    let (world_columns, world_rows) = sim_params
        .hexagon_builder
        .get_world_columns_rows(sim_params.world_rect.size.x, sim_params.world_rect.size.y);

    create_land_grid(
        &mut commands,
        &mut materials,
        &sim_params.hexagon_builder,
        &sim_params.world_rect,
        world_columns,
        world_rows,
    );

    let rng = &mut rand::thread_rng();

    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    let tree_material = materials.add(textures.texture_tree.clone().into());
    let man_material = materials.add(textures.texture_man.clone().into());

    for _ in 0..36 {
        let tree_pos = gen_in_rect(rng, &sim_params.world_rect);
        gen_tree(
            tree_pos,
            rng.gen_range(0.0..1.0),
            &sim_params.world_rect,
            &mut commands,
            tree_material.clone(),
        );
    }

    let villager_start_rect = Rectangle {
        position: sim_params.start_pos,
        size: Vec2::new(100., 100.),
    };
    for _ in 0..8 {
        let villager_pos = gen_in_rect(rng, &villager_start_rect);
        commands
            .spawn_bundle(SpriteBundle {
                material: man_material.clone(),
                transform: Transform::from_translation(villager_pos.extend(villager_pos.y)),
                sprite: Sprite::new(Vec2::new(16., 16.)),
                ..Default::default()
            })
            .insert(Creature);
    }
}

pub fn gen_in_rect(rng: &mut ThreadRng, rect: &Rectangle) -> Vec2 {
    Vec2::new(
        rng.gen_range(rect.position.x - rect.size.x / 2.0..rect.position.x + rect.size.x / 2.0),
        rng.gen_range(rect.position.y - rect.size.y / 2.0..rect.position.y + rect.size.y / 2.0),
    )
}

pub fn gen_tree(
    tree_pos: Vec2,
    init_plant_size: f32,
    world_rect: &Rectangle,
    commands: &mut Commands,
    tree_material: Handle<ColorMaterial>,
) {
    let plant_size = PlantSize {
        current: init_plant_size,
        max: 1.0,
    };
    let transform = Transform::from_translation(tree_pos.extend(tree_pos.y));
    transform.translation;
    commands
        .spawn_bundle(SpriteBundle {
            material: tree_material.clone(),
            transform,
            sprite: Sprite::new(Vec2::new(12., 48.)),
            ..Default::default()
        })
        .insert(Tree)
        .insert(Seeder {
            seed_growth_per_second: (0.0..1.0),
            seeds_since_last_time: 0.0,
            survival_probability: 0.01,
        })
        .insert(plant_size);
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
                    transform: Transform::from_translation((rect.position).extend(-origin.y)),
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
