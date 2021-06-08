use bevy::prelude::{Commands, Entity, EventWriter, Query};

use crate::{behaviour::Intent, physics::PhysicalObject};

pub struct TravelToTargetEvent {
    pub creature_id: Entity,
    pub target_id: Entity,
}

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

pub fn act_on_intent(
    mut commands: Commands,
    physical_object_query: Query<&PhysicalObject>,
    mut resource_carrier_query: Query<&mut ResourceCarrier>,
    mut resource_storage_query: Query<(&mut ResourceStorage, Entity)>,
    worker_id: &Entity,
    intent: &Intent,
    mut ev_travel_to_target: EventWriter<TravelToTargetEvent>,
) {
    match intent {
        Intent::CutTree(tree_id) => {
            let (mut storage, storage_id) = resource_storage_query.single_mut().unwrap();
            if !can_carry_more_wood(worker_id, &mut resource_carrier_query) {
                // what was interrupted, and not taken as much wood as could carry
                if is_located_near(physical_object_query, worker_id, &storage_id, 4.0) {
                    // what if storage could not contain more wood/resources
                    store_wood(worker_id, &mut storage, &mut resource_carrier_query); // TODO: convert to either an event or a component
                    commands.entity(*worker_id).remove::<Intent>(); // jsut a trigger to pick up the next intent
                } else {
                    // what if path is blocked, what if target moved
                    ev_travel_to_target.send(TravelToTargetEvent {
                        creature_id: *worker_id,
                        target_id: storage_id,
                    });
                }
            } else {
                if is_located_near(physical_object_query, worker_id, tree_id, 4.0) {
                    commands
                        .entity(*worker_id)
                        .insert(CuttingTree { tree_id: *tree_id });
                } else {
                    ev_travel_to_target.send(TravelToTargetEvent {
                        creature_id: *worker_id,
                        target_id: *tree_id,
                    });
                }
            }
        }
        _ => (),
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

fn can_carry_more_wood(
    worker_id: &Entity,
    resource_carrier_query: &mut Query<&mut ResourceCarrier>,
) -> bool {
    let carrier = resource_carrier_query.get_mut(*worker_id).unwrap();
    carrier.wood <= carrier.max_wood
}

fn is_located_near(
    query: Query<&PhysicalObject>,
    who_id: &Entity,
    where_id: &Entity,
    within_distance: f32,
) -> bool {
    let _who = query.get(*who_id).unwrap();
    let _where = query.get(*where_id).unwrap();
    (*_who).position.distance((*_where).position) < within_distance
}
