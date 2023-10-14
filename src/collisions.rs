use crate::components::*;
use crate::types::*;
use bevy::prelude::*;
use bevy::sprite::collide_aabb::collide;

const TOP_SCREEN_TRANSLATION: Vec3 = Vec3::new(0.0, 1450.0, 1.0);
const TOP_SCREEN_SIZE: Vec2 = Vec2::new(3000.0, 200.0);
const BOTTOM_SCREEN_TRANSLATION: Vec3 = Vec3::new(0.0, -1450.0, 1.0);
const BOTTOM_SCREEN_SIZE: Vec2 = Vec2::new(3000.0, 200.0);
const BRIGAND_SIZE: Vec2 = Vec2::new(60.0, 60.0);
const BRIGAND_PROJECTILE_SIZE: Vec2 = Vec2::new(20.0, 40.0);
const RIGHT_WALL_TRANSLATION: Vec3 = Vec3::new(850.0, 0.0, 1.0);
const RIGHT_WALL_SIZE: Vec2 = Vec2::new(100.0, 2000.0);
const LEFT_WALL_TRANSLATION: Vec3 = Vec3::new(-850.0, 0.0, 1.0);
const LEFT_WALL_SIZE: Vec2 = Vec2::new(100.0, 2000.0);

pub fn check_pp_brigand_collision(
    mut commands: Commands,
    mut pp_query: Query<
        (Entity, &ProjectileSize, &Transform),
        With<PlayerProjectile>,
    >,
    mut brigand_query: Query<(Entity, &Transform), With<Brigand>>,
) {
    for (pp_entity, pp_size, pp_transform) in &mut pp_query {
        for (brigand, brigand_transform) in &mut brigand_query {
            if collide(
                pp_transform.translation,
                pp_size.0,
                brigand_transform.translation,
                BRIGAND_SIZE,
            )
            .is_some()
            {
                commands.entity(brigand).despawn();
                commands.entity(pp_entity).despawn();
                commands.spawn(PPDelayTimer(Timer::from_seconds(
                    0.2,
                    TimerMode::Once,
                )));
            }
        }
    }
}

pub fn check_pp_bp_collision(
    mut commands: Commands,
    mut brigand_projectile_query: Query<
        (Entity, &Transform),
        With<BrigandProjectile>,
    >,
    mut pp_query: Query<
        (Entity, &ProjectileSize, &Transform),
        With<PlayerProjectile>,
    >,
) {
    for (pp_entity, pp_size, pp_transform) in &mut pp_query {
        for (brigand_projectile, bp_transform) in &mut brigand_projectile_query
        {
            if collide(
                pp_transform.translation,
                pp_size.0,
                bp_transform.translation,
                BRIGAND_PROJECTILE_SIZE,
            )
            .is_some()
            {
                commands.entity(brigand_projectile).despawn();
                commands.spawn(BPDelayTimer(Timer::from_seconds(
                    1.0,
                    TimerMode::Once,
                )));
                commands.entity(pp_entity).despawn();
                commands.spawn(PPDelayTimer(Timer::from_seconds(
                    0.2,
                    TimerMode::Once,
                )));
            }
        }
    }
}

pub fn check_bp_player_collision(
    mut commands: Commands,
    mut bp_query: Query<
        (Entity, &ProjectileSize, &Transform),
        (With<BrigandProjectile>, Without<Player>),
    >,
    mut player_query: Query<
        (&PlayerSize, &mut Transform),
        (With<Player>, Without<BrigandProjectile>),
    >,
    mut b_can_shoot: ResMut<BrigandCanShoot>,
    mut lives: ResMut<Lives>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    for (bp_entity, bp_size, bp_transform) in &mut bp_query {
        for (player_size, mut player_transform) in &mut player_query {
            if collide(
                bp_transform.translation,
                bp_size.0,
                player_transform.translation,
                player_size.0,
            )
            .is_some()
            {
                b_can_shoot.0 = false;
                commands.entity(bp_entity).despawn();
                commands.spawn(BPDelayTimer(Timer::from_seconds(
                    0.5,
                    TimerMode::Once,
                )));
                if lives.0 > 1 {
                    lives.0 -= 1;
                    player_transform.translation.x = -600.0;
                } else {
                    next_state.set(AppState::EndGame);
                }
            }
        }
    }
}

pub fn check_brigand_barrier_collision(
    mut commands: Commands,
    mut brigand_query: Query<(&BrigandSize, &Transform), With<Brigand>>,
    mut barrier_query: Query<(Entity, &BarrierSize, &Transform), With<Barrier>>,
) {
    for (brigand_size, brigand_transform) in &mut brigand_query {
        for (barrier, barrier_size, barrier_transform) in &mut barrier_query {
            if collide(
                brigand_transform.translation,
                brigand_size.0,
                barrier_transform.translation,
                barrier_size.0,
            )
            .is_some()
            {
                commands.entity(barrier).despawn();
            }
        }
    }
}

