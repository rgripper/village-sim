use bevy::{
    math::{Vec2, Vec3},
    prelude::{AppBuilder, Commands, Entity, Plugin, Res},
};

use crate::{
    creatures::{ConstructionSkill, Creature, CreatureActivity, Fatigue},
    loading::Materials,
    sprite_helpers::spawn_sprite_bundles,
    village::VillageTask,
    world_gen::SimParams,
};

pub struct ResidentJoinedEvent(pub Entity);

pub struct ResidentLeftEvent(pub Entity);

pub struct ResidentSettledEvent {
    pub resident: Entity,
    pub dwelling: Entity,
}

pub struct Resident {
    pub task: Option<VillageTask>,
    pub dwelling: Option<Entity>,
}

pub struct ResidencePlugin;

impl Plugin for ResidencePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_event::<ResidentJoinedEvent>()
            .add_event::<ResidentLeftEvent>();
    }
}

pub fn gen_resident(
    commands: &mut Commands,
    materials: &Res<Materials>,
    villager_pos: Vec2,
    sim_params: &Res<SimParams>,
) {
    let bounding_box = Vec3::new(16.0, 16.0, 16.0);
    spawn_sprite_bundles(
        commands,
        Vec3::ONE,
        villager_pos,
        bounding_box,
        materials.man.clone(),
        materials.shadow.clone(),
        sim_params.world_rect.size,
    )
    .insert(Resident {
        dwelling: Option::<Entity>::None,
        task: Option::<VillageTask>::None,
    })
    .insert(Creature {
        activity: CreatureActivity::Standing,
    })
    .insert(Fatigue(0.0))
    .insert(ConstructionSkill(0.75)); // just a sample value, 75% of the standard speed
}
