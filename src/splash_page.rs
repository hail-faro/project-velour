use std::time::Duration;

use bevy::{prelude::*, utils::tracing::Instrument};

use super::{despawn_screen, GameState, MENU_TEXT_COLOR};

#[derive(Component)]
struct OnSplashScreen;

#[derive(Resource, Deref, DerefMut)]
struct SplashTimer(Timer);

pub struct SplashPlugin;

impl Plugin for SplashPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::Splash).with_system(splash_setup))
            .add_system_set(SystemSet::on_update(GameState::Splash).with_system(countdown))
            .add_system_set(
                SystemSet::on_exit(GameState::Splash).with_system(despawn_screen::<OnSplashScreen>),
            );
    }
}

fn splash_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let icon = asset_server.load("branding/icon.png");
    commands.spawn((
        ImageBundle {
            style: Style {
                margin: UiRect::all(Val::Auto),
                size: Size::new(Val::Px(200.0), Val::Auto),
                ..default()
            },
            image: UiImage(icon),
            ..default()
        },
        OnSplashScreen,
    ));
    commands.insert_resource(SplashTimer(Timer::from_seconds(1.0, TimerMode::Once)));
}

fn countdown(
    mut game_state: ResMut<State<GameState>>,
    time: Res<Time>,
    mut timer: ResMut<SplashTimer>,
) {
    if timer.tick(time.delta()).finished() {
        game_state.set(GameState::Menu).unwrap()
    }
}