pub fn check_bp_barrier_collision(
    mut commands: Commands,
    mut bp_query: Query<
        (Entity, &ProjectileSize, &Transform),
        With<BrigandProjectile>,
    >,
    mut barrier_query: Query<(Entity, &BarrierSize, &Transform), With<Barrier>>,
    mut b_can_shoot: ResMut<BrigandCanShoot>,
) {
    for (projectile, projectile_size, projectile_transform) in &mut bp_query {
        for (barrier, barrier_size, barrier_transform) in &mut barrier_query {
            if collide(
                projectile_transform.translation,
                projectile_size.0,
                barrier_transform.translation,
                barrier_size.0,
            )
            .is_some()
            {
                b_can_shoot.0 = false;
                commands.entity(barrier).despawn();
                commands.entity(projectile).despawn();
                commands.spawn(BPDelayTimer(Timer::from_seconds(
                    0.5,
                    TimerMode::Once,
                )));
            }
        }
    }
}

pub fn check_pp_barrier_collision(
    mut commands: Commands,
    mut bp_query: Query<
        (Entity, &ProjectileSize, &Transform),
        With<PlayerProjectile>,
    >,
    mut barrier_query: Query<(Entity, &BarrierSize, &Transform), With<Barrier>>,
    mut p_can_shoot: ResMut<PlayerCanShoot>,
) {
    for (projectile, projectile_size, projectile_transform) in &mut bp_query {
        for (barrier, barrier_size, barrier_transform) in &mut barrier_query {
            if collide(
                projectile_transform.translation,
                projectile_size.0,
                barrier_transform.translation,
                barrier_size.0,
            )
            .is_some()
            {
                p_can_shoot.0 = false;
                commands.entity(barrier).despawn();
                commands.entity(projectile).despawn();
                commands.spawn(PPDelayTimer(Timer::from_seconds(
                    0.2,
                    TimerMode::Once,
                )));
            }
        }
    }
}

pub fn check_player_brigand_collision(
    mut commands: Commands,
    mut brigand_query: Query<
        (Entity, &Transform),
        (With<Brigand>, Without<Player>),
    >,
    mut next_state: ResMut<NextState<AppState>>,
    mut player_query: Query<
        (&PlayerSize, &mut Transform),
        (With<Player>, Without<Brigand>),
    >,
    mut lives: ResMut<Lives>,
) {
    for (player_size, mut player_transform) in &mut player_query {
        for (brigand, brigand_transform) in &mut brigand_query {
            if collide(
                player_transform.translation,
                player_size.0,
                brigand_transform.translation,
                BRIGAND_SIZE,
            )
            .is_some()
            {
                commands.entity(brigand).despawn();
                if lives.0 > 0 {
                    lives.0 -= 1;
                    player_transform.translation.x = -600.0;
                } else {
                    next_state.set(AppState::EndGame);
                }
            }
        }
    }
}

pub fn check_pp_top_screen_collision(
    mut commands: Commands,
    mut pp_query: Query<
        (Entity, &ProjectileSize, &Transform),
        With<PlayerProjectile>,
    >,
) {
    for (pp_entity, pp_size, pp_transform) in &mut pp_query {
        if collide(
            pp_transform.translation,
            pp_size.0,
            TOP_SCREEN_TRANSLATION,
            TOP_SCREEN_SIZE,
        )
        .is_some()
        {
            commands.entity(pp_entity).despawn();
            commands
                .spawn(PPDelayTimer(Timer::from_seconds(0.2, TimerMode::Once)));
        }
    }
}

pub fn check_bp_bottom_screen_collision(
    mut commands: Commands,
    bp_query: Query<
        (Entity, &ProjectileSize, &Transform),
        With<BrigandProjectile>,
    >,
) {
    for (bp_entity, bp_size, bp_transform) in &bp_query {
        if collide(
            bp_transform.translation,
            bp_size.0,
            BOTTOM_SCREEN_TRANSLATION,
            BOTTOM_SCREEN_SIZE,
        )
        .is_some()
        {
            commands.entity(bp_entity).despawn();
            commands
                .spawn(BPDelayTimer(Timer::from_seconds(1.0, TimerMode::Once)));
        }
    }
}

pub fn check_brigand_right_wall_collision(
    mut move_right: ResMut<MovingRight>,
    mut move_down: ResMut<MovingDown>,
    mut commands: Commands,
    mut brigand_query: Query<&Transform, With<Brigand>>,
) {
    for transform in &mut brigand_query {
        if collide(
            transform.translation,
            BRIGAND_SIZE,
            RIGHT_WALL_TRANSLATION,
            RIGHT_WALL_SIZE,
        )
        .is_some()
            && move_right.0
        {
            move_down.0 = true;
            move_right.0 = false;
            commands
                .spawn(DownTimer(Timer::from_seconds(0.2, TimerMode::Once)));
        }
    }
}

pub fn check_brigand_left_wall_collision(
    mut move_right: ResMut<MovingRight>,
    mut move_down: ResMut<MovingDown>,
    mut commands: Commands,
    mut brigand_query: Query<&Transform, With<Brigand>>,
) {
    for transform in &mut brigand_query {
        if collide(
            transform.translation,
            BRIGAND_SIZE,
            LEFT_WALL_TRANSLATION,
            LEFT_WALL_SIZE,
        )
        .is_some()
            && !move_right.0
        {
            move_down.0 = true;
            move_right.0 = true;
            commands
                .spawn(DownTimer(Timer::from_seconds(0.2, TimerMode::Once)));
        }
    }
}
