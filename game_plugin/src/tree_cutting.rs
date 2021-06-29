use std::collections::VecDeque;

use bevy::{app::Events, prelude::*};

use crate::{
    actions::Actions,
    behaviour::{CheckTaskEvent, Task, TravelToPosition, TravelToTarget},
    creatures::Creature,
    hexagon::Rectangle,
    physics::PhysicalObject,
    world_gen::{gen_in_rect, SimParams},
    GameState,
};

pub struct CuttingTree {
    pub tree_id: Entity,
}

pub struct ResourceCarrier {
    pub wood: f32,
    pub max_wood: f32,
}

pub struct ResourceStorage {
    pub wood: f32,
}

pub struct TaskQuePlugin;

impl Plugin for TaskQuePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_set(
            SystemSet::on_update(GameState::Playing)
                .with_system(check_tasks.system())
                .with_system(assign_intent.system()),
        );
    }
}

pub fn check_tasks(
    mut commands: Commands,
    sim_params: Res<SimParams>,
    physical_object_query: Query<&PhysicalObject>,
    physical_object_id_query: Query<Entity, With<PhysicalObject>>,
    mut resource_carrier_query: Query<&mut ResourceCarrier>,
    mut resource_storage_query: Query<(&mut ResourceStorage, Entity)>,
    mut tasks_queries: QuerySet<(Query<&mut TaskQue>, Query<Entity, Changed<TaskQue>>)>,
    mut task_events: ResMut<Events<CheckTaskEvent>>,
) {
    let creature_ids: Vec<Entity> = tasks_queries
        .q1()
        .iter()
        .chain(
            task_events
                .drain()
                .map(|CheckTaskEvent(creature_id)| creature_id),
        )
        .collect();
    println!("Something was updated for {}", creature_ids.len());

    for creature_id in creature_ids {
        if let Ok(mut task_que_component) = tasks_queries.q0_mut().get_mut(creature_id) {
            let tasks = &mut task_que_component.0;
            if let Some(task) = tasks.front() {
                println!("Checking task {:?} of {:?}", tasks, creature_id);

                if try_accomplish_task(
                    &mut commands,
                    &sim_params.world_rect,
                    &physical_object_query,
                    &physical_object_id_query,
                    &mut resource_carrier_query,
                    &mut resource_storage_query,
                    &creature_id,
                    &task,
                ) {
                    tasks.pop_front();
                    println!("Task que is now {:?} for creature {:?}", tasks, creature_id);

                    task_events.send(CheckTaskEvent(creature_id));
                }
            }
        }
    }
}

pub fn assign_intent(
    mut commands: Commands,
    creature_without_intent_query: Query<Entity, (With<Creature>, Without<Task>)>,
) {
    for (creature_id) in creature_without_intent_query.iter() {
        commands.entity(creature_id).insert(Task::WanderAimlessly);
    }
}

// #  village manager -> Village needs more wood
// - cut some wood
// - - move to a tree
// - - cut -> while not full
// - drop off resources
// - notify has no intent (idle)
// ...
// # village manager -> need more wood
// - cut some wood
// - - move to a tree
// - - cut -> while not full
// - drop off resources
// - notify has no intent (idle)
/**
# VManager -> need a house
Villager: set up a construction site
    go to the site
    create a construction site
    {spawn ConstructionSite}
    become idle
VManager: if ConstructionSite has enough resources, {spawn House}
VManager: if ConstructionSite has NOT enough resources,
    if Stockpile has NOT enough resources: post on BBS: [GetResources(Wood, AtLeast(5))]
    if Stockpile has enough resources: post on BBS: [MoveResources(Wood, 5, ConstructionSite)]
Villager: read BBS: deliver as many needed resources as can
    go to the storage
    pick max resources from the needed list
    go to the construction site
    place the resources
    become idle
*/

pub fn try_accomplish_task(
    commands: &mut Commands,
    world_rect: &Rectangle,
    physical_object_query: &Query<&PhysicalObject>,
    physical_object_id_query: &Query<Entity, With<PhysicalObject>>,
    resource_carrier_query: &mut Query<&mut ResourceCarrier>,
    resource_storage_query: &mut Query<(&mut ResourceStorage, Entity)>,
    worker_id: &Entity,
    task: &Task,
) -> bool {
    println!("try_accomplish_task {:?}", task);
    match task {
        Task::CutTree(tree_id) => {
            if physical_object_query.get(*tree_id).is_err() {
                return false; // tree has been cut apparently
            }

            if is_located_near(physical_object_query, worker_id, tree_id, 4.0) {
                println!("At the tree. Ready to cut");

                // TODO:
                // commands
                //     .entity(*worker_id)
                //     .insert(CuttingTree { tree_id: *tree_id });
                // then add TreeCut(f32) on the tree, that reaches 1.0, meaning the tree should fall/be despawned. And a resource shoudl appear (a pile of wood)

                // dummy cutting logic for now
                commands.entity(*tree_id).despawn_recursive();
                return true;
            } else {
                println!("Travel to the tree");

                commands.entity(*worker_id).insert(TravelToTarget {
                    time_to_next_location_check: 0.0,
                    last_target_position: None,
                    target_id: *tree_id,
                });
                return false;
            }
        }
        Task::PickUpWood(amount) => {
            // this is just a dummy, proper logic could be added later
            pick_up_wood(worker_id, resource_carrier_query, *amount);
            return true;
        }
        Task::DropOffResources => {
            let (mut storage, storage_id) = resource_storage_query.single_mut().unwrap();
            if is_located_near(physical_object_query, worker_id, &storage_id, 4.0) {
                // what if storage could not contain more wood/resources
                store_wood(worker_id, &mut storage, resource_carrier_query);
                return true;
            } else {
                commands.entity(*worker_id).insert(TravelToTarget {
                    time_to_next_location_check: 0.0,
                    last_target_position: None,
                    target_id: storage_id,
                });
                return false;
            }
        }
        Task::WanderAimlessly => {
            let rng = &mut rand::thread_rng();
            let position = gen_in_rect(rng, world_rect);
            commands
                .entity(*worker_id)
                .insert(TravelToPosition { position });
            return false;
        }
    }
}

fn store_wood(
    worker_id: &Entity,
    storage: &mut ResourceStorage,
    resource_carrier_query: &mut Query<&mut ResourceCarrier>,
) {
    let mut carrier = resource_carrier_query.get_mut(*worker_id).unwrap();
    (*storage).wood += (*carrier).wood; // TODO: check if storage is full
    (*carrier).wood = 0.0;
}

fn pick_up_wood(
    worker_id: &Entity,
    resource_carrier_query: &mut Query<&mut ResourceCarrier>,
    amount: f32,
) {
    let mut carrier = resource_carrier_query.get_mut(*worker_id).unwrap();
    let amount_can_carry = carrier.max_wood - carrier.wood;
    carrier.wood += amount.min(amount_can_carry);
}

fn is_located_near(
    query: &Query<&PhysicalObject>,
    who_id: &Entity,
    where_id: &Entity,
    within_distance: f32,
) -> bool {
    let _who = query.get(*who_id).unwrap();
    let _where = query.get(*where_id).unwrap();
    (*_who).position.distance((*_where).position) < within_distance
}

pub struct TaskQue(pub VecDeque<Task>);
