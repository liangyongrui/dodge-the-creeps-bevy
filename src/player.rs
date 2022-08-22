use bevy::prelude::*;
use bevy_rapier2d::prelude::{Collider, CollisionGroups, GravityScale, RigidBody};

use crate::actions::Actions;
use crate::common::animation::{update_animation, Animation, AnimationSpriteBundleBuilder};
use crate::loading::PlayerTextureAtlas;
use crate::GameState;

pub struct PlayerPlugin;

#[derive(Component)]
pub struct Player {
    speed: f32,
}

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::Playing).with_system(spawn_player))
            .add_system_set(
                SystemSet::on_update(GameState::Playing)
                    .with_system(move_player.before(update_animation)),
            );
    }
}

fn spawn_player(mut commands: Commands, player_assets: Res<PlayerTextureAtlas>) {
    commands
        .spawn_bundle(
            AnimationSpriteBundleBuilder {
                transform: Transform {
                    translation: Vec3::new(0., 0., 1.),
                    scale: Vec3::new(0.5, 0.5, 0.5),
                    ..Default::default()
                },
                animation: Animation::new(0.2, player_assets.up.clone()),
            }
            .build(),
        )
        .insert(Player { speed: 400. })
        .insert(RigidBody::Dynamic)
        .insert(Collider::capsule_y(20.0, 50.0))
        .insert(GravityScale(0.));
        // .insert(CollisionGroups::default());
}

fn move_player(
    time: Res<Time>,
    actions: Res<Actions>,
    player_assets: Res<PlayerTextureAtlas>,
    mut player_query: Query<(&mut Transform, &mut Animation, &Player)>,
) {
    for (mut player_transform, mut animation, player) in &mut player_query {
        if let Some(player_movement) = actions.player_movement {
            if player_movement != Vec2::ZERO {
                animation.play();
                if player_movement.x == 0. {
                    animation.update_texture_atlas(&player_assets.up);
                    animation.flip_y(player_movement.y < 0.);
                } else {
                    animation.update_texture_atlas(&player_assets.walk);
                    animation.flip_y(false);
                    animation.flip_x(player_movement.x < 0.);
                }
                let movement = player_movement * player.speed * time.delta_seconds();
                player_transform.translation = (player_transform.translation + movement.extend(0.))
                    .clamp(Vec3::new(-200., -360., 0.), Vec3::new(200., 360., 0.));
                continue;
            }
        }
        animation.stop();
    }
}
