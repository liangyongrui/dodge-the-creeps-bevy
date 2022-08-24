use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::audio::GameOverEvent;
use crate::common::animation::{update_animation, Animation, AnimationSpriteBundleBuilder};
use crate::common::clear_entities;
use crate::loading::PlayerTextureAtlas;
use crate::GameState;

pub struct PlayerPlugin;

#[derive(Component)]
pub struct Player {
    speed: f32,
}

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(GameState::Playing)
                .with_system(clear_entities::<Player>.before(spawn_player))
                .with_system(spawn_player),
        )
        .add_system_set(
            SystemSet::on_update(GameState::Playing)
                .with_system(move_player.before(update_animation))
                .with_system(collision_event),
        );
    }
}

pub fn spawn_player(mut commands: Commands, player_assets: Res<PlayerTextureAtlas>) {
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
        .insert(Collider::capsule_y(10.0, 40.0))
        .insert(ActiveEvents::COLLISION_EVENTS)
        .insert(LockedAxes::ROTATION_LOCKED);
}

fn move_player(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    player_assets: Res<PlayerTextureAtlas>,
    mut player_query: Query<(&mut Transform, &mut Animation, &Player)>,
) {
    for (mut player_transform, mut animation, player) in &mut player_query {
        let mut player_movement = Vec2::ZERO;
        if keyboard_input.pressed(KeyCode::Up) {
            player_movement.y += 1.;
        }
        if keyboard_input.pressed(KeyCode::Down) {
            player_movement.y -= 1.;
        }
        if keyboard_input.pressed(KeyCode::Right) {
            player_movement.x += 1.;
        }
        if keyboard_input.pressed(KeyCode::Left) {
            player_movement.x -= 1.;
        }
        if player_movement == Vec2::ZERO {
            animation.stop();
        } else {
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
        }
    }
}

fn collision_event(
    mut game_over: EventWriter<GameOverEvent>,
    mut events: EventReader<CollisionEvent>,
    mut state: ResMut<State<GameState>>,
) {
    if events
        .iter()
        .filter(|t| matches!(t, CollisionEvent::Started(..)))
        .last()
        .is_some()
    {
        game_over.send(GameOverEvent);
        state.set(GameState::Menu).unwrap();
    }
}
