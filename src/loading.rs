use crate::GameState;
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_kira_audio::AudioSource;

pub struct LoadingPlugin;

/// This plugin loads all assets using [AssetLoader] from a third party bevy plugin
/// Alternatively you can write the logic to load assets yourself
/// If interested, take a look at https://bevy-cheatbook.github.io/features/assets.html
impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        app.add_loading_state(
            LoadingState::new(GameState::Loading)
                .with_collection::<FontAssets>()
                .with_collection::<AudioAssets>()
                .with_collection::<TextureAssets>()
                .with_collection::<PlayerAssets>()
                .continue_to_state(GameState::Menu),
        )
        .add_system_set(
            SystemSet::on_exit(GameState::Loading).with_system(load_player_texture_atlas),
        );
    }
}

// the following asset collections will be loaded during the State `GameState::Loading`
// when done loading, they will be inserted as resources (see https://github.com/NiklasEi/bevy_asset_loader)

#[derive(AssetCollection)]
pub struct FontAssets {
    #[asset(path = "fonts/FiraSans-Bold.ttf")]
    pub fira_sans: Handle<Font>,
}

#[derive(AssetCollection)]
pub struct AudioAssets {
    #[asset(path = "audio/flying.ogg")]
    pub flying: Handle<AudioSource>,
}

#[derive(AssetCollection)]
pub struct TextureAssets {
    #[asset(path = "textures/bevy.png")]
    pub texture_bevy: Handle<Image>,
}

#[derive(AssetCollection)]
pub struct PlayerAssets {
    #[asset(path = "textures/playerGrey_up1.png")]
    pub player_up1: Handle<Image>,
    #[asset(path = "textures/playerGrey_up2.png")]
    pub player_up2: Handle<Image>,
    #[asset(path = "textures/playerGrey_walk1.png")]
    pub player_walk1: Handle<Image>,
    #[asset(path = "textures/playerGrey_walk2.png")]
    pub player_walk2: Handle<Image>,
}
pub struct PlayerTextureAtlas {
    pub up: Handle<TextureAtlas>,
    pub walk: Handle<TextureAtlas>,
}

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

pub fn load_player_texture_atlas(
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
