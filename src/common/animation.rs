use std::ops::{Deref, DerefMut};

use benimator::FrameRate;
use bevy::prelude::*;

use crate::GameState;

pub struct AnimationPlugin;

impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_update(GameState::Playing).with_system(animate));
    }
}

#[derive(Component, Deref)]
pub struct Animation(benimator::Animation);
#[derive(Component)]
pub struct AnimationState {
    playing: bool,
    state: benimator::State,
}
impl Default for AnimationState {
    fn default() -> Self {
        Self {
            playing: true,
            state: Default::default(),
        }
    }
}

impl Deref for AnimationState {
    type Target = benimator::State;

    fn deref(&self) -> &Self::Target {
        &self.state
    }
}
impl DerefMut for AnimationState {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.state
    }
}

impl AnimationState {
    pub fn play(&mut self) {
        self.playing = true;
    }

    pub fn stop(&mut self) {
        self.playing = false;
    }
}
impl Animation {
    pub fn from_indices(indices: impl IntoIterator<Item = usize>, frame_rate: FrameRate) -> Self {
        Animation(benimator::Animation::from_indices(indices, frame_rate))
    }
}

pub fn animate(
    time: Res<Time>,
    mut query: Query<(&mut TextureAtlasSprite, &mut AnimationState, &mut Animation)>,
) {
    for (mut texture, mut state, animation) in &mut query {
        if state.playing {
            state.update(&animation, time.delta());
            texture.index = state.frame_index();
        }
    }
}
