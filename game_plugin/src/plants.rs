use rand_distr::{Binomial, Distribution};

pub struct Tree;

pub struct PlantSize {
    pub current: f32,
    pub max: f32,
}

// TODO: could be some feritily settings, affeted by the plant's age, soil, weather, which will not be implemented for this MVP
pub struct Seeder {
    pub seeds_since_last_time: f32,
    pub seed_growth_per_second: Range<f32>,
    pub survival_probability: f32,
}

impl Seeder {
    fn produce(&mut self, delta_seconds: f32) -> u32 {
        let seeds = rand::thread_rng().gen_range(
            self.seed_growth_per_second.start * delta_seconds
                ..self.seed_growth_per_second.end * delta_seconds,
        );
        self.seeds_since_last_time += seeds;
        let whole_seeds = self.seeds_since_last_time.floor();
        self.seeds_since_last_time -= whole_seeds;

        let bin = Binomial::new(whole_seeds as u64, self.survival_probability.into()).unwrap();
        return bin.sample(&mut rand::thread_rng()).try_into().unwrap();
    }
}

use std::{convert::TryInto, ops::Range};

use crate::{
    hexagon::Rectangle,
    loading::Materials,
    sprite_helpers::spawn_sprite_bundles,
    world_gen::{gen_in_rect, SimParams},
    GameState,
};
use bevy::prelude::*;
use rand::Rng;

pub struct PlantLifePlugin;

pub struct WoodResource(pub f32);

impl Plugin for PlantLifePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_set(
            SystemSet::on_update(GameState::Playing)
                .with_system(grow.system())
                .with_system(seed.system()),
        );
    }
}

fn grow(
    time: Res<Time>,
    mut plant_size_query: Query<(&mut Transform, &mut PlantSize, &mut WoodResource)>,
) {
    for (mut transform, mut plant_size, mut wood_res) in plant_size_query.iter_mut() {
        set_tree_size_and_resource(&time, &mut transform, &mut plant_size, &mut wood_res);
    }
}

pub fn set_tree_size_and_resource(
    time: &Res<Time>,
    transform: &mut Mut<Transform>,
    plant_size: &mut Mut<PlantSize>,
    wood_res: &mut Mut<WoodResource>,
) {
    if plant_size.current < plant_size.max {
        plant_size.current = plant_size
            .max
            .min(plant_size.current + 0.1 * time.delta_seconds());
        transform.scale = get_scale_from_tree_size(&plant_size);
        wood_res.0 = plant_size.current;
    }
}

fn seed(
    time: Res<Time>,
    mut seeder_query: Query<(&Transform, &mut Seeder)>,
    mut commands: Commands,
    materials: Res<Materials>,
    sim_params: Res<SimParams>,
) {
    let rng = &mut rand::thread_rng();

    for (transform, mut seeder) in seeder_query.iter_mut() {
        let trees = seeder.produce(time.delta_seconds());
        for _ in 0..trees {
            let tree_pos = gen_in_rect(
                rng,
                &Rectangle {
                    position: transform.translation.truncate(),
                    size: Vec2::new(20., 20.),
                },
            );

            spawn_tree(
                tree_pos,
                0.0,
                &sim_params.world_rect,
                &mut commands,
                &materials.tree,
                &materials.shadow,
            );
        }
    }
}

pub fn get_scale_from_tree_size(plant_size: &PlantSize) -> Vec3 {
    Vec3::new(plant_size.current, plant_size.current, 1.0)
}

pub fn spawn_tree(
    pos: Vec2,
    init_plant_size: f32,
    world_rect: &Rectangle,
    commands: &mut Commands,
    tree_material: &Handle<ColorMaterial>,
    shadow_material: &Handle<ColorMaterial>,
) {
    let plant_size = PlantSize {
        current: init_plant_size,
        max: 1.0,
    };

    let bounding_box = Vec3::new(24.0, 48.0, 24.0);

    spawn_sprite_bundles(
        commands,
        get_scale_from_tree_size(&plant_size),
        pos,
        bounding_box,
        tree_material.clone(),
        shadow_material.clone(),
        world_rect.size,
    )
    .insert(Tree)
    .insert(WoodResource(0.0))
    .insert(Seeder {
        seed_growth_per_second: (0.0..1.0),
        seeds_since_last_time: 0.0,
        survival_probability: 0.01,
    })
    .insert(plant_size);
}
