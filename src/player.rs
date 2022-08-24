use benimator::FrameRate;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::audio::GameOverEvent;
use crate::common::animation::{animate, Animation, AnimationState};
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
                .with_system(keyboard_move_player.before(animate))
                .with_system(collision_event),
        );
    }
}

pub fn spawn_player(mut commands: Commands, player_assets: Res<PlayerTextureAtlas>) {
    commands
        .spawn_bundle(SpriteSheetBundle {
            transform: Transform {
                translation: Vec3::new(0., 0., 1.),
                scale: Vec3::new(0.5, 0.5, 0.5),
                ..Default::default()
            },
            texture_atlas: player_assets.up.clone(),
            ..Default::default()
        })
        .insert(Animation::from_indices(0..2, FrameRate::from_fps(5.0)))
        .insert(AnimationState::default())
        .insert(Player { speed: 400. })
        .insert(RigidBody::Dynamic)
        .insert(Collider::capsule_y(10.0, 40.0))
        .insert(ActiveEvents::COLLISION_EVENTS)
        .insert(LockedAxes::ROTATION_LOCKED);
}

fn keyboard_move_player(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    player_assets: Res<PlayerTextureAtlas>,
    mut player_query: Query<(
        &mut Transform,
        &mut AnimationState,
        &Player,
        &mut TextureAtlasSprite,
        &mut Handle<TextureAtlas>,
    )>,
) {
    for (mut player_transform, mut animation, player, mut sprite, mut texture_atlas) in
        &mut player_query
    {
        let mut movement = Vec2::ZERO;
        if keyboard_input.pressed(KeyCode::Up) {
            movement.y += 1.;
        }
        if keyboard_input.pressed(KeyCode::Down) {
            movement.y -= 1.;
        }
        if keyboard_input.pressed(KeyCode::Right) {
            movement.x += 1.;
        }
        if keyboard_input.pressed(KeyCode::Left) {
            movement.x -= 1.;
        }
        movement *= player.speed * time.delta_seconds();
        if movement == Vec2::ZERO {
            animation.stop();
        } else {
            animation.play();
            if movement.x == 0. {
                if texture_atlas.id != player_assets.up.id {
                    *texture_atlas = player_assets.up.clone();
                }
                sprite.flip_y = movement.y < 0.0;
            } else {
                if texture_atlas.id != player_assets.walk.id {
                    *texture_atlas = player_assets.walk.clone();
                }
                sprite.flip_y = false;
                sprite.flip_x = movement.x < 0.0;
            }
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
