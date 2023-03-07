// use super::parts::custom_button::CustomButton;
use super::{
    despawn_screen, GameState, HOVERED_BUTTON_COLOR, HOVERED_PRESSED_BUTTON_COLOR,
    MENU_BACKGROUND_COLOR, MENU_FONT, MENU_TEXT_COLOR, NORMAL_BUTTON_COLOR, PRESSED_BUTTON_COLOR,
};

use bevy::{app::AppExit, prelude::*};
use std::fmt::Debug;

#[derive(Debug, Component, PartialEq, Eq, Clone, Copy, Resource)]
pub enum DisplayQuality {
    Low,
    Medium,
    High,
}

#[derive(Debug, Component, PartialEq, Eq, Clone, Copy, Resource)]
pub struct Volume(pub u32);

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
enum MenuState {
    Main,
    Settings,
    SettingsDisplay,
    SettingsSound,
    Disabled,
}

#[derive(Component)]
enum MenuButtonAction {
    Play,
    Settings,
    SettingsDisplay,
    SettingsSound,
    BackToMainMenu,
    BackToSettings,
    Quit,
}

#[derive(Component)]
struct OnMainMenuScreen;

#[derive(Component)]
struct OnSettingsMenuScreen;

#[derive(Component)]
struct OnDisplaySettingMenuScreen;

#[derive(Component)]
struct OnSoundSettingsMenuScreen;

#[derive(Component)]
struct SelectedOption;

pub struct MainMenuPlugin;

struct MenuButton {
    text_style: TextStyle,
    style: Style,
    icon_style: Option<Style>,
}

impl MenuButton {
    fn plain(asset_server: Res<AssetServer>) -> MenuButton {
        let font = asset_server.load(MENU_FONT);
        let button_style = Style {
            size: Size::new(Val::Px(250.0), Val::Px(65.0)),
            margin: UiRect::all(Val::Px(20.0)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..Default::default()
        };

        let button_text_style = TextStyle {
            font: font.clone(),
            font_size: 20.0,
            color: MENU_TEXT_COLOR,
        };

        MenuButton {
            text_style: button_text_style,
            style: button_style,
            icon_style: None,
        }
    }

    fn with_icon(asset_server: &Res<AssetServer>) -> MenuButton {
        let font = asset_server.load(MENU_FONT);
        let button_style = Style {
            size: Size::new(Val::Px(250.0), Val::Px(65.0)),
            margin: UiRect::all(Val::Px(20.0)),
            justify_content: JustifyContent::FlexEnd,
            align_items: AlignItems::Center,
            ..Default::default()
        };

        let button_icon_style = Style {
            size: Size::new(Val::Px(30.0), Val::Auto),
            position_type: PositionType::Absolute,
            position: UiRect {
                left: Val::Px(10.0),
                right: Val::Auto,
                top: Val::Auto,
                bottom: Val::Auto,
            },
            ..Default::default()
        };

        let button_text_style = TextStyle {
            font: font.clone(),
            font_size: 40.0,
            color: MENU_TEXT_COLOR,
        };

        MenuButton {
            text_style: button_text_style,
            style: button_style,
            icon_style: Some(button_icon_style),
        }
    }
}

fn button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, Option<&SelectedOption>),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut color, selected) in &mut interaction_query {
        *color = match (interaction, selected) {
            (Interaction::Clicked, _) | (Interaction::None, Some(_)) => PRESSED_BUTTON_COLOR.into(),
            (Interaction::Hovered, Some(_)) => HOVERED_PRESSED_BUTTON_COLOR.into(),
            (Interaction::Hovered, None) => HOVERED_BUTTON_COLOR.into(),
            _ => NORMAL_BUTTON_COLOR.into(),
        }
    }
}

fn setting_button<T: Component + PartialEq + Copy + Resource>(
    interaction_query: Query<(&Interaction, &T, Entity), (Changed<Interaction>, With<Button>)>,
    mut selected_query: Query<(Entity, &mut BackgroundColor), With<SelectedOption>>,
    mut commands: Commands,
    mut setting: ResMut<T>,
) {
    for (interaction, button_setting, entity) in &interaction_query {
        if Interaction::Clicked == *interaction && *setting != *button_setting {
            let (previous_button, mut previous_color) = selected_query.single_mut();
            *previous_color = NORMAL_BUTTON_COLOR.into();
            commands.entity(previous_button).remove::<SelectedOption>();
            commands.entity(entity).insert(SelectedOption);
            *setting = *button_setting;
        }
    }
}

fn menu_setup(mut menu_state: ResMut<State<MenuState>>) {
    let _ = menu_state.set(MenuState::Main);
}

