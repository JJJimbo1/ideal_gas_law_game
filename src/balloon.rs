use bevy::prelude::*;

use crate::{GameState, GasContainer, math_fu};


pub struct BalloonPop;


pub struct BalloonPlugin;

impl Plugin for BalloonPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<BalloonPop>()
        .add_system_set(
            SystemSet::on_enter(GameState::Balloon)
                .with_system(create_balloon)
        )
        .add_system_set(
            SystemSet::on_update(GameState::Balloon)
                .with_system(setup_scene_once_loaded)
                .with_system(update_balloon)
        );
    }
}

#[derive(Debug, Clone)]
#[derive(Resource)]
pub struct BalloonAnimation(Handle<AnimationClip>);

fn create_balloon(
    assets: Res<AssetServer>,
    mut commands: Commands,
) {
    let animation = assets.load("models/balloon.glb#Animation0");
    commands.insert_resource(BalloonAnimation(animation));

    let model = assets.load("models/balloon.glb#Scene0");

    commands.spawn(SceneBundle{
        scene: model,
        ..default()
    });

    commands.insert_resource(GasContainer::new_volume(0.52360));
}

fn setup_scene_once_loaded(
    animations: Res<BalloonAnimation>,
    mut player: Query<&mut AnimationPlayer>,
    mut done: Local<bool>,
) {
    if !*done {


        println!("Not yet loaded");
        if let Ok(mut player) = player.get_single_mut() {
            println!("loaded");
            player.play(animations.0.clone_weak());
            *done = true;
        }
    }
}

fn update_balloon(
    mut balloon_pop_event_writer: EventWriter<BalloonPop>,
    mut gas_container: ResMut<GasContainer>,
    input: Res<Input<KeyCode>>,
    mut player: Query<&mut AnimationPlayer>,
) {
    if input.pressed(KeyCode::Up) {
        gas_container.moles += 0.2;
    }

    if input.pressed(KeyCode::Down) {
        gas_container.moles -= 0.2;
    }
    gas_container.moles = gas_container.moles.clamp(24.360432, 185.937);
    
    if let Ok(mut player) = player.get_single_mut() {
        
        let volume = gas_container.calculate_volume();
        let progress = math_fu::D1::diameter_from_volume(volume) - 1.0;
        println!("{} | {}", progress, gas_container.moles);
        if progress >= 0.99 {
            balloon_pop_event_writer.send(BalloonPop);
        }
        player.set_elapsed(progress);
    }


}