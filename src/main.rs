mod systems;
mod game;
mod splash;

use bevy::prelude::*;
use rand::Rng;

use splash::*;
use game::*;
//use systems::*;

#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, Hash, States)]
pub enum GameState {
    #[default]
    Splash,
    MainMenu,
    InGame,
}

#[derive(Component)]
struct Dir {
    x: f32,
    y: f32
}

#[derive(Component)]
struct Bob;

#[derive(Component)]
struct Playa;

#[derive(Component)]
struct MenuClose;

#[derive(Component)]
struct LevelUnload;

#[derive(Component, Deref, DerefMut)]
struct PrinterTick(Timer);

#[derive(Component)]
struct TextToPrint(String);

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.9, 0.9, 0.9)))
        .add_state::<GameState>()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_plugins((splash::SplashPlugin, game::GamePlugin))
        .add_systems(Update, (
            sys_print_text,
            sys_move,
            sys_bobbin,
            sys_keys,
        ))
        .run();
}

fn sys_keys(key_in: Res<Input<KeyCode>>,
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

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut rng = rand::thread_rng();
    commands.spawn(Camera2dBundle::default());

    for _i in 0..0 {
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

    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("monsta.png"),
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
        Playa));

    commands.spawn((
        PrinterTick(Timer::from_seconds(1.0, TimerMode::Repeating)),
        TextToPrint("Main Menu".to_string()),
        MenuClose,
    ));

    commands.spawn((
        PrinterTick(Timer::from_seconds(1.0, TimerMode::Repeating)),
        TextToPrint("Level 1".to_string()),
        LevelUnload,
    ));
    

}


fn sys_print_text(time: Res<Time>, mut query: Query<(&mut PrinterTick, &TextToPrint)>) {
    for (mut timer, text) in &mut query {
        if timer.tick(time.delta()).just_finished() {
            info!("{}", text.0);
        }
    }
}

fn sys_move(mut commands: Commands, time: Res<Time>, mut pos: Query<(Entity, &mut Transform, &mut Dir, &mut Sprite)>) {
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

fn sys_bobbin(time: Res<Time>, mut pos: Query<(&mut Transform, With<Bob>)>) {
    for (mut transform, _bob) in &mut pos {
        transform.translation.y +=  (time.elapsed_seconds() * 30.0).sin() * 3.0;
    }
}

// Generic system that takes a component as a parameter, and will despawn all entities with that component
fn despawn_screen<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in &to_despawn {
        commands.entity(entity).despawn_recursive();
    }
}