fn main_menu_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let button = MenuButton::with_icon(&asset_server);
    let font_style = TextStyle {
        font: asset_server.load(MENU_FONT),
        font_size: 80.0,
        color: MENU_TEXT_COLOR,
    };

    commands
        .spawn(NodeBundle {
            style: Style {
                margin: UiRect::all(Val::Auto),
                flex_direction: FlexDirection::ColumnReverse,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            background_color: Color::CRIMSON.into(),
            ..Default::default()
        })
        .insert(OnMainMenuScreen)
        .with_children(|p| {
            p.spawn(
                // Game Title
                TextBundle::from_section("Project Velour", font_style.clone()).with_style(Style {
                    margin: UiRect::all(Val::Px(80.0)),
                    ..Default::default()
                }),
            );
            // Menu Item 1
            p.spawn(ButtonBundle {
                style: button.style.clone(),
                background_color: NORMAL_BUTTON_COLOR.into(),
                ..Default::default()
            })
            .insert(MenuButtonAction::Play)
            .with_children(|p| {
                let icon = asset_server.load("icons/right.png");
                p.spawn(ImageBundle {
                    style: button.icon_style.clone().unwrap(),
                    image: UiImage(icon.clone()),
                    ..Default::default()
                });
                p.spawn(TextBundle::from_section("Play Game", font_style.clone()));
            });
            // Menu Item 2
            p.spawn(ButtonBundle {
                style: button.style.clone(),
                background_color: NORMAL_BUTTON_COLOR.into(),
                ..Default::default()
            })
            .insert(MenuButtonAction::Settings)
            .with_children(|p| {
                let icon = asset_server.load("icons/wrench.png");
                p.spawn(ImageBundle {
                    style: button.icon_style.clone().unwrap(),
                    image: UiImage(icon.clone()),
                    ..Default::default()
                });
                p.spawn(TextBundle::from_section("Settings", font_style.clone()));
            });
            // Item 3
            p.spawn(ButtonBundle {
                style: button.style.clone(),
                background_color: NORMAL_BUTTON_COLOR.into(),
                ..Default::default()
            })
            .insert(MenuButtonAction::Quit)
            .with_children(|p| {
                let icon = asset_server.load("icons/exitRight.png");
                p.spawn(ImageBundle {
                    style: button.icon_style.clone().unwrap(),
                    image: UiImage(icon.clone()),
                    ..Default::default()
                });
                p.spawn(TextBundle::from_section("Quit", font_style.clone()));
            });
        });
}

fn settings_menu_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let button = MenuButton::plain(asset_server);

    commands
        .spawn((
            NodeBundle {
                style: Style {
                    margin: UiRect::all(Val::Auto),
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    ..Default::default()
                },
                background_color: Color::CRIMSON.into(),
                ..Default::default()
            },
            OnSettingsMenuScreen,
        ))
        .with_children(|p| {
            for (action, text) in [
                (MenuButtonAction::SettingsDisplay, "Display"),
                (MenuButtonAction::SettingsSound, "Sound"),
                (MenuButtonAction::BackToMainMenu, "Back"),
            ] {
                p.spawn((
                    ButtonBundle {
                        style: button.style.clone(),
                        background_color: NORMAL_BUTTON_COLOR.into(),
                        ..Default::default()
                    },
                    action,
                ))
                .with_children(|p| {
                    p.spawn(TextBundle::from_section(text, button.text_style.clone()));
                });
            }
        });
}

// fn generic_settings_menu_setup<F: Fn(&mut ChildBuilder), T: Bundle + Component>(
//     mut commands: Commands,
//     asset_server: Res<AssetServer>,
//     setting: Res<Setting>,
//     setting_set: &mut [T],
//     child_modifier: F,
// ) {
//     // TODO Condense
//     let button_style = Style {
//         size: Size::new(Val::Px(200.0), Val::Px(50.0)),
//         margin: UiRect::all(Val::Px(20.0)),
//         justify_content: JustifyContent::Center,
//         align_items: AlignItems::Center,
//         ..Default::default()
//     };

//     let button_text_style = TextStyle {
//         font: asset_server.load(MENU_FONT),
//         font_size: 35.0,
//         color: MENU_TEXT_COLOR,
//     };

