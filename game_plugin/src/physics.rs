pub struct PhysicalObject {
    position: Vec2,
}

pub struct Velocity(pub Vec2);

pub struct Mobile(Velocity);

pub struct Walker {
    pub acceleration: f32,
    pub max_speed: f32,
}

pub struct Path(Vec<Vec2>);

// TODO: a villager could be a combo of the following components: (Mobile, Walker, Path)

pub fn walking(mut moving_query: Query<&Walker, &mut Mobile>) {
    for (mut walker, mut mobile) in moving_query.iter_mut() {
        moving.current_speed = walker
            .max_speed
            .min(moving.current_speed + moving.acceleration);
    }
    // accelerate
    // max speed
}
