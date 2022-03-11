use bevy::prelude::*;

const MAX_PADDLE_X: f32 = 50.;
const MAX_PADDLE_Y: f32 = 25.; // maximum paddle offset
const PADDLE_HEIGHT: f32 = 20.;

#[derive(Component)]
struct LeftPaddle;

#[derive(Component)]
struct RightPaddle;

#[derive(Component)]
struct Ball {
    velocity: Vec3,
}

fn main() {
    const BACKGROUND: Color = Color::rgb(0.04, 0.04, 0.04);
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(BACKGROUND))
        .add_startup_system(setup)
        .add_system(left_paddle_movement)
        .add_system(right_paddle_movement)
        .add_system(ball_movement)
        .add_system(ball_collision)
        .run();
}

fn setup(mut commands: Commands) {
    const FOREGROUND: Color = Color::rgb(0.7, 0.7, 0.7);
    commands.spawn_bundle(SpriteBundle {
        transform: Transform {
            translation: Vec3::new(-MAX_PADDLE_X, 0., 0.),
            ..Default::default()
        },
        sprite: Sprite {
            color: FOREGROUND,
            custom_size: Some(Vec2::new(2., PADDLE_HEIGHT)),
            ..Default::default()
        },
        ..Default::default()
    })
    .insert(LeftPaddle);
    commands.spawn_bundle(SpriteBundle {
        transform: Transform {
            translation: Vec3::new(MAX_PADDLE_X, 0., 0.),
            ..Default::default()
        },
        sprite: Sprite {
            color: FOREGROUND,
            custom_size: Some(Vec2::new(2., PADDLE_HEIGHT)),
            ..Default::default()
        },
        ..Default::default()
    })
    .insert(RightPaddle);
    commands.spawn_bundle(SpriteBundle {
        sprite: Sprite {
            color: FOREGROUND,
            custom_size: Some(Vec2::new(5., 5.)),
            ..Default::default()
        },
        ..Default::default()
    })
    .insert(Ball {
        velocity: Vec3::new(-0.5, 0.5, 0.).normalize()
    });
    let mut camera = OrthographicCameraBundle::new_2d();
    camera.orthographic_projection.scale = 1. / 10.;
    commands.spawn_bundle(camera);
}

fn left_paddle_movement(
    keys: Res<Input<KeyCode>>,
    mut query: Query<&mut Transform, With<LeftPaddle>>
    ) {
    // move the left paddle
    let y = &mut query.single_mut().translation.y;
    if keys.pressed(KeyCode::W) {
        *y += 1.;
    }

    if keys.pressed(KeyCode::S) {
        *y -= 1.;
    }

    *y = y.min(MAX_PADDLE_Y).max(-MAX_PADDLE_Y);

}

fn right_paddle_movement(
    keys: Res<Input<KeyCode>>,
    mut query: Query<&mut Transform, With<RightPaddle>>
    ) {
    // move the right paddle
    let y = &mut query.single_mut().translation.y;
    if keys.pressed(KeyCode::Up) {
        *y += 1.;
    }

    if keys.pressed(KeyCode::Down) {
        *y -= 1.;
    }

    *y = y.min(MAX_PADDLE_Y).max(-MAX_PADDLE_Y);
}

fn ball_movement(mut query: Query<(&Ball, &mut Transform)>) {
    let (ball, mut trans) = query.single_mut();
    trans.translation += ball.velocity;
}

