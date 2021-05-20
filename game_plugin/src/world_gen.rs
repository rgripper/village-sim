use crate::{
    creature::Creature,
    layers::{OBJECT_LAYER, SHADOW_LAYER, TILE_LAYER},
    loading::Materials,
    plants::{get_scale_from_tree_size, PlantSize, Seeder},
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

fn generate_world(mut commands: Commands, sim_params: Res<SimParams>, materials: Res<Materials>) {
    let (world_columns, world_rows) = sim_params
        .hexagon_builder
        .get_world_columns_rows(sim_params.world_rect.size.x, sim_params.world_rect.size.y);

    create_land_grid(
        &mut commands,
        &materials.tile,
        &sim_params.hexagon_builder,
        &sim_params.world_rect,
        world_columns,
        world_rows,
    );

    let rng = &mut rand::thread_rng();

    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    for _ in 0..36 {
        let tree_pos = gen_in_rect(rng, &sim_params.world_rect);
        gen_tree(
            tree_pos,
            rng.gen_range(0.0..1.0),
            &sim_params.world_rect,
            &mut commands,
            &materials.tree,
            &materials.shadow,
        );
    }

    let villager_start_rect = Rectangle {
        position: sim_params.start_pos,
        size: Vec2::new(100., 100.),
    };
    for _ in 0..8 {
        let villager_pos = gen_in_rect(rng, &villager_start_rect);
        gen_villager(&mut commands, &materials, &villager_pos, &sim_params);
    }
}

fn spawn_sprite_bundles(
    commands: &mut Commands,
    bounding_box: Vec3,
    main_material: Handle<ColorMaterial>,
    shadow: Handle<ColorMaterial>,
    world_size: Vec2,
) {
    // TODO: return here

    commands
        .spawn_bundle(SpriteBundle {
            transform: Transform::from_translation(Vec3::new(
                bounding_box.x,
                bounding_box.y,
                OBJECT_LAYER + world_size.y - bounding_box.z,
            )),
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn_bundle(SpriteBundle {
                material: main_material.clone(),
                transform: Transform::from_translation(
                    Vec2::new(0.0, bounding_box.z / 2.0).extend(OBJECT_LAYER),
                ),
                sprite: Sprite::new(Vec2::new(bounding_box.x, bounding_box.y)),
                ..Default::default()
            });
            parent.spawn_bundle(SpriteBundle {
                transform: Transform::from_translation(Vec2::new(0.0, 0.0).extend(SHADOW_LAYER)),
                sprite: Sprite::new(Vec2::new(bounding_box.x, bounding_box.y / 2.0)),
                material: shadow.clone(),
                ..Default::default()
            });
        })
}

fn gen_villager(
    commands: &mut Commands,
    materials: &Res<Materials>,
    villager_pos: &Vec2,
    sim_params: &Res<SimParams>,
) {
    let bounding_box = Vec3::new(16.0, 16.0, 16.0);
    spawn_sprite_bundles(
        commands,
        bounding_box,
        materials.man.clone(),
        materials.shadow.clone(),
        sim_params.world_rect.size,
    )
    .insert(Creature);
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
    tree_material: &Handle<ColorMaterial>,
    shadow_material: &Handle<ColorMaterial>,
) {
    let plant_size = PlantSize {
        current: init_plant_size,
        max: 1.0,
    };
    let mut transform =
        Transform::from_translation(tree_pos.extend(OBJECT_LAYER + world_rect.size.y - tree_pos.y));
    transform.scale = get_scale_from_tree_size(&plant_size);

    commands
        .spawn_bundle(SpriteBundle {
            transform,
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn_bundle(SpriteBundle {
                material: tree_material.clone(),
                transform: Transform::from_translation(Vec2::new(0.0, 0.0).extend(OBJECT_LAYER)),
                sprite: Sprite::new(Vec2::new(24., 96.)),
                ..Default::default()
            });
            // parent is a ChildBuilder, which has a similar API to Commands
            parent.spawn_bundle(SpriteBundle {
                transform: Transform::from_translation(Vec2::new(0.0, 0.0).extend(SHADOW_LAYER)),
                sprite: Sprite::new(Vec2::new(48., 24.)),
                material: shadow_material.clone(),
                ..Default::default()
            });
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
    tile_material: &Handle<ColorMaterial>,
    hexagon_builder: &HexagonBuilder,
    world_rect: &Rectangle,
    world_columns: i32,
    world_rows: i32,
) {
    let origin = world_rect.size / 2.0;

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
                    transform: Transform::from_translation((rect.position).extend(TILE_LAYER)),
                    sprite: Sprite::new(rect.size),
                    ..Default::default()
                })
                .insert(tile);
        })
}
