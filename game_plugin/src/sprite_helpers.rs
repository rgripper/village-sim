use bevy::{
    ecs::system::EntityCommands,
    math::{Vec2, Vec3},
    prelude::{BuildChildren, ColorMaterial, Commands, Handle, Sprite, SpriteBundle, Transform},
};

use crate::layers::{OBJECT_LAYER, SHADOW_LAYER};

pub fn spawn_sprite_bundles<'a, 'b>(
    commands: &'b mut Commands<'a>,
    scale: Vec3,
    position: Vec2,
    bounding_box: Vec3,
    main_material: Handle<ColorMaterial>,
    shadow: Handle<ColorMaterial>,
    world_size: Vec2,
) -> EntityCommands<'a, 'b> {
    spawn_sprite_bundles_(
        commands,
        scale,
        position,
        bounding_box,
        main_material,
        shadow,
        world_size,
        Vec2::ZERO,
    )
}

pub fn get_translation(world_size: Vec2, position: Vec2) -> Vec3 {
    Vec3::new(
        position.x,
        position.y,
        OBJECT_LAYER + world_size.y - position.y,
    )
}

pub fn spawn_sprite_bundles_<'a, 'b>(
    commands: &'b mut Commands<'a>,
    scale: Vec3,
    position: Vec2,
    bounding_box: Vec3,
    main_material: Handle<ColorMaterial>,
    shadow: Handle<ColorMaterial>,
    world_size: Vec2,
    origin: Vec2,
) -> EntityCommands<'a, 'b> {
    let mut entity_commands = commands.spawn_bundle(SpriteBundle {
        transform: Transform {
            translation: get_translation(world_size, position),
            scale,
            ..Default::default()
        },
        ..Default::default()
    });

    entity_commands.with_children(|parent| {
        parent.spawn_bundle(SpriteBundle {
            material: main_material.clone(),
            transform: Transform::from_translation(
                (Vec2::new(0.0, bounding_box.y / 2.0) + origin).extend(OBJECT_LAYER),
            ),
            sprite: Sprite::new(Vec2::new(bounding_box.x, bounding_box.y)),
            ..Default::default()
        });
        // TODO: right now shadows look out of place for bulky objects. Maybe we need customised realistic shadows
        // parent.spawn_bundle(SpriteBundle {
        //     transform: Transform::from_translation(Vec2::new(0.0, 0.0).extend(SHADOW_LAYER)),
        //     sprite: Sprite::new(Vec2::new(bounding_box.x, bounding_box.z / 2.0) * shadow_factor), // TODO: do isometric calc for Y coord
        //     material: shadow.clone(),
        //     ..Default::default()
        // });
    });
    entity_commands
}
