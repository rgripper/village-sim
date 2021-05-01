pub struct Rectangle {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

pub struct HexagonBuilder {
    size: f32,
    width: f32,
    height: f32,
    pointy_offset: f32,
    hexagon_shape: Vec<(f32, f32)>,
}

fn is_even(value: i32) -> bool {
    value.rem_euclid(2) == 0
}

impl HexagonBuilder {
    pub fn get_hexagon_at(&self, column: i32, row: i32) -> Vec<(f32, f32)> {
        let (pos_x, pos_y) = self.get_position(column, row);
        self.hexagon_shape
            .iter()
            .map(|(x, y)| (x + pos_x, y + pos_y))
            .collect()
    }

    pub fn get_world_rect(&self, column_count: i32, row_count: i32) -> Rectangle {
        Rectangle {
            x: 0.0,
            y: 0.0,
            width: (3.0 * self.size * column_count as f32 + 0.5 * self.size).ceil(),
            height: ((self.height * row_count as f32 + self.height) / 2.0).ceil(),
        }
    }

    pub fn new(size: f32) -> Self {
        let width = 2.0 * size;
        let height = 3.0f32.sqrt() * size;

        Self {
            size,
            width,
            height,
            pointy_offset: size * 1.5,
            hexagon_shape: vec![
                (width, height / 2.0),
                (width * 3.0 / 4.0, height),
                (width / 4.0, height),
                (0.0, height / 2.0),
                (width / 4.0, 0.0),
                (width * 3.0 / 4.0, 0.0),
            ],
        }
    }

    fn get_position(&self, column: i32, row: i32) -> (f32, f32) {
        (
            column as f32 * self.size * 3.0 + (if is_even(row) { self.size * 1.5 } else { 0.0 }),
            row as f32 * (self.height / 2.0),
        )
    }
}
