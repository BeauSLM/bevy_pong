use bevy::prelude::*;

const FOREGROUND: Color = Color::rgb(0.7, 0.7, 0.7);
const BACKGROUND: Color = Color::rgb(0.04, 0.04, 0.04);
const MAX_PADDLE_OFFSET: f32 = 25.;

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
    let y = &mut query.single_mut().translation.y;
    if keys.pressed(KeyCode::W) {
        *y += 1.;
    }

    if keys.pressed(KeyCode::S) {
        *y -= 1.;
    }

    *y = y.min(MAX_PADDLE_OFFSET).max(-MAX_PADDLE_OFFSET);
}

fn right_paddle_move(keys: Res<Input<KeyCode>>, mut query: Query<&mut Transform, With<RightPaddle>>) {
    let y = &mut query.single_mut().translation.y;
    if keys.pressed(KeyCode::Up) {
        *y += 1.;
    }

    if keys.pressed(KeyCode::Down) {
        *y -= 1.;
    }

    *y = y.min(MAX_PADDLE_OFFSET).max(-MAX_PADDLE_OFFSET);
}
