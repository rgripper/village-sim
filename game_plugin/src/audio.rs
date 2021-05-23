use crate::loading::AudioAssets;
use crate::GameState;
use bevy::prelude::*;
use bevy_kira_audio::{Audio, AudioChannel, AudioPlugin};

pub struct InternalAudioPlugin;

impl Plugin for InternalAudioPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.insert_resource(AudioChannels {
            birds: AudioChannel::new("birds".to_owned()),
        })
        .add_plugin(AudioPlugin)
        .add_system_set(SystemSet::on_enter(GameState::Playing).with_system(start_audio.system()))
        .add_system_set(
            SystemSet::on_update(GameState::Playing).with_system(control_bird_sound.system()),
        )
        .add_system_set(SystemSet::on_exit(GameState::Playing).with_system(stop_audio.system()));
    }
}

struct AudioChannels {
    birds: AudioChannel,
}

fn start_audio(audio_assets: Res<AudioAssets>, audio: Res<Audio>, channels: Res<AudioChannels>) {
    audio.set_volume_in_channel(0.3, &channels.birds);
    audio.play_looped_in_channel(audio_assets.birds.clone(), &channels.birds);
    audio.pause_channel(&channels.birds);
}

fn stop_audio(audio: Res<Audio>, channels: Res<AudioChannels>) {
    audio.stop_channel(&channels.birds);
}

fn control_bird_sound(
    ambience_query: Query<&Ambience>,
    audio: Res<Audio>,
    channels: Res<AudioChannels>,
) {
    for ambience in ambience_query.iter() {
        if ambience.is_forest {
            audio.resume_channel(&channels.birds);
        } else {
            audio.pause_channel(&channels.birds);
        }
    }
}

pub struct Ambience {
    pub is_forest: bool,
}
