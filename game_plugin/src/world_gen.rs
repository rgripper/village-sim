use std::collections::VecDeque;
use std::iter::FromIterator;

use crate::behaviour::{CheckTaskEvent, Task};
use crate::buildings::spawn_stockpile;
use crate::hexagon::HexagonBuilder;
use crate::plants::Tree;
use crate::residence::CreatureJoinedVillageEvent;
use crate::tree_cutting::TaskQue;
use crate::village::LivingSpaceAvailableEvent;
use crate::village::Village;
use crate::{
    audio::Ambience, buildings::spawn_house, layers::TILE_LAYER, loading::Materials,
    plants::spawn_tree, residence::spawn_villager,
};
use crate::{hexagon::Rectangle, land_grid::LandTile, GameState};
use bevy::prelude::*;
use rand::{prelude::ThreadRng, Rng};

pub struct SimParams {
    pub start_pos: Vec2,
    pub hexagon_builder: HexagonBuilder,
    pub world_rect: Rectangle,
}

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
    sim_params: Res<SimParams>,
    materials: Res<Materials>,
    mut ev_creature_joined_village: EventWriter<CreatureJoinedVillageEvent>,
    mut ev_living_space_available: EventWriter<LivingSpaceAvailableEvent>,
    mut ev_creature_available_for_tasks: EventWriter<CreatureAvailableForTasks>,
) {
    commands.spawn().insert(Ambience { is_forest: true });

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
        spawn_tree(
            tree_pos,
            rng.gen_range(0.0..1.0),
            &sim_params.world_rect,
            &mut commands,
            &materials.tree,
            &materials.shadow,
        );
    }

    let village_start_rect = Rectangle {
        position: sim_params.start_pos,
        size: Vec2::new(100., 100.),
    };

    for _ in 0..8 {
        let resident_pos = gen_in_rect(rng, &village_start_rect);
        let new_villager_id = spawn_villager(
            &mut commands,
            &materials,
            resident_pos,
            &sim_params,
            &mut ev_creature_joined_village,
        );

        ev_creature_available_for_tasks.send(CreatureAvailableForTasks(new_villager_id))
    }

    for _ in 0..2 {
        let house_pos = gen_in_rect(rng, &village_start_rect);
        spawn_house(
            &mut commands,
            &materials,
            house_pos,
            &sim_params,
            2,
            &mut ev_living_space_available,
        );
    }

    let stockpile_pos = gen_in_rect(rng, &village_start_rect);
    spawn_stockpile(&mut commands, &materials, stockpile_pos, &sim_params);

    commands.spawn().insert(Village {
        habitants_count: 0,
        homeless_count: 0,
        wood: 0.0,
    });
}

pub fn gen_in_rect(rng: &mut ThreadRng, rect: &Rectangle) -> Vec2 {
    Vec2::new(
        rng.gen_range(rect.position.x - rect.size.x / 2.0..rect.position.x + rect.size.x / 2.0),
        rng.gen_range(rect.position.y - rect.size.y / 2.0..rect.position.y + rect.size.y / 2.0),
    )
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

pub struct ExperimentalPlugin;
impl Plugin for ExperimentalPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_set(SystemSet::on_update(GameState::Playing).with_system(cut_tree.system()))
            .add_event::<CreatureAvailableForTasks>();

        // .add_system_set(
        //     SystemSet::on_exit(GameState::Playing).with_system(remove_world.system()),
        // );
    }
}

pub struct CreatureAvailableForTasks(pub Entity);

fn cut_tree(
    mut villager_tasks_query: Query<&mut TaskQue>,
    tree_query: Query<Entity, With<Tree>>,
    mut ev_creature_available_for_tasks: EventReader<CreatureAvailableForTasks>,
    mut ev_check_task_event: EventWriter<CheckTaskEvent>,
) {
    if let Some(CreatureAvailableForTasks(villager_id)) =
        ev_creature_available_for_tasks.iter().next()
    {
        println!("cut_tree has creatures available for tasks");

        if let Some(tree_id) = tree_query.iter().next() {
            let task_que = &mut villager_tasks_query.get_mut(*villager_id).unwrap().0;
            println!("ready to put a task in a task queue");

            task_que.push_back(Task::CutTree(tree_id));
            task_que.push_back(Task::PickUpWood(5.0));
            task_que.push_back(Task::DropOffResources);
        }
    }
}
