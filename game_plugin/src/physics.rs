use bevy::math::Vec2;

pub struct PhysicalObject {
    pub position: Vec2,
}

pub struct Speed(pub f32); // TODO: deprecate in favour of Velocity

// pub struct Velocity {
//     pub speed: f32,
//     pub direction: Vec2,
// }

pub struct Mobile(pub Speed);

pub fn get_point_between(point1: Vec2, point2: Vec2, distance_from_point1: f32) -> Vec2 {
    let total_distance = point2.distance(point1);
    if distance_from_point1 >= total_distance {
        return point2;
    }

    let s = distance_from_point1 / total_distance;
    point1.lerp(point2, s)
}
