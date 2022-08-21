use crate::actions::Actions;
use crate::loading::PlayerTextureAtlas;
use crate::GameState;
use bevy::prelude::*;
use bevy_rapier2d::prelude::{Collider, GravityScale, RigidBody};

pub struct PlayerPlugin;

#[derive(Component)]
pub struct Player {
    animation_timer: Timer,
}

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::Playing).with_system(spawn_player))
            .add_system_set(SystemSet::on_update(GameState::Playing).with_system(move_player));
    }
}

fn spawn_player(mut commands: Commands, player_assets: Res<PlayerTextureAtlas>) {
    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: player_assets.up.clone(),
            transform: Transform {
                translation: Vec3::new(0., 0., 1.),
                scale: Vec3::new(0.5, 0.5, 0.5),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Player {
            animation_timer: Timer::from_seconds(0.2, true),
        })
        .insert(RigidBody::Dynamic)
        .insert(Collider::capsule_y(20.0, 50.0))
        .insert(GravityScale(0.));
}

fn move_player(
    time: Res<Time>,
    actions: Res<Actions>,
    player_assets: Res<PlayerTextureAtlas>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut player_query: Query<(
        &mut Transform,
        &mut Player,
        &mut Handle<TextureAtlas>,
        &mut TextureAtlasSprite,
    )>,
) {
    for (mut player_transform, mut player, mut textrue, mut sprite) in &mut player_query {
        if let Some(player_movement) = actions.player_movement {
            if player_movement == Vec2::ZERO {
                continue;
            }
            player.animation_timer.tick(time.delta());
            if player_movement.x != 0. {
                if textrue.id != player_assets.walk.id {
                    *textrue = player_assets.walk.clone()
                }
                sprite.flip_y = false;
                sprite.flip_x = player_movement.x < 0.;
            } else {
                if textrue.id != player_assets.up.id {
                    *textrue = player_assets.up.clone()
                }
                sprite.flip_y = player_movement.y < 0.;
            }
            if player.animation_timer.just_finished() {
                let texture_atlas = texture_atlases.get(&textrue).unwrap();
                sprite.index = (sprite.index + 1) % texture_atlas.textures.len();
            }
            let speed = 400.;
            let movement = Vec3::new(
                player_movement.x * speed * time.delta_seconds(),
                player_movement.y * speed * time.delta_seconds(),
                0.,
            );
            player_transform.translation = (player_transform.translation + movement)
                .clamp(Vec3::new(-200., -360., 0.), Vec3::new(200., 360., 0.));
        }
    }
}
