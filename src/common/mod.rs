//! 这里是一些和游戏逻辑无关的代码
//!

pub mod animation;
pub mod path;

use bevy::prelude::*;

// This example game uses States to separate logic
// See https://bevy-cheatbook.github.io/programming/states.html
// Or https://github.com/bevyengine/bevy/blob/main/examples/ecs/state.rs
#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub enum GameState {
    // During the loading State the LoadingPlugin will load our assets
    Loading,
    // During this State the actual game logic is executed
    Playing,
    // Here the menu is drawn and waiting for player interaction
    Menu,
}

pub fn clear_entities<T: Component>(mut commands: Commands, query: Query<Entity, With<T>>) {
    for e in &query {
        commands.entity(e).despawn_recursive();
    }
}
