// use bevy::ecs::entity;
use bevy::prelude::shape as bevy_shape;
use bevy::prelude::*;
// use bevy_rapier3d::prelude::*;

mod camera;
mod game;
mod menu;
// mod parts;
mod player;
mod ship;
mod splash_page;

// pub mod parts;

// Standards
pub const MENU_BACKGROUND_COLOR: BackgroundColor = bevy::prelude::BackgroundColor(Color::CRIMSON);
pub const MENU_TEXT_COLOR: Color = Color::LIME_GREEN;
pub const NORMAL_BUTTON_COLOR: Color = Color::ANTIQUE_WHITE;
pub const HOVERED_BUTTON_COLOR: Color = Color::CYAN;
pub const HOVERED_PRESSED_BUTTON_COLOR: Color = Color::AZURE;
pub const PRESSED_BUTTON_COLOR: Color = Color::CRIMSON;
pub const MENU_FONT: &str = "fonts/FiraMono-Medium.ttf";

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub enum GameState {
    Splash,
    Menu,
    Game,
    Paused,
}

// #[derive(Debug, Component, PartialEq, Eq, Clone, Copy, Resource)]
// enum Setting {
//     DisplayQuality,
//     Volume,
// }

// impl Setting {
//     fn new(setting: T) -> Setting {}
// }

fn despawn_screen<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in &to_despawn {
        commands.entity(entity).despawn_recursive();
    }
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn main() {
    // RETIRED IN PLACE
    // let mut app = App::new();

    // app.insert_resource(Msaa { samples: 1 })
    //     .add_plugins(DefaultPlugins)
    //     .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
    //     // .add_plugin(RapierDebugRenderPlugin::default())
    //     .add_startup_system(camera::setup_camera)
    //     .add_startup_system(player::spawn_player)
    //     .add_system(player::player_control_system)
    //     .add_system(ship::move_ship_system)
    //     .add_system(camera::move_camera_system)
    //     .add_system(test_setup)
    //     .run();

    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(menu::DisplayQuality::Medium)
        .insert_resource(menu::Volume(7))
        .add_startup_system(camera::setup_camera)
        .add_state(GameState::Splash)
        .add_plugin(splash_page::SplashPlugin)
        .add_plugin(menu::MainMenuPlugin)
        .add_plugin(game::GamePlugin)
        .run();
}

//ship
