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
                check_for_pickup_collisions,
                check_for_wall_collisions
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
struct Wall;

#[derive(Component)]
struct IsDead;

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
            sprite: Sprite {
                custom_size: Some(Vec2::new(50.0, 50.0)),
                ..default()
            },
            ..default()
        },
        Playa,
        OnGameScreen));

    for i in 0..20 {
        let ent = commands.spawn((
            SpriteBundle {
                texture: asset_server.load("char.png"),
                transform: Transform::from_xyz(
                    (rng.gen::<f32>()) * 1000. - 500.,
                    (rng.gen::<f32>()) * 500. - 250.,
                    0.),
                sprite: Sprite {
                    custom_size: Some(Vec2::new(50.0, 50.0)),
                    ..default()
                },                
                ..default()
            },
            Peep,
            OnGameScreen
        )).id();

        if i > 7 {
            commands.entity(ent).insert(Dir {
                x: rng.gen::<f32>() * 400.0 - 200.0,
                y: rng.gen::<f32>() * 400.0 - 200.0
            });
        }
            
    }

    for _i in 0..100 {
        commands.spawn((
            SpriteBundle {            
                texture: asset_server.load("wall.png"),
                transform: Transform::from_xyz(
                    (rng.gen::<f32>() * 11.).floor() * 100. - 500.,
                    (rng.gen::<f32>() * 10.).floor() * 150. - 500.,
                    0.),
                sprite: Sprite {
                    custom_size: Some(Vec2::new(100.0, 50.0)),
                    ..default()
                },                
                ..default()
            },
            Wall,
            OnGameScreen
        ));
    }
}

fn check_if_done(
    key_in: Res<Input<KeyCode>>,
    mut game_state: ResMut<NextState<GameState>>,
    playa_query: Query<&IsDead, With<Playa>>,
    peeps_query: Query<Entity, With<Peep>>
) {
    if peeps_query.is_empty() || !playa_query.is_empty() ||  key_in.pressed(KeyCode::Space) {
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

fn check_for_pickup_collisions(
    mut commands: Commands,
    mut playa_query: Query<(&Transform, &Sprite), With<Playa>>,
    peeps_query: Query<(Entity, &Transform, &Sprite), With<Peep>>
) {
    let (playa, plays) = playa_query.single_mut();
    let playa_size = plays.custom_size.unwrap_or(Vec2::ONE);

    for (entity, transform, sprite) in &peeps_query {
        let collision = collide(
            playa.translation,
            playa_size,
            transform.translation,
            sprite.custom_size.unwrap_or(Vec2::ONE)
        );
        if let Some(_collision) = collision {
            commands.entity(entity).despawn();
        }
    }
}

fn check_for_wall_collisions(
    mut commands: Commands,
    mut playa_query: Query<(Entity, &Transform, &Sprite), With<Playa>>,
    wall_query: Query<(&Transform, &Sprite), With<Wall>>
) {
    let (entity, playa, plays) = playa_query.single_mut();
    let playa_size = plays.custom_size.unwrap_or(Vec2::ONE);

    for (transform, sprite) in &wall_query {
        let collision = collide(
            playa.translation,
            playa_size,
            transform.translation,
            sprite.custom_size.unwrap_or(Vec2::ONE)
        );
        if let Some(_collision) = collision {
            commands.entity(entity).insert(IsDead);
        }
    }
}
