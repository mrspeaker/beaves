use bevy::prelude::*;
use rand::Rng;

use crate::{ despawn_screen, GameState, Dir, Bob, sys_move };

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(GameState::InGame), game_setup)
            .add_systems(Update, (sys_move).run_if(in_state(GameState::InGame)))
            .add_systems(OnExit(GameState::InGame), despawn_screen::<OnGameScreen>);
    }
}

#[derive(Component)]
struct OnGameScreen;

#[derive(Resource, Deref, DerefMut)]
struct GameTimer(Timer);

fn game_setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {
    let mut rng = rand::thread_rng();

    for _i in 0..10 {
        commands.spawn((
            SpriteBundle {
                texture: asset_server.load("char.png"),
                transform: Transform::from_xyz(
                    (rng.gen::<f32>()) * 1000. - 500.,
                    (rng.gen::<f32>()) * 500. - 250.,
                    0.)
                    .with_scale(Vec3::splat(0.25)),
                sprite: Sprite {
                    flip_y: false,
                    ..default()
                },
                ..default()
            },
            Dir {
                x: rng.gen::<f32>() * 400.0 - 200.0,
                y: rng.gen::<f32>() * 400.0 - 200.0
            }
        ));
    }
}
