use bevy::prelude::*;

const FOREGROUND: Color = Color::rgb(0.7, 0.7, 0.7);
const BACKGROUND: Color = Color::rgb(0.04, 0.04, 0.04);
const PADDLE_SPEED: f32 = 1.;
const BALL_SPEED: f32 = 1.;

#[derive(Component)]
struct LeftPaddle;

#[derive(Component)]
struct RightPaddle;

#[derive(Component)]
struct Velocity {
    x: f32,
    y: f32,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(BACKGROUND))
        .add_startup_system(setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn_bundle(SpriteBundle {
        sprite: Sprite {
            color: FOREGROUND,
            custom_size: Some(Vec2::new(2., 30.)),
            ..Default::default()
        },
        ..Default::default()
    })
    .insert(LeftPaddle);
    let mut camera = OrthographicCameraBundle::new_2d();
    camera.orthographic_projection.scale = 1. / 10.;
    commands.spawn_bundle(camera);
}

fn left_paddle_move(keys: Res<Input<KeyCode>>, mut query: Query<&mut Transform, With<LeftPaddle>>) {
    let mut trans = query.iter_mut().next().unwrap();
    if keys.pressed(KeyCode::W) {
        trans.translation.y += 1.;
    }

    if keys.pressed(KeyCode::S) {
        trans.translation.y -= 1.;
    }
}

fn right_paddle_move(keys: Res<Input<KeyCode>>, mut query: Query<&mut Transform, With<RightPaddle>>) {
    let mut trans = query.iter_mut().next().unwrap();
    if keys.pressed(KeyCode::Up) {
        trans.translation.y += 1.;
    }

    if keys.pressed(KeyCode::Down) {
        trans.translation.y -= 1.;
    }
}
