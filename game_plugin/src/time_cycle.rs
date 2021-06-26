use bevy::core::Time;
use bevy::prelude::*;

use crate::GameState;

pub struct TimeCycle {
    day: u64,
    time: time::Time,
    speed: u32,
}
impl Default for TimeCycle {
    fn default() -> Self {
        Self {
            day: 0,
            // A new day begins at seven o’clock 😀
            time: time::Time::from_hms(7, 0, 0).unwrap(),
            speed: 400,
        }
    }
}
pub fn time_cycle_system(
    time: Res<Time>,
    mut time_cycle: ResMut<TimeCycle>,
) {
    let delta = time.delta();
    let now = time_cycle.time + delta * time_cycle.speed;
    // Warning: Only apply to delta * speed < 24 hours
    if now <= time_cycle.time {
        time_cycle.day += 1;
    }
    time_cycle.time = now;
}

impl TimeCycle {
    // Set: From 6 p.m. to 5 a.m
    pub fn is_night(&self) -> bool {
        let hour = self.time.hour();
        hour >= 18 || hour <= 5
    } 
    pub fn to_string(&self) -> String {
        format!("{}day {:02}:{:02}",self.day,self.time.hour(),self.time.minute())
    }
}
pub struct TimeUi;

pub fn set_up_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    time_cycle: Res<TimeCycle>
) {
    commands.spawn_bundle(
        TextBundle {
            style: Style {
                align_self: AlignSelf::Center,
                position_type: PositionType::Absolute,
                position: Rect {
                    top: Val::Percent(0.0),
                    left: Val::Percent(40.0),
                    ..Default::default()
                },
                ..Default::default()
            },
            text: Text {
                sections: vec![
                    TextSection {
                    value:time_cycle.to_string(),
                    style: TextStyle {
                        font: asset_server.get_handle("fonts/FiraSans-Bold.ttf"),
                        font_size: 40.0,
                        color: Color::rgb(0.9, 0.9, 0.9),
                    },
                    }
                ],
                alignment: TextAlignment {
                    vertical: VerticalAlign::Top,
                    horizontal: HorizontalAlign::Center,
                },
            },
            transform: Transform::from_xyz(0.0,0.0,10.0),
            ..Default::default()
        }
    )
    .insert(TimeUi);
}

pub fn sync_ui(
    mut ui_query: Query<&mut Text,With<TimeUi>>,
    time_cycle: Res<TimeCycle>
) {
    if let Ok(mut text) = ui_query.single_mut() {
        if let Some(text_section) = text.sections.get_mut(0) {
            text_section.value = time_cycle.to_string();
        }
    }
}

pub struct TimeCyclePlugin;

impl Plugin for TimeCyclePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
        .init_resource::<TimeCycle>()
        .add_system_set(
            SystemSet::on_enter(GameState::Playing)
            .with_system(set_up_ui.system())
        )
        .add_system_set(
            SystemSet::on_update(GameState::Playing)
            .with_system(time_cycle_system.system())
            .with_system(sync_ui.system())
        );
    }
}