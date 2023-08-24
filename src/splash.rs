use bevy::prelude::*;

use crate::{despawn_screen, GameState};

pub struct SplashPlugin;

impl Plugin for SplashPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(GameState::Splash), splash_setup)
            .add_systems(Update, countdown.run_if(in_state(GameState::Splash)))
            .add_systems(OnExit(GameState::Splash), despawn_screen::<OnSplashScreen>);
    }
}

#[derive(Resource, Deref, DerefMut)]
struct SplashTimer(Timer);

#[derive(Component)]
struct OnSplashScreen;    

fn splash_setup(mut commands: Commands) {
    commands.insert_resource(SplashTimer(Timer::from_seconds(1.0, TimerMode::Once)));
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
