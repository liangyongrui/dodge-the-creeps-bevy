use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_kira_audio::AudioSource;

use crate::GameState;

fn general_texture_atlas_handle(
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

pub struct LoadingPlugin;

impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        app.add_loading_state(
            LoadingState::new(GameState::Loading)
                .with_collection::<FontAssets>()
                .with_collection::<AudioAssets>()
                .with_collection::<TextureAssets>()
                .with_collection::<PlayerAssets>()
                .with_collection::<EnemyAssets>()
                .continue_to_state(GameState::Menu),
        )
        .add_system_set(
            SystemSet::on_exit(GameState::Loading)
                .with_system(load_player_texture_atlas)
                .with_system(load_enemy_texture_atlas),
        );
    }
}

// the following asset collections will be loaded during the State `GameState::Loading`
// when done loading, they will be inserted as resources (see https://github.com/NiklasEi/bevy_asset_loader)

#[derive(AssetCollection)]
pub struct FontAssets {
    #[asset(path = "fonts/Xolonium-Regular.ttf")]
    pub xolonium_regular: Handle<Font>,
}

#[derive(AssetCollection)]
pub struct AudioAssets {
    #[asset(path = "audio/gameover.wav")]
    pub gameover: Handle<AudioSource>,
}

#[derive(AssetCollection)]
pub struct TextureAssets {
    #[asset(path = "textures/bevy.png")]
    pub texture_bevy: Handle<Image>,
}

#[derive(AssetCollection)]
struct PlayerAssets {
    #[asset(path = "textures/playerGrey_up1.png")]
    player_up1: Handle<Image>,
    #[asset(path = "textures/playerGrey_up2.png")]
    player_up2: Handle<Image>,
    #[asset(path = "textures/playerGrey_walk1.png")]
    player_walk1: Handle<Image>,
    #[asset(path = "textures/playerGrey_walk2.png")]
    player_walk2: Handle<Image>,
}

#[derive(AssetCollection)]
struct EnemyAssets {
    #[asset(path = "textures/enemyFlyingAlt_1.png")]
    fly1: Handle<Image>,
    #[asset(path = "textures/enemyFlyingAlt_2.png")]
    fly2: Handle<Image>,
    #[asset(path = "textures/enemySwimming_1.png")]
    swim1: Handle<Image>,
    #[asset(path = "textures/enemySwimming_2.png")]
    swim2: Handle<Image>,
    #[asset(path = "textures/enemyWalking_1.png")]
    walk1: Handle<Image>,
    #[asset(path = "textures/enemyWalking_2.png")]
    walk2: Handle<Image>,
}

pub struct EnemyTextureAtlas {
    pub fly: Handle<TextureAtlas>,
    pub swim: Handle<TextureAtlas>,
    pub walk: Handle<TextureAtlas>,
}
impl EnemyTextureAtlas {
    pub fn random(&self) -> Handle<TextureAtlas> {
        let i = rand::random::<u8>() % 3;
        if i == 0 {
            self.fly.clone()
        } else if i == 1 {
            self.swim.clone()
        } else {
            self.walk.clone()
        }
    }
}
pub struct PlayerTextureAtlas {
    pub up: Handle<TextureAtlas>,
    pub walk: Handle<TextureAtlas>,
}

fn load_player_texture_atlas(
    mut commands: Commands,
    mut textures: ResMut<Assets<Image>>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    player_assets: Res<PlayerAssets>,
) {
    let up = general_texture_atlas_handle(
        &[&player_assets.player_up1, &player_assets.player_up2],
        &mut textures,
        &mut texture_atlases,
    );
    let walk = general_texture_atlas_handle(
        &[&player_assets.player_walk1, &player_assets.player_walk2],
        &mut textures,
        &mut texture_atlases,
    );
    commands.insert_resource(PlayerTextureAtlas { up, walk });
}

fn load_enemy_texture_atlas(
    mut commands: Commands,
    mut textures: ResMut<Assets<Image>>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    enemy_assets: Res<EnemyAssets>,
) {
    let fly = general_texture_atlas_handle(
        &[&enemy_assets.fly1, &enemy_assets.fly2],
        &mut textures,
        &mut texture_atlases,
    );
    let swim = general_texture_atlas_handle(
        &[&enemy_assets.swim1, &enemy_assets.swim2],
        &mut textures,
        &mut texture_atlases,
    );
    let walk = general_texture_atlas_handle(
        &[&enemy_assets.walk1, &enemy_assets.walk2],
        &mut textures,
        &mut texture_atlases,
    );
    commands.insert_resource(EnemyTextureAtlas { fly, swim, walk });
}
