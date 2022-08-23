use bevy::prelude::*;
use bevy_kira_audio::prelude::*;

use crate::common::GameState;
use crate::loading::AudioAssets;

pub struct GameOverEvent;

pub struct InternalAudioPlugin;

impl Plugin for InternalAudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(AudioPlugin)
            .add_system_set(SystemSet::on_update(GameState::Playing).with_system(game_over_sound))
            .add_event::<GameOverEvent>();
    }
}

fn game_over_sound(
    audio: Res<Audio>,
    audio_assets: Res<AudioAssets>,
    mut events: EventReader<GameOverEvent>,
) {
    if events.iter().last().is_some() {
        audio.play(audio_assets.gameover.clone());
    }
}
