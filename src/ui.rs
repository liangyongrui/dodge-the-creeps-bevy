use bevy::prelude::*;

use crate::common::GameState;
use crate::loading::FontAssets;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_exit(GameState::Loading).with_system(setup))
            .add_system_set(SystemSet::on_enter(GameState::Playing).with_system(clear_score))
            .add_system_set(SystemSet::on_update(GameState::Playing).with_system(update_score));
    }
}
#[derive(Component)]
struct Score(f32);

fn setup(mut commands: Commands, font_assets: Res<FontAssets>) {
    commands
        .spawn_bundle(
            TextBundle::from_sections([TextSection::from_style(TextStyle {
                font: font_assets.xolonium_regular.clone(),
                font_size: 60.0,
                color: Color::GOLD,
            })])
            .with_style(Style {
                position_type: PositionType::Absolute,
                position: UiRect {
                    top: Val::Px(10.0),
                    left: Val::Px(170.),
                    ..default()
                },
                ..default()
            }),
        )
        .insert(Score(0.0));
}

fn clear_score(mut query: Query<&mut Score>) {
    for mut score in &mut query {
        score.0 = 0.0;
    }
}
fn update_score(time: Res<Time>, mut query: Query<(&mut Text, &mut Score)>) {
    for (mut text, mut score) in &mut query {
        score.0 += time.delta_seconds();
        text.sections[0].value = score.0.floor().to_string();
    }
}
