use bevy::{
    prelude::*,
    sprite::collide_aabb::{collide},
    window::PrimaryWindow
};
use rand::Rng;
use crate::{ despawn_screen, GameState };

pub const PLAYA_SPEED:f32 = 250.0;
pub const NUM_CHARS:usize = 50;

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
                check_for_wall_collisions,
                confine_to_window
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
    window_query: Query<&Window, With<PrimaryWindow>>,    
    asset_server: Res<AssetServer>       
) {
    let mut rng = rand::thread_rng();

    let window: &Window = window_query.get_single().unwrap();
    
    // Make the player
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("monsta.png"),
            transform: Transform::from_xyz(
                window.width() / 2.0,
                window.height() / 2.0 + 50.,
                1.0
            ),
            sprite: Sprite {
                custom_size: Some(Vec2::new(50.0, 50.0)),
                ..default()
            },
            ..default()
        },
        Playa,
        OnGameScreen));

    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("bg.png"),
            transform: Transform::from_xyz(
                window.width() / 2.0,
                window.height() / 2.0 + 50.,
                0.0
            ).with_scale(Vec3::new(1.8, 1.62, 0.0)),
            ..default()
        },
        OnGameScreen
    ));


    // Make the baddies
    for i in 0..NUM_CHARS {
        let ent = commands.spawn((
            SpriteBundle {
                texture: asset_server.load("char.png"),
                transform: Transform::from_xyz(
                    (rng.gen::<f32>()) * window.width(),
                    (rng.gen::<f32>()) * window.height(),
                    1.),
                sprite: Sprite {
                    custom_size: Some(Vec2::new(50.0, 50.0)),
                    ..default()
                },                
                ..default()
            },
            Peep,
            OnGameScreen
        )).id();

        // Make some move around (with Dir component)
        if i > 7 {
            let speed = rng.gen_range(50.0..150.0);
            let dir = Vec2 {
                x: rng.gen_range(-1.0..1.0),
                y: rng.gen_range(-1.0..1.0)
            }.normalize() * speed;
            commands.entity(ent).insert(Dir {
                x: dir.x,
                y: dir.y
            });
        }
            
    }

    let xo = (window.width() / 100.0).trunc() + 1.;
    let yo = (window.height() / 150.0).trunc() + 1.;
    for _i in 0..100 {
        commands.spawn((
            SpriteBundle {            
                texture: asset_server.load("wall.png"),
                transform: Transform::from_xyz(
                    (rng.gen::<f32>() * xo).floor() * 100.0 + 50.0,
                    (rng.gen::<f32>() * yo).floor() * 150.0 + 25.0,
                    0.1),
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

fn move_bounce(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    time: Res<Time>,
    mut pos: Query<(Entity, &mut Transform, &mut Dir, &mut Sprite)>
) {
    let window: &Window = window_query.get_single().unwrap();
    
    for (ent, mut transform, mut dir, mut spr) in &mut pos {
        transform.translation.x += dir.x * time.delta_seconds();
        transform.translation.y += dir.y * time.delta_seconds();

        if transform.translation.x < 0.0 || transform.translation.x > window.width() {
            transform.translation.x = if transform.translation.x < 0.0 { 0.0 } else { window.width() };
            dir.x = dir.x * -1.;
            spr.flip_x = !spr.flip_x;
            commands.entity(ent).insert(Bob);
        }
        if transform.translation.y < 0.0 || transform.translation.y > window.height() {
            dir.y = dir.y * -1.0;
            spr.flip_y = !spr.flip_y;
        }

    }
}

fn move_bob(time: Res<Time>, mut pos: Query<(&mut Transform, With<Bob>)>) {
    for (mut transform, _bob) in &mut pos {
        transform.translation.y += (time.elapsed_seconds() * 20.0).sin() * 1.5;
    }
}

fn move_with_keys(
    key_in: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut query: Query<&mut Transform, With<Playa>>
) {
    let mut dir = Vec3::ZERO;
    let mut transform = query.single_mut();

    if key_in.pressed(KeyCode::Right) {
        dir.x += 1.0;
    }
    if key_in.pressed(KeyCode::Left) {
        dir.x -= 1.0;
    }
    if key_in.pressed(KeyCode::Up) {
        dir.y += 1.0;
    }
    if key_in.pressed(KeyCode::Down) {
        dir.y -= 1.0;
    }
    if dir.length() > 0.0 {
        dir = dir.normalize();
        transform.translation += dir * PLAYA_SPEED * time.delta_seconds();
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

fn confine_to_window(
    mut playa_query: Query<(&Sprite, &mut Transform), With<Playa>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
){
    let (sprite, mut transform) = playa_query.single_mut() ;
    let window: &Window = window_query.get_single().unwrap();
    let hw = sprite.custom_size.unwrap_or(Vec2::ONE).x / 2.0;
    let hh = sprite.custom_size.unwrap_or(Vec2::ONE).y / 2.0;
    let x1 = hw;
    let x2 = window.width() - hw;
    let y1 = hh;
    let y2 = window.height() - hh;
    let mut t: Vec3 = transform.translation;
    if t.x < x1 { t.x = x1 }
    if t.x > x2 { t.x = x2 }
    if t.y < y1 { t.y = y1 }
    if t.y > y2 { t.y = y2 }
    transform.translation = t;
}
