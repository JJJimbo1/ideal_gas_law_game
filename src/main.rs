
use bevy::{prelude::*, gltf::Gltf};

mod constants;
mod gas;
mod utility;
mod balloon;

pub use constants::*;
pub use gas::*;
pub use utility::*;
pub use balloon::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum GameState {
    Balloon,
    AluminumDrum,
    VacuumChamber,
}


fn main() {
    App::new()
        .insert_resource(ClearColor(CLEAR_COLOR))
        // .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_plugin(BalloonPlugin)
        .add_startup_system(create_camera)

        .insert_resource(Random::<WichmannHill>::seeded(0.254623))
        // .add_startup_system(create_splash_screen)
        .run();
}

pub fn create_camera(
    mut commands: Commands,
) {
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 5000.0,
            ..default()
        },
        transform: Transform::from_translation(Vec3::new(0.0, 10.0, 10.0)),
        ..default()
    });

    commands.spawn(Camera3dBundle {
        transform: Transform::from_translation(Vec3::new(0.3, 0.3, 0.3) * 15.0)
            .looking_at(Vec3::new(0.0, 1.0, 0.0), Vec3::Y),
        ..default()
    });
}

#[test]
fn atest() {
    use crate::gas::*;
    let gas = GasContainer::new_volume(4.19);
    println!("{} | {}", gas.cubic_meters , gas.moles);
}