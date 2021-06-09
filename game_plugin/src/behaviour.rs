use bevy::{core::Time, math::Vec2, prelude::*};

use crate::{
    physics::{get_point_between, Mobile, PhysicalObject},
    world_gen::{gen_in_rect, SimParams},
};

pub struct Walker {
    pub acceleration: f32,
    pub max_speed: f32,
}

pub struct CheckIntentEvent(pub Entity);

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_event::<CheckIntentEvent>()
            .add_system(go_to_target.system());
    }
}

pub struct TravelToTarget {
    pub time_to_next_location_check: f32,
    pub target_id: Entity,
    pub last_target_position: Option<Vec2>,
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
    mut ev_check_intent: EventWriter<CheckIntentEvent>,
) {
    let recheck_position_interval = 3000.0;
    let game_hour = 3600.0 * 0.00001; // TODO: move to a daytime calc
    let hours = time.delta_seconds() * game_hour;

    for (entity, mut transform, walker, mut travel_to_target, mut mobile) in moving_query.iter_mut()
    {
        if travel_to_target.time_to_next_location_check == 0.0 {
            travel_to_target.time_to_next_location_check = recheck_position_interval;
            travel_to_target.last_target_position =
                Some(physical_object_query.get_mut(entity).unwrap().position);
        } else {
            travel_to_target.time_to_next_location_check -= 0.0f32.max(time.delta_seconds());
        }

        let mut physical_object = physical_object_query.get_mut(entity).unwrap();
        let destination: Vec2 = travel_to_target.last_target_position.unwrap();

        if physical_object.position == destination {
            mobile.0 .0 = 0.0;
            continue;
        }

        let new_speed = mobile.0 .0 + walker.acceleration * hours;
        mobile.0 .0 = walker.max_speed.min(new_speed);

        let speed = mobile.0 .0;
        let distance_travelled = mobile.0 .0 * hours;
        physical_object.position = get_point_between(physical_object.position, destination, speed);
        transform.translation = physical_object.position.extend(transform.translation.z);

        if physical_object.position == destination {
            commands.entity(entity).remove::<TravelToTarget>();
            ev_check_intent.send(CheckIntentEvent(entity))
        }
    }
}

pub enum Intent {
    CutTree(Entity),
    Idle,
}
