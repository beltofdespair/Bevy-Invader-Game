use crate::components::*;
use crate::types::*;
use crate::Player;
use bevy::app::AppExit;
use bevy::prelude::*;

const MOVEMENT_SPEED: f32 = 450.0;

pub fn movement_system(
    mut next_state: ResMut<NextState<AppState>>,
    mut player: Query<&mut Transform, With<Player>>,
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
    window: Query<&Window>,
) {
    let mut transform = player.single_mut();
    let mut dir = 0;
    if transform.translation.x > -window.single().width() + 100.0
        && input.pressed(KeyCode::A)
    {
        dir = -1;
    }
    if transform.translation.x < window.single().width() - 100.0
        && input.pressed(KeyCode::D)
    {
        dir = 1;
    }
    let velocity = dir as f32 * MOVEMENT_SPEED * time.delta_seconds();
    transform.translation.x += velocity;
    // }

    if input.just_pressed(KeyCode::Escape) {
        next_state.set(AppState::Paused);
    }
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

pub fn player_projectile_system(
    mut commands: Commands,
    player: Query<&Transform, With<Player>>,
    input: Res<Input<KeyCode>>,
    asset_server: Res<AssetServer>,
    mut can_shoot: ResMut<PlayerCanShoot>,
) {
    if can_shoot.0 && input.pressed(KeyCode::Space) {
        can_shoot.0 = false;
        let projectile = SpriteBundle {
            transform: Transform {
                translation: Vec3 {
                    x: player.single().translation.x,
                    y: player.single().translation.y + 50.0,
                    z: 1.0,
                },
                ..default()
            },
            texture: asset_server.load("laserBlue07.png"),
            ..default()
        };
        commands.spawn((
            projectile,
            PlayerProjectile,
            ProjectileSize(Vec2::new(5.0, 40.0)),
            PlayerProjectileVelocity(Vec2::new(0.0, 2500.0)),
        ));
    }
}
