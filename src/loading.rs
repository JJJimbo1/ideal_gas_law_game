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
                .with_collection::<GltfAssets>()
                .with_collection::<AudioAssets>()
                .continue_to_state(GameState::Game),
        );
    }
}

// the following asset collections will be loaded during the State `GameState::Loading`
// when done loading, they will be inserted as resources (see https://github.com/NiklasEi/bevy_asset_loader)

#[derive(AssetCollection, Resource)]
pub struct FontAssets {
    #[asset(path = "fonts/FiraSans-Bold.ttf")]
    pub fire_sans: Handle<Font>
}

#[derive(AssetCollection, Resource)]
pub struct GltfAssets {
    // #[asset(path = "models/balloon.glb#Scene0")]
    // pub balloon_gltf: Handle<Scene>,
    // #[asset(path = "models/vacuum_chamber.glb#Scene0")]
    // pub vacuum_chamber: Handle<Scene>,
    #[asset(path = "models/vacuum_chamber_machine.glb#Scene0")]
    pub vacuum_chamber_machine: Handle<Scene>,
    #[asset(path = "models/vacuum_chamber_open.glb#Scene0")]
    pub vacuum_chamber_open: Handle<Scene>,
    #[asset(path = "models/vacuum_chamber_glass.glb#Mesh0/Primitive0")]
    pub vacuum_chamber_glass_mesh: Handle<Mesh>,
    #[asset(path = "models/vacuum_chamber_plunger.glb#Scene0")]
    pub vacuum_chamber_plunger: Handle<Scene>,
}

#[derive(AssetCollection, Resource)]
pub struct AudioAssets {
    #[asset(path = "audio/Gas Sound Effects.ogg")]
    pub gas: Handle<AudioSource>,
}
