use bevy::{
    math::{Vec2, Vec3},
    prelude::{AppBuilder, Commands, Entity, EventWriter, Plugin, Res},
};
use rand::{prelude::ThreadRng, Rng};

use crate::{
    behaviour::{CreatureActivityChangedEvent, Walker},
    creatures::{ConstructionSkill, Creature, CreatureActivity, Fatigue},
    loading::Materials,
    physics::{Mobile, PhysicalObject, Speed},
    random_names::RANDOM_NAMES,
    sprite_helpers::spawn_sprite_bundles,
    village::VillageTask,
    world_gen::SimParams,
};

pub struct CreatureJoinedVillageEvent(pub Entity);

pub struct CreatureLeftVillageEvent(pub Entity);

pub struct VillagerSettledEvent {
    pub resident: Entity,
    pub residence: Entity,
}

pub struct Resident {
    pub residence_id: Entity,
}

pub struct Villager {
    pub task: Option<VillageTask>,
}

pub struct ResidencePlugin;

impl Plugin for ResidencePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_event::<CreatureJoinedVillageEvent>()
            .add_event::<CreatureLeftVillageEvent>();
    }
}

pub fn spawn_villager(
    commands: &mut Commands,
    materials: &Res<Materials>,
    position: Vec2,
    sim_params: &Res<SimParams>,
    ev_creature_joined_village: &mut EventWriter<CreatureJoinedVillageEvent>,
    ev_creature_activity_changed: &mut EventWriter<CreatureActivityChangedEvent>,
) {
    let bounding_box = Vec3::new(16.0, 16.0, 16.0);
    let name = RANDOM_NAMES[rand::thread_rng().gen_range(0..RANDOM_NAMES.len() - 1)];
    let creature_id = spawn_sprite_bundles(
        commands,
        Vec3::ONE,
        position,
        bounding_box,
        materials.man.clone(),
        materials.shadow.clone(),
        sim_params.world_rect.size,
    )
    .insert(Villager {
        task: Option::<VillageTask>::None,
    })
    .insert(Creature {
        name,
        activity: CreatureActivity::Idling,
    })
    .insert(Fatigue(0.0))
    .insert(ConstructionSkill(0.75)) // just a sample value, 75% of the standard speed
    .insert(PhysicalObject { position })
    .insert(Mobile(Speed(0.0)))
    .insert(Walker {
        acceleration: 15.0,
        max_speed: 3.0,
    })
    .id();

    ev_creature_joined_village.send(CreatureJoinedVillageEvent(creature_id));
    ev_creature_activity_changed.send(CreatureActivityChangedEvent {
        creature_id,
        activity: CreatureActivity::Idling,
    });

    // TODO: joined village
}
