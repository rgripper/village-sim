pub struct Villager {
    pub fatigue: f32,
}

pub struct VillageResources {
    pub wood: f32,
}

pub enum Building {
    House {
        max_people: u32,
        current_people: u32,
    },
    Storage,
}

pub struct Construction {
    pub wood_required: f32,
    pub work_required: f32,
}
