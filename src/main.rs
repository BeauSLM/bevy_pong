use bevy::prelude::*;

const FOREGROUND: Color = Color::rgb(0.7, 0.7, 0.7);
const BACKGROUND: Color = Color::rgb(0.04, 0.04, 0.04);
const MAX_PADDLE_OFFSET: f32 = 25.;

#[derive(Component)]
struct LeftPaddle;

#[derive(Component)]
struct RightPaddle;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(BACKGROUND))
        .add_startup_system(setup)
        .add_system(left_paddle_move)
        .add_system(right_paddle_move)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn_bundle(SpriteBundle {
        transform: Transform {
            translation: Vec3::new(-50., 0., 0.),
            ..Default::default()
        },
        sprite: Sprite {
            color: FOREGROUND,
            custom_size: Some(Vec2::new(2., 20.)),
            ..Default::default()
        },
        ..Default::default()
    })
    .insert(LeftPaddle);
    commands.spawn_bundle(SpriteBundle {
        transform: Transform {
            translation: Vec3::new(50., 0., 0.),
            ..Default::default()
        },
        sprite: Sprite {
            color: FOREGROUND,
            custom_size: Some(Vec2::new(2., 20.)),
            ..Default::default()
        },
        ..Default::default()
    })
    .insert(RightPaddle);
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
