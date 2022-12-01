
use bevy::prelude::*;
use bevy_rapier3d::{prelude::{RapierPhysicsPlugin, NoUserData}, render::RapierDebugRenderPlugin};
use bevy_kira_audio::AudioPlugin;

mod loading;
mod constants;
mod gas;
mod utility;
mod chamber;
mod ui;

pub use loading::*;
pub use constants::*;
pub use gas::*;
pub use utility::*;
pub use chamber::*;
pub use ui::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum GameState {
    Loading,
    Game,
    AluminumDrum,
    VacuumChamber,
}


fn main() {
    App::new()
        .insert_resource(ClearColor(LOADING_CLEAR_COLOR))
        // .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                width: 800.0,
                height: 600.0,
                title: "ideal_gas_law_simulator".to_string(),
                canvas: Some("#bevy".to_owned()),
                ..default()
            },
            ..default()
        }))
        .add_plugin(AudioPlugin)

        .add_event::<PlungerEvent>()
        .add_event::<HeatEvent>()
        .add_event::<GasEvent>()

        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_plugin(LoadingPlugin)
        .add_plugin(GamePlugin)
        .add_startup_system(create_camera)

        .insert_resource(Random::<WichmannHill>::seeded(0.254623))
        .add_state(GameState::Loading)
        // .add_startup_system(create_splash_screen)
        .run();
}

#[derive(Debug, Clone, Copy)]
#[derive(Component)]
pub struct CookingLight;

pub fn create_camera(
    mut commands: Commands,
) {
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: 5000.0,
            ..default()
        },
        transform: Transform::from_rotation(Quat::from_euler(EulerRot::XYZ, 0.0, 1.4, 2.5)),
        ..default()
    });

    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 50.0,
            color: Color::Rgba { red: 1.0, green: 0.0, blue: 1.0, alpha: 1.0 },
            ..default()
        },
        transform: Transform::from_xyz(0.0, 0.35, 0.0),
        ..default()
    }).insert(CookingLight);

    commands.spawn(Camera3dBundle {
        transform: Transform::from_translation(Vec3::new(5.0, 2.65, 0.0) * 1.0)
            .looking_at(Vec3::new(0.0, 1.65, 0.0), Vec3::Y),
        ..default()
    });
}

// #[test]
// fn atest() {
//     use crate::gas::*;
//     let gas = GasContainer::new_volume(4.19);
//     println!("{} | {}", gas.cubic_meters , gas.moles);
// }