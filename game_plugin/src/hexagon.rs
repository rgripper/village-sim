use bevy::math::Vec2;

pub struct Rectangle {
    pub position: Vec2,
    pub size: Vec2,
}

pub struct Hexagon {
    pub vertices: Vec<Vec2>,
}

impl Hexagon {
    pub fn get_bounding_rectangle(&self) -> Rectangle {
        Rectangle {
            position: Vec2::new(self.vertices[0].x, self.vertices[1].y),
            size: Vec2::new(
                self.vertices[3].x - self.vertices[0].x,
                self.vertices[4].y - self.vertices[2].y,
            ),
        }
    }
}

pub struct HexagonBuilder {
    size: f32,
    height: f32,
    pointy_offset: f32,
    hexagon_shape: Vec<Vec2>,
}

fn is_even(value: i32) -> bool {
    value.rem_euclid(2) == 0
}

impl HexagonBuilder {
    pub fn get_hexagon_at(&self, origin: Vec2, column: i32, row: i32) -> Hexagon {
        let pos = self.get_top_left(column, row) + origin;
        Hexagon {
            vertices: self
                .hexagon_shape
                .iter()
                .map(|vertex| pos + *vertex)
                .collect(),
        }
    }

    pub fn get_world_rect(&self, column_count: i32, row_count: i32) -> Rectangle {
        let width = (3.0 * self.size * column_count as f32 + 0.5 * self.size).ceil();
        let height = ((self.height * row_count as f32 + self.height) / 2.0).ceil();
        Rectangle {
            position: Vec2::new(-width / 2.0, -height / 2.0),
            size: Vec2::new(width, height),
        }
    }

    pub fn new(size: f32) -> Self {
        let width = 2.0 * size;
        let height = 3.0f32.sqrt() * size;

        let center = Vec2::new(width / 2.0, height / 2.0);

        Self {
            size,
            height,
            pointy_offset: size * 1.5,
            hexagon_shape: vec![
                Vec2::new(width, height / 2.0),
                Vec2::new(width * 3.0 / 4.0, height),
                Vec2::new(width / 4.0, height),
                Vec2::new(0.0, height / 2.0),
                Vec2::new(width / 4.0, 0.0),
                Vec2::new(width * 3.0 / 4.0, 0.0),
            ]
            .iter()
            .map(|vertex| *vertex - center)
            .collect(),
        }
    }

    fn get_top_left(&self, column: i32, row: i32) -> Vec2 {
        Vec2::new(
            column as f32 * self.size * 3.0
                + (if is_even(row) {
                    self.pointy_offset
                } else {
                    0.0
                }),
            row as f32 * (self.height / 2.0),
        )
    }
}
