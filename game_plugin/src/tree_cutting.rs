use bevy::prelude::*;

use crate::{
    actions::Actions,
    behaviour::{CheckIntentEvent, Intent, TravelToPosition, TravelToTarget},
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

pub struct IntentPlugin;

impl Plugin for IntentPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_set(
            SystemSet::on_update(GameState::Playing)
                .with_system(check_intent.system())
                .with_system(assign_intent.system()),
        );
    }
}

pub fn check_intent(
    mut commands: Commands,
    sim_params: Res<SimParams>,
    intent_query: Query<&Intent>,
    physical_object_query: Query<&PhysicalObject>,
    physical_object_id_query: Query<Entity, With<PhysicalObject>>,
    mut resource_carrier_query: Query<&mut ResourceCarrier>,
    mut resource_storage_query: Query<(&mut ResourceStorage, Entity)>,
    added_intent_query: Query<Entity, Added<Intent>>,
    mut ev_intent_event: EventReader<CheckIntentEvent>,
) {
    for creature_id in added_intent_query.iter().chain(
        ev_intent_event
            .iter()
            .map(|CheckIntentEvent(creature_id)| *creature_id),
    ) {
        if let Ok(intent) = intent_query.get(creature_id) {
            act_on_intent(
                &mut commands,
                &sim_params.world_rect,
                &physical_object_query,
                &physical_object_id_query,
                &mut resource_carrier_query,
                &mut resource_storage_query,
                &creature_id,
                intent,
            );
        }
    }
}

pub fn assign_intent(
    mut commands: Commands,
    creature_without_intent_query: Query<Entity, (With<Creature>, Without<Intent>)>,
) {
    for (creature_id) in creature_without_intent_query.iter() {
        commands.entity(creature_id).insert(Intent::Idle);
    }
}

pub fn act_on_intent(
    commands: &mut Commands,
    world_rect: &Rectangle,
    physical_object_query: &Query<&PhysicalObject>,
    physical_object_id_query: &Query<Entity, With<PhysicalObject>>,
    resource_carrier_query: &mut Query<&mut ResourceCarrier>,
    resource_storage_query: &mut Query<(&mut ResourceStorage, Entity)>,
    worker_id: &Entity,
    intent: &Intent,
) {
    match intent {
        Intent::CutTree(tree_id) => {
            let (mut storage, storage_id) = resource_storage_query.single_mut().unwrap();
            if !can_carry_more_wood(worker_id, resource_carrier_query) {
                if is_located_near(physical_object_query, worker_id, &storage_id, 4.0) {
                    // what if storage could not contain more wood/resources
                    store_wood(worker_id, &mut storage, resource_carrier_query);
                    commands.entity(*worker_id).remove::<Intent>();
                } else {
                    commands.entity(*worker_id).insert(TravelToTarget {
                        time_to_next_location_check: 0.0,
                        last_target_position: None,
                        target_id: storage_id,
                    }); // jsut a trigger to pick up the next intent
                }
            } else {
                if is_located_near(physical_object_query, worker_id, tree_id, 4.0) {
                    commands
                        .entity(*worker_id)
                        .insert(CuttingTree { tree_id: *tree_id });
                } else {
                    commands.entity(*worker_id).insert(TravelToTarget {
                        time_to_next_location_check: 0.0,
                        last_target_position: None,
                        target_id: *tree_id,
                    }); // jsut a trigger to pick up the next intent
                }
            }
        }
        Intent::Idle => {
            let rng = &mut rand::thread_rng();
            let position = gen_in_rect(rng, world_rect);
            commands
                .entity(*worker_id)
                .insert(TravelToPosition { position });
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

fn can_carry_more_wood(
    worker_id: &Entity,
    resource_carrier_query: &mut Query<&mut ResourceCarrier>,
) -> bool {
    let carrier = resource_carrier_query.get_mut(*worker_id).unwrap();
    carrier.wood <= carrier.max_wood
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
