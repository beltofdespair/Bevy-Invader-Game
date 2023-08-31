use crate::types::AppState;
use crate::Opponent;
use crate::Player;
const PADDLE_SPEED: f32 = 250.0;
use bevy::prelude::*;

pub fn movement_system(
    mut next_state: ResMut<NextState<AppState>>,
    mut player: Query<(&mut Transform, &Player), Without<Opponent>>,
    mut opponent: Query<(&mut Transform, &Opponent), Without<Player>>,
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    for mut transform in player.iter_mut() {
        let mut dir = Vec3::new(0.0, 0.0, 0.0);
        if transform.0.translation.y < 310.0 && input.pressed(KeyCode::W) {
            dir.y = 1.0;
        }
        if transform.0.translation.y > -310.0 && input.pressed(KeyCode::S) {
            dir.y = -1.0;
        }
        let velocity =
            dir.normalize_or_zero() * PADDLE_SPEED * time.delta_seconds();
        transform.0.translation += velocity;
    }

    for mut transform in opponent.iter_mut() {
        let mut dir = Vec3::new(0.0, 0.0, 0.0);
        if transform.0.translation.y < 310.0 && input.pressed(KeyCode::Up) {
            dir.y = 1.0;
        }
        if transform.0.translation.y > -310.0 && input.pressed(KeyCode::Down) {
            dir.y = -1.0;
        }
        let velocity =
            dir.normalize_or_zero() * PADDLE_SPEED * time.delta_seconds();
        transform.0.translation += velocity;
    }
    if input.just_pressed(KeyCode::Escape) {
        next_state.set(AppState::Paused);
    }
}
