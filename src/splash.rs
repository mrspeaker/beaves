use bevy::prelude::*;

use crate::{despawn_screen, GameState};

pub struct SplashPlugin;

impl Plugin for SplashPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(GameState::Splash), splash_setup)
            .add_systems(Update, (countdown).run_if(in_state(GameState::Splash)))
            .add_systems(OnExit(GameState::Splash), despawn_screen::<OnSplashScreen>);
    }
}

#[derive(Component)]
struct OnSplashScreen;

#[derive(Resource, Deref, DerefMut)]
struct SplashTimer(Timer);

fn splash_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(SplashTimer(Timer::from_seconds(2.0, TimerMode::Once)));
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("monsta.png"),
            transform: Transform::from_xyz(0., 0., 0.)
                .with_scale(Vec3::splat(0.5)),
            sprite: Sprite {
                flip_y: false,
                ..default()
            },
            ..default()
        },
        OnSplashScreen));
}

fn countdown(
    mut game_state: ResMut<NextState<GameState>>,
    time: Res<Time>,
    mut timer: ResMut<SplashTimer>,
) {
    if timer.tick(time.delta()).finished() {
        game_state.set(GameState::InGame);
    }
}
