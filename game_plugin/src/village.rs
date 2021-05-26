use crate::{
    creatures::{ConstructionSkill, Creature, CreatureActivity, Fatigue},
    loading::Materials,
    residence::{CreatureJoinedVillageEvent, CreatureLeftVillageEvent, Resident},
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

pub struct Building;

pub struct LivingSpace {
    max_people: u32,
    current_people: u32,
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
    pub residence_entity: Entity,
}

struct VillagePlugin;

impl Plugin for Village {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_set(
            SystemSet::on_update(GameState::Playing)
                .with_system(control_residence.system())
                .with_system(house_homeless.system()),
        )
        .add_event::<LivingSpaceAvailableEvent>();
    }
}

fn control_residence(
    mut commands: Commands,
    mut village_query: Query<&mut Village>,
    resident_query: Query<&Resident>,
    mut ev_residents_joined: EventReader<CreatureJoinedVillageEvent>,
    mut ev_residents_left: EventReader<CreatureLeftVillageEvent>,
    mut ev_living_space_available: EventWriter<LivingSpaceAvailableEvent>,
) {
    let mut village = village_query
        .single_mut()
        .expect("So far there must be one village");

    for CreatureLeftVillageEvent(creature_entity) in ev_residents_left.iter() {
        village.habitants_count -= 1;
        if let Result::Ok(resident) = resident_query.get(*creature_entity) {
            ev_living_space_available.send(LivingSpaceAvailableEvent {
                residence_entity: resident.residence_entity,
            });
        }
    }

    for _ in ev_residents_joined.iter() {
        village.habitants_count += 1;
        village.homeless_count += 1;
    }

    // TODO: maybe settle each resident right away somehow?
}

fn house_homeless(
    mut commands: Commands,
    mut ev_living_space_available: EventReader<LivingSpaceAvailableEvent>,
    homeless_query: Query<Entity, Without<LivingSpace>>,
    mut living_space_query: Query<&mut LivingSpace>,
) {
    for LivingSpaceAvailableEvent { residence_entity } in ev_living_space_available.iter() {
        for homeless_entity in homeless_query.iter() {
            commands.entity(homeless_entity).insert(Resident {
                residence_entity: *residence_entity,
            });
            if let Result::Ok(mut living_space) = living_space_query.get_mut(*residence_entity) {
                if living_space.current_people == living_space.max_people {
                    panic!("Residence could not have more residents");
                }
                living_space.current_people += 1;
            }

            break;
        }
    }
}
