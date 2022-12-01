use bevy::prelude::*;
use bevy_kira_audio::{Audio, AudioInstance, AudioControl, AudioTween};
use bevy_rapier3d::prelude::{Collider, RapierContext, QueryFilter};

use crate::{GameState, GasContainer, GltfAssets, AudioAssets, CLEAR_COLOR, CookingLight, shapes, create_container_ui_menu, container_menu_update};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
        .add_system_set(
            SystemSet::on_enter(GameState::Game)
                .with_system(setup_chamber)
                .with_system(create_container_ui_menu)
        )
        .add_system_set(
            SystemSet::on_update(GameState::Game)
                // .with_system(flash_machine)
                .with_system(update_chamber)
                .with_system(process_events.after(update_chamber))
                .with_system(container_menu_update.after(process_events))
        );
    }
}

#[derive(Debug, Clone, Copy)]
#[derive(Component)]
pub struct Machine;

#[derive(Debug, Clone, Copy)]
#[derive(Component)]
pub enum VacuumButton {
    PlungerUp,
    PlungerDown,
    HeatUp,
    HeatDown,
    GasUp,
    GasDown,
}

#[derive(Debug, Clone)]
#[derive(Resource)]
pub struct GasAudio(Handle<AudioInstance>);

#[derive(Debug, Clone, Copy)]
#[derive(Component)]
pub struct PlungerEvent(f32);

#[derive(Debug, Clone, Copy)]
#[derive(Component)]
pub struct HeatEvent(f32);

#[derive(Debug, Clone, Copy)]
#[derive(Component)]
pub struct GasEvent(f32);

#[derive(Debug, Clone, Copy)]
#[derive(Component)]
pub struct Plunger;

fn setup_chamber(
    assets: Res<GltfAssets>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    audio_assets: Res<AudioAssets>,
    audio: Res<Audio>,
    mut commands: Commands,
) {
    audio.pause();

    commands.spawn(SceneBundle {
        scene: assets.vacuum_chamber_machine.clone(),
        // visibility: Visibility::INVISIBLE,
        ..default()
    })
    .insert(Machine)
    .with_children(|parent| {

        parent.spawn(TransformBundle {
            local: Transform::from_xyz(1.1, 1.67651, 1.57772),
            ..default()
        })
        .insert(Collider::cuboid(0.01, 0.153099, 0.153099))
        .insert(VacuumButton::PlungerUp);

        parent.spawn(TransformBundle {
            local: Transform::from_xyz(1.1, 1.33881, 1.57772),
            ..default()
        })
        .insert(Collider::cuboid(0.01, 0.153099, 0.153099))
        .insert(VacuumButton::PlungerDown);

        parent.spawn(TransformBundle {
            local: Transform::from_xyz(1.09659, 0.425882, 0.229338),
            ..default()
        })
        .insert(Collider::cuboid(0.048089, 0.151086, 0.120788))
        .insert(VacuumButton::HeatUp);

        parent.spawn(TransformBundle {
            local: Transform::from_xyz(1.09659, 0.425882, -0.232219),
            ..default()
        })
        .insert(Collider::cuboid(0.048089, 0.151086, 0.120788))
        .insert(VacuumButton::HeatDown);

        parent.spawn(TransformBundle {
            local: Transform::from_xyz(0.001593, 1.32588, -1.71815),
            ..default()
        })
        .insert(Collider::cuboid(0.048089, 0.151086, 0.120788))
        .insert(VacuumButton::GasUp);

        parent.spawn(TransformBundle {
            local: Transform::from_xyz(0.001593, 1.32588, -2.18077),
            ..default()
        })
        .insert(Collider::cuboid(0.048089, 0.151086, 0.120788))
        .insert(VacuumButton::GasDown);
    });

    commands.spawn(SceneBundle{
        scene: assets.vacuum_chamber_open.clone(),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        ..default()
    });

    commands.spawn(PbrBundle {
        mesh: assets.vacuum_chamber_glass_mesh.clone(),
        material: materials.add(StandardMaterial {
            base_color: Color::rgba(0.3, 0.7, 1.0, 0.5),
            alpha_mode: AlphaMode::Blend,
            ..default()
        }),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        ..default()
    });

    commands.spawn(SceneBundle {
        scene: assets.vacuum_chamber_plunger.clone(),
        transform: Transform::from_xyz(0.0, 2.575, 0.0),
        ..default()
    }).insert(Plunger);


    commands.insert_resource(ClearColor(CLEAR_COLOR));
    commands.insert_resource(GasContainer::new_volume(1570.8));
    let handle = audio.play(audio_assets.gas.clone()).looped().with_volume(0.3).handle();
    commands.insert_resource(GasAudio(handle));
}

