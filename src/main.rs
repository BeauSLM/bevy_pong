use bevy::prelude::*;
use bevy::sprite::collide_aabb::{ Collision, collide };

const MAX_PADDLE_X: f32 = 50.;
const MAX_PADDLE_Y: f32 = 25.; // maximum paddle offset
const PADDLE_HEIGHT: f32 = 20.;

#[derive(Component, Clone, Copy)]
enum Paddle {
    Left,
    Right,
}

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
        .add_system(paddle_movement)
        .add_system(ball_movement)
        .add_system(ball_collision)
        .add_system_to_stage(CoreStage::PostUpdate, game_reset_system)
        .run();
}

fn setup(mut commands: Commands) {
    const FOREGROUND: Color = Color::rgb(0.7, 0.7, 0.7);
    commands.spawn_bundle(SpriteBundle {
        transform: Transform {
            translation: Vec3::new(-MAX_PADDLE_X, 0., 0.),
            scale: Vec3::new(2., PADDLE_HEIGHT, 0.),
            ..Default::default()
        },
        sprite: Sprite {
            color: FOREGROUND,
            ..Default::default()
        },
        ..Default::default()
    })
    .insert(Paddle::Left);
    commands.spawn_bundle(SpriteBundle {
        transform: Transform {
            translation: Vec3::new(MAX_PADDLE_X, 0., 0.),
            scale: Vec3::new(2., PADDLE_HEIGHT, 0.),
            ..Default::default()
        },
        sprite: Sprite {
            color: FOREGROUND,
            ..Default::default()
        },
        ..Default::default()
    })
    .insert(Paddle::Right);
    commands.spawn_bundle(SpriteBundle {
        transform: Transform {
            scale: Vec3::new(5., 5., 0.),
            ..Default::default()
        },
        sprite: Sprite {
            color: FOREGROUND,
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

fn paddle_movement(
    keys: Res<Input<KeyCode>>,
    mut query: Query<(&mut Transform, &Paddle)>
    ) {
    // move the left paddle
    for (mut trans, paddle) in query.iter_mut() {
        let y = &mut trans.translation.y;
        match *paddle {
            Paddle::Left => {
                if keys.pressed(KeyCode::W) {
                    *y += 1.;
                }

                if keys.pressed(KeyCode::S) {
                    *y -= 1.;
                }
            },
            Paddle::Right => {
                if keys.pressed(KeyCode::Up) {
                    *y += 1.;
                }

                if keys.pressed(KeyCode::Down) {
                    *y -= 1.;
                }
            }
        }

        *y = y.min(MAX_PADDLE_Y).max(-MAX_PADDLE_Y);
    }
}

fn ball_movement(mut query: Query<(&Ball, &mut Transform)>) {
    let (ball, mut trans) = query.single_mut();
    trans.translation += ball.velocity;
}

fn ball_collision(
    mut ball_query: Query<(&mut Ball, &Transform)>,
    paddle_query: Query<&Transform, With<Paddle>>,
    ) {
    let (mut ball, ball_trans) = ball_query.single_mut();
    let velocity = &mut ball.velocity;
    const MAX_BALL_Y: f32 = PADDLE_HEIGHT / 2. + MAX_PADDLE_Y;
    if ball_trans.translation.y > MAX_BALL_Y || ball_trans.translation.y < -MAX_BALL_Y {
        velocity.y *= -1.;
    }
    // DELETE ME
    // if ball_trans.translation.x > MAX_PADDLE_X || ball_trans.translation.x < -MAX_PADDLE_X {
    //     velocity.x *= -1.;
    // }

    for paddle_trans in paddle_query.iter() {
        let collision = collide(
            ball_trans.translation,
            ball_trans.scale.truncate(),
            paddle_trans.translation,
            paddle_trans.scale.truncate()
            );
        let mut reflect_x = false;
        let mut reflect_y = false;
        if let Some(collision) = collision {
            match collision {
                Collision::Left => reflect_x = velocity.x > 0.,
                Collision::Right => reflect_x = velocity.x < 0.,
                Collision::Top => reflect_y = velocity.y < 0.,
                Collision::Bottom => reflect_y = velocity.y > 0.,
            };
        }
        if reflect_x {
            velocity.x *= -1.;
        }
        if reflect_y {
            velocity.y *= -1.;
        }
    }
}

fn game_reset_system(
    // TODO: find a better setup here
    mut ball_query: Query<(&mut Ball, &mut Transform), Without<Paddle>>,
    mut paddles_query: Query<&mut Transform, With<Paddle>>
    ) {
    const MAX_BALL_X: f32 = MAX_PADDLE_X + 10.;
    let (mut ball, mut ball_trans) = ball_query.single_mut();
    let mut scored = true;
    // TODO: find some slick way to write this out
    match ball_trans.translation.x {
        x if x < -MAX_BALL_X => {
            // score for right-side player
            ball.velocity = Vec3::new(-0.5, 0.5, 0.).normalize();
        },
        x if x > MAX_BALL_X => {
            // score for right-side player
            ball.velocity = Vec3::new(0.5, -0.5, 0.).normalize();
        },
        _ => scored = false
    };

    if scored {
        ball_trans.translation = Vec3::ZERO;
        for mut paddle_trans in paddles_query.iter_mut() {
            paddle_trans.translation.y = 0.;
        }
    }
}
