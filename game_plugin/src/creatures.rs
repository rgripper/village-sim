pub struct Creature {
    pub activity: CreatureActivity,
}

pub struct Fatigue(pub f32);

pub struct ConstructionSkill(pub f32);

pub enum CreatureActivity {
    Sleeping,
    Standing,
    Walking,
    Constructing,
}
