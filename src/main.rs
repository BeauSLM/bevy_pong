use bevy::prelude::*;
use bevy::sprite::collide_aabb::{collide, Collision};

const MAX_PADDLE_X: f32 = 500.;
const MAX_PADDLE_Y: f32 = 300.; // maximum paddle offset
const PADDLE_HEIGHT: f32 = 100.;
const FOREGROUND: Color = Color::rgb(0.7, 0.7, 0.7);

#[derive(Component)]
enum Paddle {
    Left,
    Right,
}

#[derive(Component)]
struct Ball {
    velocity: Vec3,
}

#[derive(Component)]
struct LeftScore;

#[derive(Component)]
struct RightScore;

struct Scoreboard {
    left: usize,
    right: usize,
}

struct BallRespawnTimer {
    timer: Timer,
}

impl Default for BallRespawnTimer {
    fn default() -> Self {
        BallRespawnTimer {
            timer: Timer::from_seconds(0.5, false),
        }
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]
struct ScoreUpdateLabel;

fn main() {
    const BACKGROUND: Color = Color::rgb(0.04, 0.04, 0.04);
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(BACKGROUND))
        .add_startup_system(setup)
        .add_system(paddle_movement)
        .add_system(ball_movement)
        .add_system(ball_collision)
        .insert_resource(Scoreboard { left: 0, right: 0 })
        .add_system_to_stage(
            CoreStage::PreUpdate, 
            ball_respawn)
        .add_system_to_stage(
            CoreStage::PostUpdate,
            score_system
            .label(ScoreUpdateLabel))
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    let text_style = TextStyle {
        font,
        font_size: 90.0,
        color: Color::WHITE,
    };
    let text_alignment = TextAlignment {
        vertical: VerticalAlign::Center,
        horizontal: HorizontalAlign::Center,
    };
    commands
        .spawn_bundle(Text2dBundle {
            text: Text::with_section("0", text_style.clone(), text_alignment),
            transform: Transform {
                translation: Vec3::new(-50., 300., 0.),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(LeftScore);
    commands
        .spawn_bundle(Text2dBundle {
            text: Text::with_section("0", text_style, text_alignment),
            transform: Transform {
                translation: Vec3::new(50., 300., 0.),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(RightScore);
    commands
        .spawn_bundle(SpriteBundle {
            transform: Transform {
                translation: Vec3::new(-MAX_PADDLE_X, 0., 0.),
                scale: Vec3::new(15., PADDLE_HEIGHT, 0.),
                ..Default::default()
            },
            sprite: Sprite {
                color: FOREGROUND,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Paddle::Left);
    commands
        .spawn_bundle(SpriteBundle {
            transform: Transform {
                translation: Vec3::new(MAX_PADDLE_X, 0., 0.),
                scale: Vec3::new(15., PADDLE_HEIGHT, 0.),
                ..Default::default()
            },
            sprite: Sprite {
                color: FOREGROUND,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Paddle::Right);
    commands
        .spawn_bundle(SpriteBundle {
            transform: Transform {
                scale: Vec3::new(25., 25., 0.),
                ..Default::default()
            },
            sprite: Sprite {
                color: FOREGROUND,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Ball {
            velocity: Vec3::new(-0.5, 0.5, 0.).normalize(),
        });
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

// TODO: this is ugly, kill it
fn paddle_movement(keys: Res<Input<KeyCode>>, mut query: Query<(&mut Transform, &Paddle)>) {
    const PADDLE_SPEED: f32 = 5.;
    for (mut trans, paddle) in query.iter_mut() {
        let y = &mut trans.translation.y;
        match *paddle {
            Paddle::Left => {
                if keys.pressed(KeyCode::W) {
                    *y += PADDLE_SPEED;
                }

                if keys.pressed(KeyCode::S) {
                    *y -= PADDLE_SPEED;
                }
            }
            Paddle::Right => {
                if keys.pressed(KeyCode::Up) {
                    *y += PADDLE_SPEED;
                }

                if keys.pressed(KeyCode::Down) {
                    *y -= PADDLE_SPEED;
                }
            }
        }

        *y = y.min(MAX_PADDLE_Y).max(-MAX_PADDLE_Y);
    }
}

fn ball_movement(mut query: Query<(&Ball, &mut Transform)>) {
    const BALL_SPEED: f32 = 7.5;
    if let Ok((ball, mut trans)) = query.get_single_mut() {
        trans.translation += ball.velocity * BALL_SPEED;
    }
}

fn ball_collision(
    mut ball_query: Query<(&mut Ball, &Transform)>,
    paddle_query: Query<&Transform, With<Paddle>>,
) {
    if let Ok((mut ball, ball_trans)) = ball_query.get_single_mut() {
        let velocity = &mut ball.velocity;
        const MAX_BALL_Y: f32 = PADDLE_HEIGHT / 2. + MAX_PADDLE_Y;
        if ball_trans.translation.y > MAX_BALL_Y || ball_trans.translation.y < -MAX_BALL_Y {
            velocity.y *= -1.;
        }
        for paddle_trans in paddle_query.iter() {
            let collision = collide(
                ball_trans.translation,
                ball_trans.scale.truncate(),
                paddle_trans.translation,
                paddle_trans.scale.truncate(),
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
}

// TODO: find a decent name here
fn score_system(
    ball_query: Query<(Entity, &Transform), With<Ball>>,
    mut score: ResMut<Scoreboard>,
    mut left_score: Query<&mut Text, (With<LeftScore>, Without<RightScore>)>,
    mut right_score: Query<&mut Text, With<RightScore>>,
    mut commands: Commands,
) {
    const MAX_BALL_X: f32 = MAX_PADDLE_X + 10.;
    if let Ok((ball_entity, ball_trans)) = ball_query.get_single() {
        let mut scored = true;
        // TODO: find some slick way to write this out
        match ball_trans.translation.x {
            x if x < -MAX_BALL_X => {
                // score for right-side player
                score.right += 1;
                let mut text = right_score.single_mut();
                text.sections[0].value = format!("{}", score.right);
            }
            x if x > MAX_BALL_X => {
                // score for right-side player
                score.left += 1;
                let mut text = left_score.single_mut();
                text.sections[0].value = format!("{}", score.left);
            }
            _ => scored = false,
        };
        if scored {
            commands.entity(ball_entity).despawn();
            commands.insert_resource(BallRespawnTimer::default());
        }
    }
}

fn ball_respawn(
    time: Res<Time>,
    mut dir: Local<bool>,
    mut respawn: Option<ResMut<BallRespawnTimer>>,
    mut commands: Commands,
    ) {
    // if timer isn't active, or not finished, there's nothing to do
    if let Some(mut respawn) = respawn {
        if !respawn.timer.tick(time.delta()).finished() { return; }
        let spawn_dir = if *dir { 0.5 } else { -0.5 };
        *dir = !*dir;
        commands
            .spawn_bundle(SpriteBundle {
                transform: Transform {
                    scale: Vec3::new(25., 25., 0.),
                    ..Default::default()
                },
                sprite: Sprite {
                    color: FOREGROUND,
                    ..Default::default()
                },
                ..Default::default()
            })
        .insert(Ball {
            velocity: Vec3::new(spawn_dir, 0.5, 0.).normalize(),
        });
        commands.remove_resource::<BallRespawnTimer>()
    }
}
