use bevy::{core::Time, math::Vec2, prelude::*};

use crate::{
    creatures::CreatureActivity,
    physics::{get_point_between, Mobile, PhysicalObject},
    world_gen::{gen_in_rect, SimParams},
};

pub struct Walker {
    pub acceleration: f32,
    pub max_speed: f32,
}

pub struct TravellingTo(Vec2);

pub struct ArrivedToPointEvent(pub Entity);

pub struct CreatureActivityChangedEvent {
    pub creature_id: Entity,
    pub activity: CreatureActivity,
}

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_event::<ArrivedToPointEvent>()
            .add_event::<CreatureActivityChangedEvent>()
            .add_system(travel.system())
            .add_system(continue_idling.system())
            .add_system(check_activity.system());
    }
}

pub fn travel(
    time: Res<Time>,
    mut commands: Commands,
    mut moving_query: Query<(
        Entity,
        &mut PhysicalObject,
        &mut Transform,
        &Walker,
        &TravellingTo,
        &mut Mobile,
    )>,
    mut ev_arrived: EventWriter<ArrivedToPointEvent>,
) {
    let game_hour = 3600.0 * 0.00001; // TODO: move to a daytime calc
    let hours = time.delta_seconds() * game_hour;

    for (
        entity,
        mut physical_object,
        mut transform,
        walker,
        TravellingTo(destination),
        mut mobile,
    ) in moving_query.iter_mut()
    {
        if physical_object.position == *destination {
            mobile.0 .0 = 0.0;
            continue;
        }

        let new_speed = mobile.0 .0 + walker.acceleration * hours;
        mobile.0 .0 = walker.max_speed.min(new_speed);

        let speed = mobile.0 .0;
        let distance_travelled = mobile.0 .0 * hours;
        physical_object.position = get_point_between(physical_object.position, *destination, speed);
        transform.translation = physical_object.position.extend(transform.translation.z);

        if physical_object.position == *destination {
            commands.entity(entity).remove::<TravellingTo>();
            ev_arrived.send(ArrivedToPointEvent(entity))
        }
    }
}

pub fn continue_idling(
    mut commands: Commands,
    sim_params: Res<SimParams>,
    mut ev_arrived_to_point: EventReader<ArrivedToPointEvent>,
) {
    for ArrivedToPointEvent(creature_id) in ev_arrived_to_point.iter() {
        let destination = gen_in_rect(&mut rand::thread_rng(), &sim_params.world_rect);
        commands
            .entity(*creature_id)
            .insert(TravellingTo(destination));
    }
}

pub fn check_activity(
    mut commands: Commands,
    sim_params: Res<SimParams>,
    mut ev_activity_changed: EventReader<CreatureActivityChangedEvent>,
    mut ev_arrived_to_point: EventWriter<ArrivedToPointEvent>,
) {
    for CreatureActivityChangedEvent {
        creature_id,
        activity,
    } in ev_activity_changed.iter()
    {
        match activity {
            CreatureActivity::Idling => ev_arrived_to_point.send(ArrivedToPointEvent(*creature_id)),
            _ => (),
        }
    }
}