fn update_chamber(
    mut plunger_events: EventWriter<PlungerEvent>,
    mut heat_events: EventWriter<HeatEvent>,
    mut gas_events: EventWriter<GasEvent>,
    input_keys: Res<Input<KeyCode>>,
    input_mouse_buttons: Res<Input<MouseButton>>,
    windows: Res<Windows>,
    cameras : Query<(&GlobalTransform, &Camera)>,
    context : Res<RapierContext>,
    buttons: Query<&VacuumButton>,
) {
    if input_mouse_buttons.pressed(MouseButton::Left) {
        let Some(mouse_position) = windows.get_primary().and_then(|window| window.cursor_position()) else { return; };
        let (transform, camera) = cameras.single();
        let Some(ray) = camera.viewport_to_world(transform, mouse_position) else { return; };
        let Some((entity, _)) = context.cast_ray(ray.origin, ray.direction, 1000.0, true, QueryFilter::new()) else { return; };
        let Ok(button) = buttons.get(entity) else { return; };
        let shift: f32 = if input_keys.pressed(KeyCode::LShift) || input_keys.pressed(KeyCode::RShift) { 0.1 } else { 1.0 };
        let ctrl: f32 = if input_keys.pressed(KeyCode::LControl) || input_keys.pressed(KeyCode::RControl) { 0.01 } else { 1.0 };
        let alt: f32 = if input_keys.pressed(KeyCode::LAlt) || input_keys.pressed(KeyCode::RAlt) { 10.0 } else { 1.0 };
        let slow = shift * ctrl * alt;
        match button {
            VacuumButton::PlungerUp => { plunger_events.send(PlungerEvent((0.001 * slow).max(0.000001))); },
            VacuumButton::PlungerDown => { plunger_events.send(PlungerEvent((-0.001 * slow).min(-0.000001))); },
            VacuumButton::HeatUp => { heat_events.send(HeatEvent((1.0 * slow).max(0.0001))); },
            VacuumButton::HeatDown => { heat_events.send(HeatEvent((-1.0 * slow).min(-0.0001))); },
            VacuumButton::GasUp => {gas_events.send(GasEvent((0.5 * slow).max(0.0001))); },
            VacuumButton::GasDown => {gas_events.send(GasEvent((-0.5 * slow).min(-0.0001))); },
        }
    }

}

fn process_events(
    mut playing: Local<bool>,
    mut plunger_events: EventReader<PlungerEvent>,
    mut heat_events: EventReader<HeatEvent>,
    mut gas_events: EventReader<GasEvent>,
    mut gas_container: ResMut<GasContainer>,
    mut plungers: Query<&mut Transform, With<Plunger>>,
    mut lights: Query<(&mut Visibility, &mut PointLight), With<CookingLight>>,
    gas_audio: Res<GasAudio>,
    mut audio_instances: ResMut<Assets<AudioInstance>>,

) {
    let playing_last_frame = *playing;
    let mut transform = plungers.single_mut();
    for event in plunger_events.iter() {
        transform.translation.y += event.0;
        transform.translation.y = (transform.translation.y.clamp(0.775, 2.575) * 10000000.0).round() / 10000000.0;
    }
    let (mut visibility, mut light) = lights.single_mut();
    visibility.is_visible = false;
    for event in heat_events.iter() {
        if event.0 > 0.0 {
            visibility.is_visible = true;
            light.color = Color::ORANGE_RED;
        } else if event.0 < 0.0 {
            visibility.is_visible = true;
            light.color = Color::rgba(0.15, 0.45, 1.0, 1.0);
        }
        gas_container.kelvin += event.0;
    }

    *playing = false;
    for event in gas_events.iter() {
        *playing = true;
        gas_container.moles += event.0;
    }

    if !playing_last_frame && *playing {
        let Some(instance) = audio_instances.get_mut(&gas_audio.0) else { return; };
        instance.resume(AudioTween::default());
    } else if playing_last_frame && !*playing {
        let Some(instance) = audio_instances.get_mut(&gas_audio.0) else { return; };
        instance.pause(AudioTween::default());
    }

    gas_container.liters = (shapes::cylinder::liters_from_radius_and_height(0.5, transform.translation.y - 0.575) * 10000.0).round() / 10000.0;
    gas_container.kelvin = (gas_container.kelvin.clamp(1.0, 1000.0) * 10000.0).round() / 10000.0;
    gas_container.moles = (gas_container.moles.clamp(0.5, 350.0) * 10000.0).round() / 10000.0;
}

// #[test]
// fn atest() {
//     use crate::gas::GasContainer;

//     let container = GasContainer::new_volume(1.5708);

//     println!("{}", container.moles);

// }

// #[test]
// fn atest() {
//     use std::fs::File;
//     use std::io::Write;
//     let mut nums: [u8; 96] = [0; 96];
//     for i in 32..128 {
//         nums[i-32] = i as u8;
//     }
//     let ch = String::from_utf8(nums.to_vec()).unwrap();
//     println!("STRING: <{}>", ch);

//     let mut file = File::create("a_file.txt").unwrap();
//     write!(file, "{}", ch);
// }