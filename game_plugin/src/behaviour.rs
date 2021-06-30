use bevy::{core::Time, math::Vec2, prelude::*};

use crate::{
    physics::{get_point_between, Mobile, PhysicalObject},
    world_gen::{gen_in_rect, SimParams},
};

pub struct Walker {
    pub acceleration: f32,
    pub max_speed: f32,
}

pub struct CheckTaskEvent(pub Entity);

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_event::<CheckTaskEvent>()
            .add_system(go_to_target.system())
            .add_system(go_to_position.system());
    }
}

pub struct TravelToTarget {
    pub time_to_next_location_check: f32,
    pub target_id: Entity,
    pub last_target_position: Option<Vec2>,
}

impl TravelToTarget {
    fn update(&mut self, new_position: Vec2, recheck_position_interval: f32, delta_seconds: f32) {
        if self.time_to_next_location_check == 0.0 {
            self.time_to_next_location_check = recheck_position_interval;
            self.last_target_position = Some(new_position);
        } else {
            self.time_to_next_location_check -= delta_seconds.min(self.time_to_next_location_check);
        }
    }
}

pub struct TravelToPosition {
    pub position: Vec2,
}

pub fn go_to_target(
    time: Res<Time>,
    mut commands: Commands,
    mut moving_query: Query<(
        Entity,
        &mut Transform,
        &Walker,
        &mut TravelToTarget,
        &mut Mobile,
    )>,
    mut physical_object_query: Query<&mut PhysicalObject>,
    mut ev_check_intent: EventWriter<CheckTaskEvent>,
) {
    let recheck_position_interval = 3000.0;
    let game_hour = 3600.0 * 0.00001; // TODO: move to a daytime calc
    let hours = time.delta_seconds() * game_hour;

    for (entity, mut transform, walker, mut travel_to_target, mut mobile) in moving_query.iter_mut()
    {
        let position = physical_object_query
            .get_mut(travel_to_target.target_id)
            .unwrap()
            .position;
        travel_to_target.update(position, recheck_position_interval, time.delta_seconds());

        let mut physical_object = physical_object_query.get_mut(entity).unwrap();
        let destination: Vec2 = travel_to_target.last_target_position.unwrap();

        let result = _go_to_position(
            &mut physical_object,
            destination,
            &mut mobile,
            &mut commands,
            entity,
            &mut ev_check_intent,
            walker,
            hours,
            &mut transform,
        );

        match result {
            TravelResult::Arrived => {
                commands.entity(entity).remove::<TravelToTarget>();
                ev_check_intent.send(CheckTaskEvent(entity));
                println!("Arrived to a target located at {}", destination);
            }
            TravelResult::Traveling => (),
        }
    }
}

pub fn go_to_position(
    time: Res<Time>,
    mut commands: Commands,
    mut moving_query: Query<(
        Entity,
        &mut Transform,
        &Walker,
        &TravelToPosition,
        &mut Mobile,
    )>,
    mut physical_object_query: Query<&mut PhysicalObject>,
    mut ev_check_intent: EventWriter<CheckTaskEvent>,
) {
    let recheck_position_interval = 3000.0;
    let game_hour = 3600.0 * 0.00001; // TODO: move to a daytime calc
    let hours = time.delta_seconds() * game_hour;

    for (
        entity,
        mut transform,
        walker,
        TravelToPosition {
            position: destination,
        },
        mut mobile,
    ) in moving_query.iter_mut()
    {
        let mut physical_object = physical_object_query.get_mut(entity).unwrap();

        let result = _go_to_position(
            &mut physical_object,
            *destination,
            &mut mobile,
            &mut commands,
            entity,
            &mut ev_check_intent,
            walker,
            hours,
            &mut transform,
        );

        match result {
            TravelResult::Arrived => {
                commands.entity(entity).remove::<TravelToPosition>();
                ev_check_intent.send(CheckTaskEvent(entity));
                println!("Arrived to a position {}", destination);
            }
            TravelResult::Traveling => (),
        }
    }
}

pub enum TravelResult {
    Arrived,
    Traveling,
}

fn _go_to_position(
    physical_object: &mut PhysicalObject,
    destination: Vec2,
    moving: &mut Mobile,
    commands: &mut Commands,
    entity: Entity,
    ev_check_intent: &mut EventWriter<CheckTaskEvent>,
    walker: &Walker,
    hours: f32,
    transform: &mut Transform,
) -> TravelResult {
    if physical_object.position == destination {
        moving.0 .0 = 0.0;
        TravelResult::Arrived
    } else {
        let new_speed = moving.0 .0 + walker.acceleration * hours;
        moving.0 .0 = walker.max_speed.min(new_speed);

        let speed = moving.0 .0;
        let distance_travelled = moving.0 .0 * hours;
        physical_object.position = get_point_between(physical_object.position, destination, speed);
        transform.translation = physical_object.position.extend(transform.translation.z);
        TravelResult::Traveling
    }
}

#[derive(Debug, Clone)]
pub enum Task {
    CutTree(Entity),
    PickUpWood(f32),
    DropOffResources,
    WanderAimlessly,
}
