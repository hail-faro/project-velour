use bevy::{asset, prelude::*};
// use bevy::ecs::entity;
use bevy::prelude::shape as bevy_shape;
use bevy::prelude::*;
use bevy_rapier3d::prelude::{NoUserData, RapierPhysicsPlugin};
// use bevy_rapier3d::prelude::*;

use super::{
    camera, despawn_screen, menu::DisplayQuality, menu::Volume, player, ship, GameState,
    MENU_TEXT_COLOR,
};

pub struct GamePlugin;

#[derive(Component)]
struct OnGameScreen;

#[derive(Deref, DerefMut, Resource)]
struct GameTimer(Timer);

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        // app.add_system_set(SystemSet::on_enter(GameState::Game).with_system(game_setup))
        //     .add_system_set(SystemSet::on_update(GameState::Game).with_system(game))
        //     .add_system_set(
        //         SystemSet::on_exit(GameState::Game).with_system(despawn_screen::<OnGameScreen>),
        //     );

        app.insert_resource(Msaa { samples: 1 })
            .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
            // .add_plugin(RapierDebugRenderPlugin::default())
            // .add_system_set(SystemSet::on_enter(GameState::Game).with_system(camera::setup_camera))
            .add_system_set(SystemSet::on_enter(GameState::Game).with_system(player::spawn_player))
            .add_system_set(
                SystemSet::on_update(GameState::Game).with_system(player::player_control_system),
            )
            .add_system_set(
                SystemSet::on_update(GameState::Game).with_system(ship::move_ship_system),
            )
            .add_system_set(
                SystemSet::on_update(GameState::Game).with_system(camera::move_camera_system),
            )
            .add_system_set(SystemSet::on_enter(GameState::Game).with_system(test_setup));
        // .add_startup_system(camera::setup_camera)
        // .add_system(player::spawn_player)
        // .add_system(player::player_control_system)
        // .add_system(ship::move_ship_system)
        // .add_system(camera::move_camera_system)
        // .add_system(test_setup);
        // .run();
    }
}

fn test_setup(
    mut commands: Commands,
    // asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(bevy_shape::Plane { size: 50.0 })),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        transform: Transform::from_xyz(0.0, -10.0, 500.0),
        ..Default::default()
    });
    commands.spawn(PointLightBundle {
        transform: Transform::from_xyz(4.0, 5.0, -4.0),
        ..Default::default()
    });
}

// fn game_setup(
//     mut commands: Commands,
//     asset_server: Res<AssetServer>,
//     display_quality: Res<DisplayQuality>,
//     volume: Res<Volume>,
// ) {
//     let font = asset_server.load("fonts/FiraMono-Medium.ttf");

//     commands
//         .spawn((
//             NodeBundle {
//                 style: Style {
//                     margin: UiRect {
//                         left: Val::Px(10.0),
//                         top: Val::Px(10.0),
//                         ..Default::default()
//                     },
//                     flex_direction: FlexDirection::ColumnReverse,
//                     align_items: AlignItems::Center,
//                     ..Default::default()
//                 },
//                 // color: Color::BLACK.into(),
//                 ..Default::default()
//             },
//             OnGameScreen,
//         ))
//         .with_children(|parent| {
//             parent.spawn(
//                 TextBundle::from_section(
//                     "This is a test",
//                     TextStyle {
//                         font: font.clone(),
//                         font_size: 20.0,
//                         color: MENU_TEXT_COLOR,
//                     },
//                 )
//                 .with_style(Style {
//                     margin: UiRect::all(Val::Px(50.0)),
//                     ..Default::default()
//                 }),
//             );
//             parent.spawn(
//                 TextBundle::from_sections([
//                     TextSection::new(
//                         format!("quality: {:?}", *display_quality),
//                         TextStyle {
//                             font: font.clone(),
//                             font_size: 60.0,
//                             color: MENU_TEXT_COLOR,
//                         },
//                     ),
//                     TextSection::new(
//                         "-",
//                         TextStyle {
//                             font: font.clone(),
//                             font_size: 60.0,
//                             color: MENU_TEXT_COLOR,
//                         },
//                     ),
//                     TextSection::new(
//                         format!("Volume: {:?}", *volume),
//                         TextStyle {
//                             font: font.clone(),
//                             font_size: 60.0,
//                             color: MENU_TEXT_COLOR,
//                         },
//                     ),
//                 ])
//                 .with_style(Style {
//                     margin: UiRect::all(Val::Px(50.0)),
//                     ..Default::default()
//                 }),
//             );
//         });
//     commands.insert_resource(GameTimer(Timer::from_seconds(5.0, TimerMode::Once)))
// }

// fn game(time: Res<Time>, mut game_state: ResMut<State<GameState>>, mut timer: ResMut<GameTimer>) {
//     // TODO
//     // Start Game HERE
//     if timer.tick(time.delta()).finished() {
//         game_state.set(GameState::Menu).unwrap()
//     }
// }
