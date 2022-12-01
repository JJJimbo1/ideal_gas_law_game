use bevy::prelude::*;
use thousands::Separable;
use crate::*;

pub fn create_container_ui_menu(
    font_assets: Res<FontAssets>,
    mut commands : Commands,
) {
    let debug_menu = ContainerUI::new(&font_assets, &mut commands);
    commands.insert_resource(debug_menu);
}

#[derive(Copy, Clone)]
#[derive(Resource)]
pub struct ContainerUI {
    volume : Entity,
    temperature : Entity,
    moles : Entity,
    pressure: Entity,
}

impl ContainerUI {
    pub fn new(
        font_assets: &FontAssets,
        commands : &mut Commands,
        // mut materials: &mut Assets<ColorMaterial>
    ) -> Self {
        let font = font_assets.fire_sans.clone();

        let font_size = 20.0;
        let mut entity_commands = commands.spawn(NodeBundle {
            style: Style {
                position_type : PositionType::Absolute,
                position: UiRect {
                    top : Val::Px(10.0),
                    right: Val::Px(10.0),
                    ..Default::default()
                },
                size: Size::new(Val::Percent(30.0), Val::Percent(40.0)),
                ..Default::default()
            },
            background_color : DARK_BACKGROUND_COLOR.into(),
            visibility : Visibility { is_visible : true},
            ..Default::default()
        });

        let mut volume = None;

        let entity_commands = entity_commands.with_children(|parent| {
            volume = Some(parent.spawn(TextBundle {
                style: Style {
                    position_type : PositionType::Absolute,
                    position: UiRect {
                        top : Val::Px(10.0),
                        left: Val::Px(10.0),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                text: Text::from_section(
                    "Volume (Liters) : 1.5708",
                    TextStyle {
                        font : font.clone(),
                        font_size,
                        color: Color::WHITE,
                    },
                ),
                visibility : Visibility { is_visible : true},
                ..Default::default()
            }).id());
        });

        let mut temperature = None;

        entity_commands.with_children(|parent| {
            // text
            temperature = Some(parent.spawn(TextBundle {
                style: Style {
                    position_type : PositionType::Absolute,
                    position: UiRect {
                        top : Val::Px(40.0),
                        left: Val::Px(10.0),
                        // top: Val::Px(0.0),
                        ..Default::default()
                    },
                    //margin: UiRect::all(Val::Px(5.0)),
                    ..Default::default()
                },
                text: Text::from_section(
                    "Temperature (Kelvin): 273.15",
                    TextStyle {
                        font : font.clone(),
                        font_size,
                        color: Color::WHITE,
                    },
                ),
                visibility : Visibility { is_visible : true},
                ..Default::default()
            }).id());
        });

        let mut moles = None;

        entity_commands.with_children(|parent| {
            // text
            moles = Some(parent.spawn(TextBundle {
                style: Style {
                    position_type : PositionType::Absolute,
                    position: UiRect {
                        top : Val::Px(70.0),
                        left: Val::Px(10.0),
                        // top: Val::Px(0.0),
                        ..Default::default()
                    },
                    //margin: UiRect::all(Val::Px(5.0)),
                    ..Default::default()
                },
                text: Text::from_section(
                    "Moles: 70.08129",
                    TextStyle {
                        font : font.clone(),
                        font_size,
                        color: Color::WHITE,
                    },
                ),
                visibility : Visibility { is_visible : true},
                ..Default::default()
            }).id());
        });

        let mut pressure = None;

        entity_commands.with_children(|parent| {
            // text
            pressure = Some(parent.spawn(TextBundle {
                style: Style {
                    position_type : PositionType::Absolute,
                    position: UiRect {
                        top : Val::Px(100.0),
                        left: Val::Px(10.0),
                        // top: Val::Px(0.0),
                        ..Default::default()
                    },
                    //margin: UiRect::all(Val::Px(5.0)),
                    ..Default::default()
                },
                text: Text::from_section(
                    "Pressure (Atm): 101,325",
                    TextStyle {
                        font : font.clone(),
                        font_size,
                        color: Color::WHITE,
                    },
                ),
                visibility : Visibility { is_visible : true},
                ..Default::default()
            }).id());
        });

        entity_commands.with_children(|parent| {
            // text
            parent.spawn(TextBundle {
                style: Style {
                    position_type : PositionType::Absolute,
                    position: UiRect {
                        top : Val::Px(130.0),
                        left: Val::Px(10.0),
                        // top: Val::Px(0.0),
                        ..Default::default()
                    },
                    //margin: UiRect::all(Val::Px(5.0)),
                    ..Default::default()
                },
                text: Text::from_section(
                    "Hold Shift and/or\nCtrl to slow down\nHold alt to speed up",
                    TextStyle {
                        font : font.clone(),
                        font_size,
                        color: Color::WHITE,
                    },
                ),
                visibility : Visibility { is_visible : true},
                ..Default::default()
            });
        });

        Self {
            volume : volume.unwrap(),
            temperature : temperature.unwrap(),
            moles: moles.unwrap(),
            pressure: pressure.unwrap()
        }
    }
}

pub fn container_menu_update(
    mut container: ResMut<GasContainer>,
    menu : Res<ContainerUI>,
    mut texts : Query<&mut Text>,
) {

    if let Ok(mut text) = texts.get_mut(menu.volume) {
        text.sections[0].value = format!("Volume (Liters): {}", container.liters);
    }
    if let Ok(mut text) = texts.get_mut(menu.temperature) {
        text.sections[0].value = format!("Temperature (Kelvin): {}", container.kelvin);
    }
    if let Ok(mut text) = texts.get_mut(menu.moles) {
        text.sections[0].value = format!("Moles: {}", container.moles);
    }
    if let Ok(mut text) = texts.get_mut(menu.pressure) {
        let s = ((container.calculate_pressure() * 10000.0).round() / 10000.0).to_string().separate_with_commas();
        text.sections[0].value = format!("Pressure (Atm): {}", s);
    }
}