//     commands
//         .spawn((
//             NodeBundle {
//                 style: Style {
//                     margin: UiRect::all(Val::Auto),
//                     flex_direction: FlexDirection::Column,
//                     align_items: AlignItems::Center,
//                     ..Default::default()
//                 },
//                 background_color: MENU_BACKGROUND_COLOR,
//                 ..Default::default()
//             },
//             OnDisplaySettingMenuScreen,
//         ))
//         .with_children(|p| {
//             p.spawn(NodeBundle {
//                 style: Style {
//                     align_items: AlignItems::Center,
//                     ..Default::default()
//                 },
//                 background_color: MENU_BACKGROUND_COLOR,
//                 ..Default::default()
//             })
//             .with_children(|p| {
//                 p.spawn(TextBundle::from_section(
//                     // {text}
//                     "Display Quality",
//                     button_text_style,
//                 ));
//                 // for setting in {set}
//                 for s_s in setting_set {
//                     let mut entity = p.spawn(ButtonBundle {
//                         style: Style {
//                             size: Size::new(Val::Px(150.0), Val::Px(65.0)),
//                             ..button_style
//                         },
//                         background_color: NORMAL_BUTTON_COLOR.into(),
//                         ..Default::default()
//                     });
//                     // Use generic in decleration to pass closure as parameter
//                     entity.insert(*s_s).with_children(|p| {
//                         child_modifier(p);
//                         // p.spawn(TextBundle::from_section(
//                         //     format!("{s_s:?}"),
//                         //     button_text_style,
//                         // ));
//                     });
//                     if *setting == Setting::new(s_s) {
//                         entity.insert(SelectedOption);
//                     }
//                 }
//             });
//             p.spawn((
//                 ButtonBundle {
//                     style: button_style,
//                     background_color: NORMAL_BUTTON_COLOR.into(),
//                     ..Default::default()
//                 },
//                 MenuButtonAction::BackToSettings,
//             ))
//             .with_children(|p| {
//                 p.spawn(TextBundle::from_section("Back", button_text_style));
//             });
//         });
// }

