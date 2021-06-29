use bevy::{
    math::{Vec2, Vec3},
    prelude::*,
};

use crate::{
    loading::Materials,
    physics::PhysicalObject,
    sprite_helpers::spawn_sprite_bundles_,
    tree_cutting::ResourceStorage,
    village::{Building, LivingSpace, LivingSpaceAvailableEvent},
    world_gen::SimParams,
};

pub fn spawn_house(
    commands: &mut Commands,
    materials: &Res<Materials>,
    position: Vec2,
    sim_params: &Res<SimParams>,
    max_people: u32,
    ev_living_space_available: &mut EventWriter<LivingSpaceAvailableEvent>,
) {
    let bounding_box = Vec3::new(40.0, 30.0, 40.0);
    let residence_id = spawn_sprite_bundles_(
        commands,
        Vec3::ONE,
        position,
        bounding_box,
        materials.house.clone(),
        materials.shadow.clone(),
        sim_params.world_rect.size,
        Vec2::new(0.0, -10.0),
    )
    .insert(Building)
    .insert(LivingSpace {
        current_people: 0,
        max_people,
    })
    .id();

    ev_living_space_available.send(LivingSpaceAvailableEvent { residence_id })
}

pub fn spawn_stockpile(
    commands: &mut Commands,
    materials: &Res<Materials>,
    position: Vec2,
    sim_params: &Res<SimParams>,
) {
    let bounding_box = Vec3::new(100.0, 40.0, 40.0);
    spawn_sprite_bundles_(
        commands,
        Vec3::ONE,
        position,
        bounding_box,
        materials.stockpile.clone(),
        materials.shadow.clone(),
        sim_params.world_rect.size,
        Vec2::new(0.0, -20.0),
    )
    .insert(PhysicalObject { position })
    .insert(ResourceStorage { wood: 0.0 });
}

// pub fn display_resource_pile(children_query: Query<(Entity, &Children, &ResourceStorage)>, commands: &mut Commands) {
//     for (stockpile_id, children, resource_storage) in children_query.iter() {
//         commands.entity(stockpile_id).
//         if resource_storage.wood == 0.0 {
//             for &child in children.iter() {
//                 let child_comp = q_child.get(child)

//             }
//         }
//     }
// }
