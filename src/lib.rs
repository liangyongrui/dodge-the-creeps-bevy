mod audio;
mod common;
mod enemy;
mod loading;
mod menu;
mod player;
mod ui;

use bevy::app::App;
#[cfg(debug_assertions)]
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use common::animation::AnimationPlugin;
use common::path::Path;
use common::GameState;
use enemy::EnemyPlugin;
use ui::UiPlugin;

use crate::audio::InternalAudioPlugin;
use crate::loading::LoadingPlugin;
use crate::menu::MenuPlugin;
use crate::player::PlayerPlugin;

#[derive(Deref)]
pub struct ScreenPath(Path<4>);
impl Default for ScreenPath {
    fn default() -> Self {
        Self(Path([
            Vec2::new(-200., 360.),
            Vec2::new(200., 360.),
            Vec2::new(200., -360.),
            Vec2::new(-200., -360.),
        ]))
    }
}
pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_state(GameState::Loading)
            .add_plugin(UiPlugin)
            .add_plugin(LoadingPlugin)
            .add_plugin(MenuPlugin)
            .add_plugin(InternalAudioPlugin)
            .add_plugin(PlayerPlugin)
            .add_plugin(EnemyPlugin)
            .add_plugin(AnimationPlugin)
            .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
            .init_resource::<ScreenPath>()
            .add_startup_system(setup);

        #[cfg(debug_assertions)]
        {
            app.add_plugin(FrameTimeDiagnosticsPlugin::default())
                .add_plugin(LogDiagnosticsPlugin::default())
                .add_plugin(RapierDebugRenderPlugin::default());
        }
    }
}

pub fn setup(mut commands: Commands, mut rapier_configuration: ResMut<RapierConfiguration>) {
    commands.spawn_bundle(Camera2dBundle::default());
    rapier_configuration.gravity = Vec2::ZERO;
}
