mod movement_system;
mod setup;
mod types;

const INITIAL_BALL_SPEED: f32 = 300.0;

use crate::movement_system::*;
use crate::setup::*;
use crate::types::*;
use bevy::app::AppExit;
use bevy::{
    prelude::*,
    sprite::collide_aabb::{collide, Collision},
};
use rand::prelude::*;

fn main() {
    App::new()
        .insert_resource(Scoreboard {
            player_score: 0,
            opponent_score: 0,
        })
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_state::<AppState>()
        .add_systems(OnEnter(AppState::MainMenu), (menu_setup, camera_setup))
        .add_systems(Update, menu.run_if(in_state(AppState::MainMenu)))
        .add_systems(
            OnExit(AppState::MainMenu),
            (
                cleanup_menu,
                player_setup,
                wall_setup,
                divider_setup,
                opponent_setup,
                ball_setup,
                score_setup,
            ),
        )
        .add_systems(OnEnter(AppState::Paused), pause_setup)
        .add_systems(Update, pause.run_if(in_state(AppState::Paused)))
        .add_systems(OnExit(AppState::Paused), cleanup_pause)
        .add_systems(
            Update,
            (
                reset,
                check_for_collisions,
                apply_velocity,
                movement_system,
                update_score,
            )
                .run_if(in_state(AppState::InGame)),
        )
        .run();
}

fn check_for_collisions(
    mut ball_query: Query<(&mut Velocity, &Transform), With<Ball>>,
    collider_query: Query<&Transform, With<Collider>>,
) {
    let (mut ball_velocity, ball_transform) = ball_query.single_mut();
    let ball_size = ball_transform.scale.truncate();

    for transform in &collider_query {
        let collision = collide(
            ball_transform.translation,
            ball_size,
            transform.translation,
            transform.scale.truncate(),
        );
        if let Some(collision) = collision {
            let mut reflect_x = false;
            let mut reflect_y = false;

            match collision {
                Collision::Left => reflect_x = ball_velocity.x > 0.0,
                Collision::Right => reflect_x = ball_velocity.x < 0.0,
                Collision::Top => reflect_y = ball_velocity.y < 0.0,
                Collision::Bottom => reflect_y = ball_velocity.y > 0.0,
                Collision::Inside => {}
            }

            if reflect_x {
                ball_velocity.x = -ball_velocity.x;
                ball_velocity.x += 30.0 * ball_velocity.x.signum();
                ball_velocity.y += 30.0 * ball_velocity.y.signum();
            }

            if reflect_y {
                ball_velocity.y = -ball_velocity.y;
            }
        }
    }
}

fn apply_velocity(
    mut query: Query<(&mut Transform, &Velocity)>,
    time_step: Res<FixedTime>,
) {
    for (mut transform, velocity) in &mut query {
        transform.translation.x += velocity.x * time_step.period.as_secs_f32();
        transform.translation.y += velocity.y * time_step.period.as_secs_f32();
    }
}

fn reset(
    mut scoreboard: ResMut<Scoreboard>,
    mut ball_query: Query<(&mut Velocity, &mut Transform), With<Ball>>,
) {
    for (mut velocity, mut transform) in ball_query.iter_mut() {
        if transform.translation.x > 690.0 {
            let mut y_sign = 1;
            let mut x_sign = 1;
            if rand::random() {
                y_sign *= -1;
            }
            if rand::random() {
                x_sign *= -1;
            }
            let mut rng = rand::thread_rng();
            let y: f32 = rng.gen();
            let ball_dir =
                Dir(Vec2::new(1.0 * x_sign as f32, y * y_sign as f32));
            transform.translation = Vec3::new(0.0, 0.0, 0.0);
            scoreboard.player_score += 1;
            velocity.0 = *Velocity(ball_dir.normalize() * INITIAL_BALL_SPEED);
        }
        if transform.translation.x < -690.0 {
            let mut y_sign = 1;
            let mut x_sign = 1;
            if rand::random() {
                y_sign *= -1;
            }
            if rand::random() {
                x_sign *= -1;
            }
            let mut rng = rand::thread_rng();
            let y: f32 = rng.gen();
            let ball_dir =
                Dir(Vec2::new(1.0 * x_sign as f32, y * y_sign as f32));
            transform.translation = Vec3::new(0.0, 0.0, 0.0);
            scoreboard.opponent_score += 1;
            velocity.0 = *Velocity(ball_dir.normalize() * INITIAL_BALL_SPEED);
        }
    }
}

fn update_score(
    scoreboard: Res<Scoreboard>,
    mut player_query: Query<
        &mut Text,
        (With<PlayerScore>, Without<OpponentScore>),
    >,
    mut opponent_query: Query<
        &mut Text,
        (With<OpponentScore>, Without<PlayerScore>),
    >,
) {
    let mut player_text = player_query.single_mut();
    player_text.sections[0].value = scoreboard.player_score.to_string();

    let mut opponent_text = opponent_query.single_mut();
    opponent_text.sections[0].value = scoreboard.opponent_score.to_string();
}

pub fn menu(
    mut next_state: ResMut<NextState<AppState>>,
    input: Res<Input<KeyCode>>,
    mut exit: EventWriter<AppExit>,
) {
    if input.just_pressed(KeyCode::Return) {
        next_state.set(AppState::InGame);
    }
    if input.just_pressed(KeyCode::Escape) {
        exit.send(AppExit);
    }
}

fn cleanup_menu(mut commands: Commands, menu: Res<MenuData>) {
    commands.entity(menu.menu_text).despawn();
    commands.entity(menu.enter_text).despawn();
}

pub fn pause(
    mut exit: EventWriter<AppExit>,
    mut next_state: ResMut<NextState<AppState>>,
    input: Res<Input<KeyCode>>,
) {
    if input.just_pressed(KeyCode::Escape) || input.just_pressed(KeyCode::N) {
        next_state.set(AppState::InGame);
    }
    if input.just_pressed(KeyCode::Y) {
        exit.send(AppExit);
    }
}

fn cleanup_pause(mut commands: Commands, pause: Res<PauseData>) {
    commands.entity(pause.pause_text).despawn();
    commands.entity(pause.pause_shadow).despawn();
}
