use bevy::{
    prelude::*,
    sprite::collide_aabb::{collide},
};

use rand::Rng;

use crate::{ despawn_screen, GameState };

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(GameState::InGame), game_setup)
            .add_systems(Update, (
                check_if_done,
                move_bounce,
                move_bob,
                move_with_keys,
                check_for_collisions
            ).run_if(in_state(GameState::InGame)))
            .add_systems(OnExit(GameState::InGame), despawn_screen::<OnGameScreen>);
    }
}

#[derive(Component)]
struct OnGameScreen;

#[derive(Component)]
struct Playa;

#[derive(Component)]
struct Peep;

#[derive(Component)]
struct Dir {
    pub x: f32,
    pub y: f32
}

#[derive(Component)]
struct Bob;

#[derive(Resource, Deref, DerefMut)]
struct GameTimer(Timer);

fn game_setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {
    let mut rng = rand::thread_rng();

    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("monsta.png"),
            transform: Transform::from_xyz(0.,0., 0.)
                .with_scale(Vec3::splat(0.25)),
            ..default()
        },
        Playa,
        OnGameScreen));
    
    for _i in 0..10 {
        commands.spawn((
            SpriteBundle {
                texture: asset_server.load("char.png"),
                transform: Transform::from_xyz(
                    (rng.gen::<f32>()) * 1000. - 500.,
                    (rng.gen::<f32>()) * 500. - 250.,
                    0.)
                    .with_scale(Vec3::splat(0.25)),
                ..default()
            },
            Dir {
                x: rng.gen::<f32>() * 400.0 - 200.0,
                y: rng.gen::<f32>() * 400.0 - 200.0
            },
            Peep,
            OnGameScreen
        ));
    }
}

fn check_if_done(
    key_in: Res<Input<KeyCode>>,
    mut game_state: ResMut<NextState<GameState>>
) {
    if key_in.pressed(KeyCode::Space) {
        game_state.set(GameState::Splash);
    }
}
    

fn move_bounce(mut commands: Commands, time: Res<Time>, mut pos: Query<(Entity, &mut Transform, &mut Dir, &mut Sprite)>) {
    for (ent, mut transform, mut dir, mut spr) in &mut pos {
        transform.translation.x += dir.x * time.delta_seconds();
        transform.translation.y += dir.y * time.delta_seconds();
        
        if transform.translation.x > 650. || transform.translation.x < -650. {            
            dir.x = dir.x * -1.;
            spr.flip_x = !spr.flip_x;
            commands.entity(ent).insert(Bob);
        }
        if transform.translation.y < -350. || transform.translation.y > 350. {
            dir.y = dir.y * -1.;
            spr.flip_y = !spr.flip_y;
        }

    }
}

fn move_bob(time: Res<Time>, mut pos: Query<(&mut Transform, With<Bob>)>) {
    for (mut transform, _bob) in &mut pos {
        transform.translation.y += (time.elapsed_seconds() * 30.0).sin() * 3.0;
    }
}

fn move_with_keys(key_in: Res<Input<KeyCode>>,
            time: Res<Time>,
            mut query: Query<&mut Transform, With<Playa>>) {
    let mut transform = query.single_mut();
    let speed = 300.0;
    if key_in.pressed(KeyCode::Right) {
        transform.translation.x += speed * time.delta_seconds();
    }
    if key_in.pressed(KeyCode::Left) {
        transform.translation.x -= speed * time.delta_seconds();
    }
    if key_in.pressed(KeyCode::Up) {
        transform.translation.y += speed * time.delta_seconds();
    }
    if key_in.pressed(KeyCode::Down) {
        transform.translation.y -= speed * time.delta_seconds();
    }
}

fn check_for_collisions(
    mut commands: Commands,
    mut playa_query: Query<&Transform, With<Playa>>,
    peeps_query: Query<(Entity, &Transform), With<Peep>>
) {
    let playa = playa_query.single_mut();
    let playa_size = playa.scale.truncate();

    for (entity, transform) in &peeps_query {
        let collision = collide(
            playa.translation,
            playa_size,
            transform.translation,
            transform.scale.truncate(),
        );
        if collision.is_some() {
            info!("{}", transform.translation.x);
        }
        if let Some(_collision) = collision {
            info!("Y");
            commands.entity(entity).despawn();
        }
    }
}
