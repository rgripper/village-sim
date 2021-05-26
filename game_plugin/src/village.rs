use crate::{
    creatures::{ConstructionSkill, Creature, CreatureActivity, Fatigue},
    loading::Materials,
    residence::{Resident, ResidentJoinedEvent, ResidentLeftEvent},
    sprite_helpers::spawn_sprite_bundles,
    world_gen::SimParams,
    GameState,
};
use bevy::core::Time;
use bevy::prelude::*;

pub enum VillageTask {
    Construction {
        target: Option<Entity>,
        building_type: PlannedBuildingType,
        workers: Vec<Entity>,
    },
}

pub enum PlannedBuildingType {
    House,
    Storage,
}

pub struct Village {
    pub wood: f32,
    pub habitants_count: u32,
    pub homeless_count: u32,
}

pub enum Building {
    House {
        max_people: u32,
        current_people: u32,
    },
    Storage,
}

// pub struct Construction {
//     pub wood_required: f32,
//     pub work_required: f32,
// }

// pub fn construct(
//     time: Res<Time>,
//     mut seeder_query: Query<(&ConstructionSkill, &mut Fatigue)>,
//     mut commands: Commands,
//     materials: Res<Materials>,
//     sim_params: Res<SimParams>,
// ) {
// }

pub struct LivingSpaceAvailableEvent {
    pub dwelling: Entity,
}

struct VillagePlugin;

impl Plugin for Village {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_set(
            SystemSet::on_update(GameState::Playing).with_system(control_residence.system()),
        )
        .add_event::<LivingSpaceAvailableEvent>();
    }
}

fn control_residence(
    mut commands: Commands,
    mut village_query: Query<&mut Village>,
    resident_query: Query<&Resident>,
    mut ev_residents_joined: EventReader<ResidentJoinedEvent>,
    mut ev_residents_left: EventReader<ResidentLeftEvent>,
    mut ev_living_space_available: EventWriter<LivingSpaceAvailableEvent>,
) {
    let mut village = village_query
        .single_mut()
        .expect("So far there must be one village");

    for ResidentLeftEvent(resident_entity) in ev_residents_left.iter() {
        village.habitants_count -= 1;
        let resident = resident_query.get(*resident_entity).unwrap();
        if let Some(dwelling_entity) = resident.dwelling {
            ev_living_space_available.send(LivingSpaceAvailableEvent {
                dwelling: dwelling_entity,
            });
        }
    }

    for ResidentJoinedEvent(resident_entity) in ev_residents_joined.iter() {
        village.habitants_count += 1;
        village.homeless_count += 1;
    }

    // TODO: maybe settle each resident right away somehow?
}
