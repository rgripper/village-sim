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
    world_gen::{gen_in_rect, gen_tree, SimParams},
    GameState,
};
use bevy::prelude::*;
use rand::Rng;

pub struct PlantLifePlugin;

impl Plugin for PlantLifePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_set(
            SystemSet::on_update(GameState::Playing)
                .with_system(grow.system())
                .with_system(seed.system()),
        );
    }
}

fn grow(time: Res<Time>, mut plant_size_query: Query<(&mut Transform, &mut PlantSize)>) {
    for (mut transform, mut plant_size) in plant_size_query.iter_mut() {
        set_tree_size(&time, &mut transform, &mut plant_size);
    }
}

pub fn set_tree_size(
    time: &Res<Time>,
    transform: &mut Mut<Transform>,
    plant_size: &mut Mut<PlantSize>,
) {
    if plant_size.current < plant_size.max {
        plant_size.current = plant_size
            .max
            .min(plant_size.current + 0.1 * time.delta_seconds());
        transform.scale = get_scale_from_tree_size(&plant_size)
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

            gen_tree(
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