fn display_settings_menu_setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    display_quality: Res<DisplayQuality>,
) {
    let setting_set = vec![
        DisplayQuality::Low,
        DisplayQuality::Medium,
        DisplayQuality::High,
    ];

    // fn modifier(p: &mut ChildBuilder) {
    //     p.spawn(TextBundle::from_section(
    //         format!("{quality_setting:?}"),
    //         button_text_style,
    //     ));
    // }
    // generic_settings_menu_setup(
    //     commands,
    //     asset_server,
    //     display_quality,
    //     setting_set,
    //     modifier(),
    // );

    let button_style = Style {
        size: Size::new(Val::Px(200.0), Val::Px(50.0)),
        margin: UiRect::all(Val::Px(20.0)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..Default::default()
    };

    let button_text_style = TextStyle {
        font: asset_server.load(MENU_FONT),
        font_size: 35.0,
        color: MENU_TEXT_COLOR,
    };

    commands
        .spawn((
            NodeBundle {
                style: Style {
                    margin: UiRect::all(Val::Auto),
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    ..Default::default()
                },
                background_color: MENU_BACKGROUND_COLOR,
                ..Default::default()
            },
            OnDisplaySettingMenuScreen,
        ))
        .with_children(|p| {
            p.spawn(NodeBundle {
                style: Style {
                    align_items: AlignItems::Center,
                    ..Default::default()
                },
                background_color: MENU_BACKGROUND_COLOR,
                ..Default::default()
            })
            .with_children(|p| {
                p.spawn(TextBundle::from_section(
                    // {text}
                    "Display Quality",
                    button_text_style.clone(),
                ));
                // for setting in {set}
                for quality_setting in [
                    DisplayQuality::Low,
                    DisplayQuality::Medium,
                    DisplayQuality::High,
                ] {
                    let mut entity = p.spawn(ButtonBundle {
                        style: Style {
                            size: Size::new(Val::Px(150.0), Val::Px(65.0)),
                            ..button_style
                        },
                        background_color: NORMAL_BUTTON_COLOR.into(),
                        ..Default::default()
                    });
                    // Use generic in decleration to pass closure as parameter
                    entity.insert(quality_setting).with_children(|p| {
                        p.spawn(TextBundle::from_section(
                            format!("{quality_setting:?}"),
                            button_text_style.clone(),
                        ));
                    });
                    if *display_quality == quality_setting {
                        entity.insert(SelectedOption);
                    }
                }
            });
            p.spawn((
                ButtonBundle {
                    style: button_style,
                    background_color: NORMAL_BUTTON_COLOR.into(),
                    ..Default::default()
                },
                MenuButtonAction::BackToSettings,
            ))
            .with_children(|p| {
                p.spawn(TextBundle::from_section("Back", button_text_style.clone()));
            });
        });
}

fn sound_setting_menu_setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    volume: Res<Volume>,
) {
    let button_style = Style {
        size: Size {
            width: Val::Px(200.0),
            height: Val::Px(65.0),
        },
        margin: UiRect::all(Val::Px(20.0)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    };

    let button_text_style = TextStyle {
        font: asset_server.load(MENU_FONT),
        font_size: 40.0,
        ..Default::default()
    };

    commands
        .spawn((
            NodeBundle {
                style: Style {
                    margin: UiRect::all(Val::Auto),
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    ..Default::default()
                },
                background_color: MENU_BACKGROUND_COLOR,
                ..Default::default()
            },
            OnSoundSettingsMenuScreen,
        ))
        .with_children(|p| {
            p.spawn(NodeBundle {
                style: Style {
                    align_items: AlignItems::Center,
                    ..Default::default()
                },
                background_color: MENU_BACKGROUND_COLOR,
                ..Default::default()
            })
            .with_children(|p| {
                p.spawn(TextBundle::from_section(
                    "Volume",
                    button_text_style.clone(),
                ));

                for vol_setting in [0, 1, 2, 3, 4, 5, 6, 7, 8, 9] {
                    let mut entity = p.spawn(ButtonBundle {
                        style: Style {
                            size: Size::new(Val::Px(30.0), Val::Px(65.0)),
                            ..button_style.clone()
                        },
                        background_color: MENU_BACKGROUND_COLOR,
                        ..Default::default()
                    });

                    entity.insert(Volume(vol_setting));
                    if *volume == Volume(vol_setting) {
                        entity.insert(SelectedOption);
                    }
                }
            });

            p.spawn((
                ButtonBundle {
                    style: button_style,
                    background_color: NORMAL_BUTTON_COLOR.into(),
                    ..Default::default()
                },
                MenuButtonAction::BackToSettings,
            ))
            .with_children(|p| {
                p.spawn(TextBundle::from_section("Back", button_text_style));
            });
        });
}

fn menu_action(
    query: Query<(&Interaction, &MenuButtonAction), (Changed<Interaction>, With<Button>)>,
    mut app_exit_events: EventWriter<AppExit>,
    mut menu_state: ResMut<State<MenuState>>,
    mut game_state: ResMut<State<GameState>>,
) {
    for (interaction, menu_button_action) in &query {
        if *interaction == Interaction::Clicked {
            match menu_button_action {
                MenuButtonAction::Play => {
                    game_state.set(GameState::Game).unwrap();
                    menu_state.set(MenuState::Disabled).unwrap();
                }
                MenuButtonAction::Settings => menu_state.set(MenuState::Settings).unwrap(),
                MenuButtonAction::SettingsDisplay => {
                    menu_state.set(MenuState::SettingsDisplay).unwrap();
                }
                MenuButtonAction::SettingsSound => {
                    menu_state.set(MenuState::SettingsSound).unwrap();
                }
                MenuButtonAction::BackToMainMenu => menu_state.set(MenuState::Main).unwrap(),
                MenuButtonAction::BackToSettings => menu_state.set(MenuState::Settings).unwrap(),
                _ => app_exit_events.send(AppExit),
            }
        }
    }
}

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_state(MenuState::Disabled)
            .add_system_set(SystemSet::on_enter(GameState::Menu).with_system(menu_setup))
            // Main Menu
            .add_system_set(SystemSet::on_enter(MenuState::Main).with_system(main_menu_setup))
            .add_system_set(
                SystemSet::on_exit(MenuState::Main).with_system(despawn_screen::<OnMainMenuScreen>),
            )
            // Settings Menu
            .add_system_set(
                SystemSet::on_enter(MenuState::Settings).with_system(settings_menu_setup),
            )
            .add_system_set(
                SystemSet::on_exit(MenuState::Settings)
                    .with_system(despawn_screen::<OnSettingsMenuScreen>),
            )
            .add_system_set(
                SystemSet::on_enter(MenuState::SettingsDisplay)
                    .with_system(display_settings_menu_setup),
            )
            .add_system_set(
                SystemSet::on_update(MenuState::SettingsDisplay)
                    .with_system(setting_button::<DisplayQuality>),
            )
            .add_system_set(
                SystemSet::on_exit(MenuState::SettingsDisplay)
                    .with_system(despawn_screen::<OnDisplaySettingMenuScreen>),
            )
            .add_system_set(
                SystemSet::on_enter(MenuState::SettingsSound).with_system(sound_setting_menu_setup),
            )
            .add_system_set(
                SystemSet::on_update(MenuState::SettingsSound)
                    .with_system(setting_button::<Volume>),
            )
            .add_system_set(
                SystemSet::on_exit(MenuState::SettingsSound)
                    .with_system(despawn_screen::<OnSoundSettingsMenuScreen>),
            )
            .add_system_set(
                SystemSet::on_update(GameState::Menu)
                    .with_system(menu_action)
                    .with_system(button_system),
            );
    }
}
