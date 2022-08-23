use std::f32::consts::PI;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use rand::Rng;

use crate::common::animation::{update_animation, Animation, AnimationSpriteBundleBuilder};
use crate::common::clear_entities;
use crate::loading::EnemyTextureAtlas;
use crate::{GameState, ScreenPath};

pub struct EnemyPlugin;

#[derive(Component)]
pub struct Enemy {
    direction: Vec2,
    /// 每秒移动距离
    speed: f32,
}

#[derive(Deref, DerefMut)]
struct EnemyTimer(Timer);

impl Default for EnemyTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(0.5, true))
    }
}

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<EnemyTimer>()
            .add_system_set(
                SystemSet::on_enter(GameState::Playing).with_system(clear_entities::<Enemy>),
            )
            .add_system_set(
                SystemSet::on_update(GameState::Playing)
                    .with_system(move_enemy.before(update_animation))
                    .with_system(spawn_enemy),
            );
    }
}

fn spawn_enemy(
    time: Res<Time>,
    mut timer: ResMut<EnemyTimer>,
    mut commands: Commands,
    enemy_assets: Res<EnemyTextureAtlas>,
    path: Res<ScreenPath>,
) {
    timer.tick(time.delta());
    if !timer.just_finished() {
        return;
    }
    let seg = path.random_segment();
    let r = seg.rotation();
    let translation = seg.random_point();
    let mut rng = rand::thread_rng();
    let direction =
        Vec2::new(r.y, -r.x).rotate(Vec2::from_angle(rng.gen_range(-PI / 4.0..PI / 4.0)));
    let speed = rng.gen_range(200.0..600.);

    commands
        .spawn_bundle(
            AnimationSpriteBundleBuilder {
                transform: Transform {
                    translation: translation.extend(0.),
                    scale: Vec3::new(0.5, 0.5, 0.5),
                    // rotation: Quat::from_xyzw(direction.x, direction.y, 0., 1.),
                    rotation: Default::default(),
                },
                animation: Animation::new(0.2, enemy_assets.random()),
            }
            .build(),
        )
        .insert(Enemy { direction, speed })
        .insert(RigidBody::Dynamic)
        .insert(Collider::capsule_x(20.0, 50.0))
        .insert(CollisionGroups::new(0b1, 0b10));
}

fn move_enemy(
    mut commands: Commands,
    time: Res<Time>,
    mut enemy_query: Query<(&mut Transform, &Enemy, Entity), With<Enemy>>,
) {
    for (mut enemy_transform, enemy, entity) in &mut enemy_query {
        enemy_transform.translation +=
            enemy.speed * time.delta_seconds() * enemy.direction.extend(0.);
        if !(-250.0..250.).contains(&enemy_transform.translation.x)
            || !(-400.0..400.).contains(&enemy_transform.translation.y)
        {
            commands.entity(entity).despawn();
        }
    }
}
