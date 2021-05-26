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

pub struct CreatureJoinedVillageEvent(pub Entity);

pub struct CreatureLeftVillageEvent(pub Entity);

pub struct VillagerSettledEvent {
    pub resident: Entity,
    pub residence: Entity,
}

pub struct Resident {
    pub residence_entity: Entity,
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
    .insert(Villager {
        task: Option::<VillageTask>::None,
    })
    .insert(Creature {
        activity: CreatureActivity::Standing,
    })
    .insert(Fatigue(0.0))
    .insert(ConstructionSkill(0.75)); // just a sample value, 75% of the standard speed
}
