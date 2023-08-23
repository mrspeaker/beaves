use bevy::prelude::*;
use rand::Rng;

#[derive(Component)]
struct Dir {
    x: f32,
    y: f32
}

#[derive(Component)]
struct Bob;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.9, 0.9, 0.9)))
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, (move_sprites, bobbin))
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut rng = rand::thread_rng();
    commands.spawn(Camera2dBundle::default());
    for _i in 0..1000 {
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

fn move_sprites(mut commands: Commands, time: Res<Time>, mut pos: Query<(Entity, &mut Transform, &mut Dir, &mut Sprite)>) {
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

fn bobbin(time: Res<Time>, mut pos: Query<(&mut Transform, &Bob)>) {
    for (mut transform, _bob) in &mut pos {
        transform.translation.y +=  (time.elapsed_seconds() * 30.0).sin() * 3.0;
    }
}
