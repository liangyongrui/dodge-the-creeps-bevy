use bevy::prelude::*;
use bevy::sprite::{SpriteSheetBundle, TextureAtlas, TextureAtlasSprite};
use bevy::time::{Time, Timer};

use crate::GameState;

pub struct AnimationPlugin;

impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_update(GameState::Playing).with_system(update_animation));
    }
}

#[derive(Bundle, Default)]
pub struct AnimationSpriteBundle {
    #[bundle]
    sprite: SpriteSheetBundle,
    animation: Animation,
}

pub struct AnimationSpriteBundleBuilder {
    pub transform: Transform,
    pub animation: Animation,
}
impl AnimationSpriteBundleBuilder {
    pub fn build(self) -> AnimationSpriteBundle {
        AnimationSpriteBundle {
            sprite: SpriteSheetBundle {
                texture_atlas: self.animation.texture_atlas.clone(),
                transform: self.transform,
                ..SpriteSheetBundle::default()
            },
            animation: self.animation,
        }
    }
}

#[derive(Component, Default)]
pub struct Animation {
    play: bool,
    timer: Timer,
    flip_x: bool,
    flip_y: bool,
    texture_atlas: Handle<TextureAtlas>,
}

impl Animation {
    pub fn new(seconds: f32, texture_atlas: Handle<TextureAtlas>) -> Self {
        Self {
            play: true,
            flip_x: false,
            flip_y: false,
            timer: Timer::from_seconds(seconds, true),
            texture_atlas,
        }
    }

    pub fn update_texture_atlas(&mut self, texture_atlas: &Handle<TextureAtlas>) {
        if self.texture_atlas.id != texture_atlas.id {
            self.texture_atlas = texture_atlas.clone();
        }
    }

    pub fn play(&mut self) {
        self.play = true;
    }

    pub fn flip_x(&mut self, b: bool) {
        self.flip_x = b;
    }

    pub fn flip_y(&mut self, b: bool) {
        self.flip_y = b;
    }

    pub fn stop(&mut self) {
        self.play = false;
    }
}

pub fn update_animation(
    time: Res<Time>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut query: Query<(
        &mut Animation,
        &mut TextureAtlasSprite,
        &mut Handle<TextureAtlas>,
    )>,
) {
    for (mut animation, mut sprite, mut textrue) in &mut query {
        if animation.play {
            if textrue.id != animation.texture_atlas.id {
                *textrue = animation.texture_atlas.clone();
            }
            animation.timer.tick(time.delta());
            if animation.timer.just_finished() {
                let texture_atlas = texture_atlases.get(&textrue).unwrap();
                sprite.index = (sprite.index + 1) % texture_atlas.textures.len();
            }
            sprite.flip_x = animation.flip_x;
            sprite.flip_y = animation.flip_y;
        }
    }
}


pub fn general_texture_atlas_handle(
    images: &[&Handle<Image>],
    textures: &mut ResMut<Assets<Image>>,
    texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
) -> Handle<TextureAtlas> {
    let mut texture_atlas_builder = TextureAtlasBuilder::default();
    for &i in images {
        let texture = textures.get(i).unwrap();
        texture_atlas_builder.add_texture(i.clone(), texture);
    }
    texture_atlases.add(texture_atlas_builder.finish(textures).unwrap())
}
