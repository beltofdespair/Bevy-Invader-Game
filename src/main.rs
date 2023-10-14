mod collisions;
mod components;
mod input;
mod setup;
mod types;
use bevy::{prelude::*, window::WindowResolution};
use rand::*;

use crate::collisions::*;
use crate::components::*;
use crate::input::*;
use crate::setup::*;
use crate::types::*;
// use bevy::window::PrimaryWindow;
// use bevy::winit;
// use bevy::winit::Monitor::*;
// use bevy::winit::WinitWindows;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins.set(ImagePlugin::default_nearest()).set(
            WindowPlugin {
                primary_window: Some(Window {
                    // title: "I am a window!".into(),
                    resolution: WindowResolution::new(700.0, 900.0),
                    // .with_scale_factor_override(1.0),
                    // resolution: (700., 900.).into(),
                    // present_mode: PresentMode::AutoVsync,
                    // Tells wasm to resize the window according to the available canvas
                    // fit_canvas_to_parent: true,
                    // Tells wasm not to override default event handling, like F5, Ctrl+R etc.
                    // prevent_default_event_handling: false,
                    // window_theme: Some(WindowTheme::Dark),
                    ..default()
                }),
                ..default()
            },
        ),))
        .insert_resource(LowRowRandCol {
            low_row: 0,
            rand_col: rand::thread_rng().gen_range(0..11),
        })
        .insert_resource(MovingRight(true))
        .insert_resource(Level(0))
        .insert_resource(MovingDown(false))
        .insert_resource(Lives(3))
        .insert_resource(PlayerCanShoot(true))
        .insert_resource(BrigandCanShoot(true))
        .insert_resource(Scoreboard { player_score: 0 })
        .add_state::<AppState>()
        .add_systems(OnEnter(AppState::MainMenu), (menu_setup, camera_setup))
        .add_systems(Update, menu.run_if(in_state(AppState::MainMenu)))
        .add_systems(
            OnExit(AppState::MainMenu),
            (
                cleanup_menu,
                barriers_setup,
                player_setup,
                brigands_setup,
                show_lives,
            ),
        )
        .add_systems(OnEnter(AppState::Paused), pause_setup)
        .add_systems(Update, pause.run_if(in_state(AppState::Paused)))
        .add_systems(OnExit(AppState::Paused), cleanup_pause)
        .add_systems(OnEnter(AppState::EndGame), end_game)
        .add_systems(OnEnter(AppState::NextLevel), next_level)
        .add_systems(
            OnExit(AppState::NextLevel),
            (brigands_setup, barriers_setup),
        )
        .add_systems(OnExit(AppState::EndGame), initialize_resources)
        .add_systems(
            Update,
            next_level_timer.run_if(in_state(AppState::NextLevel)),
        )
        .add_systems(
            Update,
            (
                apply_pp_velocity,
                apply_brigand_projectile_velocity,
                movement_system,
                player_projectile_system,
                move_brigands,
                gen_bp,
                pp_timeout,
                bp_timeout,
                down_timeout,
                gen_rowcol,
                update_lives,
                check_zero_brigands,
            )
                .run_if(in_state(AppState::InGame)),
        )
        .add_systems(
            Update,
            (
                check_player_brigand_collision,
                check_pp_brigand_collision,
                check_pp_top_screen_collision,
                check_pp_bp_collision,
                check_pp_barrier_collision,
                check_brigand_left_wall_collision,
                check_brigand_right_wall_collision,
                check_bp_player_collision,
                check_bp_barrier_collision,
                check_bp_bottom_screen_collision,
                check_brigand_barrier_collision,
            )
                .run_if(in_state(AppState::InGame)),
        )
        .run();
}

fn update_lives(
    mut commands: Commands,
    mut lifesprite_q: Query<(Entity, &LifeSprite), With<LifeSprite>>,
    lives: Res<Lives>,
) {
    for (lifesprite, life) in &mut lifesprite_q {
        match lives.0 {
            n => {
                if life.0 == n {
                    commands.entity(lifesprite).despawn();
                }
            }
        }
    }
}

fn initialize_resources(
    mut low_row_rand_col: ResMut<LowRowRandCol>,
    mut move_right: ResMut<MovingRight>,
    mut move_down: ResMut<MovingDown>,
    mut lives: ResMut<Lives>,
    mut p_can_shoot: ResMut<PlayerCanShoot>,
    mut b_can_shoot: ResMut<BrigandCanShoot>,
    mut level: ResMut<Level>,
) {
    low_row_rand_col.low_row = 0;
    low_row_rand_col.rand_col = rand::thread_rng().gen_range(0..11);
    move_right.0 = true;
    move_down.0 = false;
    lives.0 = 3;
    p_can_shoot.0 = true;
    b_can_shoot.0 = true;
    level.0 += 0;
}

fn next_level(
    mut low_row_rand_col: ResMut<LowRowRandCol>,
    mut move_right: ResMut<MovingRight>,
    mut move_down: ResMut<MovingDown>,
    mut p_can_shoot: ResMut<PlayerCanShoot>,
    mut b_can_shoot: ResMut<BrigandCanShoot>,
    mut level: ResMut<Level>,
    mut commands: Commands,
) {
    low_row_rand_col.low_row = 0;
    low_row_rand_col.rand_col = rand::thread_rng().gen_range(0..11);
    move_right.0 = true;
    move_down.0 = false;
    p_can_shoot.0 = true;
    b_can_shoot.0 = true;
    level.0 += 1;
    commands.spawn(NextLevelTimer(Timer::from_seconds(0.2, TimerMode::Once)));
}

fn end_game(
    mut commands: Commands,
    mut player_q: Query<Entity, With<Player>>,
    mut barrier_q: Query<Entity, With<Barrier>>,
    mut pp_q: Query<Entity, With<PlayerProjectile>>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    for player in &mut player_q {
        commands.entity(player).despawn();
    }
    for pp in &mut pp_q {
        commands.entity(pp).despawn();
    }
    for barrier in &mut barrier_q {
        commands.entity(barrier).despawn();
    }
    next_state.set(AppState::MainMenu);
}

fn gen_bp(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    mut b_can_shoot: ResMut<BrigandCanShoot>,
    brigands: Query<(&Transform, &RowCol), With<Brigand>>,
    low_row_rand_col: Res<LowRowRandCol>,
) {
    if b_can_shoot.0 {
        for (transform, rowcol) in &brigands {
            if rowcol.row == low_row_rand_col.low_row
                && rowcol.col == low_row_rand_col.rand_col
            {
                let projectile = SpriteBundle {
                    transform: Transform {
                        translation: Vec3 {
                            x: transform.translation.x,
                            y: transform.translation.y - 50.0,
                            ..default()
                        },
                        ..default()
                    },
                    texture: asset_server.load("laserRed07.png"),
                    ..default()
                };
                commands.spawn((
                    projectile,
                    BrigandProjectile,
                    ProjectileSize(Vec2::new(5.0, 40.0)),
                    BrigandProjectileVelocity(Vec2::new(0.0, 2000.0)),
                ));
                b_can_shoot.0 = false;
            }
        }
    }
}

fn gen_rowcol(
    mut low_row_rand_col: ResMut<LowRowRandCol>,
    brigands: Query<&RowCol, With<Brigand>>,
) {
    (low_row_rand_col.low_row, low_row_rand_col.rand_col) =
        (4, rand::thread_rng().gen_range(0..11));
    for rowcol in &brigands {
        if rowcol.col == low_row_rand_col.rand_col
            && rowcol.row < low_row_rand_col.low_row
        {
            low_row_rand_col.low_row = rowcol.row;
        }
    }
}

fn pp_timeout(
    mut can_shoot: ResMut<PlayerCanShoot>,
    time: Res<Time>,
    mut timer_query: Query<&mut PPDelayTimer>,
) {
    for mut timer in &mut timer_query {
        if timer.tick(time.delta()).just_finished() {
            can_shoot.0 = true;
        }
    }
}

fn bp_timeout(
    mut can_shoot: ResMut<BrigandCanShoot>,
    time: Res<Time>,
    mut timer_query: Query<&mut BPDelayTimer>,
) {
    for mut timer in &mut timer_query {
        if timer.tick(time.delta()).just_finished() {
            can_shoot.0 = true;
        }
    }
}

fn down_timeout(
    time: Res<Time>,
    mut timer_query: Query<&mut DownTimer>,
    mut move_down: ResMut<MovingDown>,
) {
    for mut timer in &mut timer_query {
        if timer.tick(time.delta()).just_finished() {
            move_down.0 = false;
        }
    }
}

fn move_brigands(
    move_right: ResMut<MovingRight>,
    move_down: ResMut<MovingDown>,
    mut brigands: Query<&mut Transform, With<Brigand>>,
) {
    let brigands_count = brigands.iter().count();
    for mut transform in &mut brigands {
        if move_down.0 {
            transform.translation.y -= 5.0;
        } else if move_right.0 {
            transform.translation.x += 30.0 * (1.0 / brigands_count as f32);
        } else {
            transform.translation.x -= 30.0 * (1.0 / brigands_count as f32);
        }
    }
}

fn next_level_timer(
    time: Res<Time>,
    mut timer_query: Query<&mut NextLevelTimer>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    for mut timer in &mut timer_query {
        if timer.tick(time.delta()).just_finished() {
            next_state.set(AppState::InGame);
        }
    }
}

fn check_zero_brigands(
    mut next_state: ResMut<NextState<AppState>>,
    brigands: Query<With<Brigand>>,
) {
    if brigands.iter().count() == 0 {
        next_state.set(AppState::NextLevel);
    }
}

fn apply_pp_velocity(
    mut query: Query<(&mut Transform, &PlayerProjectileVelocity)>,
    time_step: Res<FixedTime>,
) {
    for (mut transform, velocity) in &mut query {
        transform.translation.y += velocity.y * time_step.period.as_secs_f32();
    }
}

fn apply_brigand_projectile_velocity(
    mut query: Query<(&mut Transform, &BrigandProjectileVelocity)>,
    time_step: Res<FixedTime>,
) {
    for (mut transform, velocity) in &mut query {
        transform.translation.y -= velocity.y * time_step.period.as_secs_f32();
    }
}

fn cleanup_menu(mut commands: Commands, menu: Res<MenuData>) {
    commands.entity(menu.menu_text).despawn();
    commands.entity(menu.enter_text).despawn();
}

fn cleanup_pause(mut commands: Commands, pause: Res<PauseData>) {
    commands.entity(pause.pause_text).despawn();
    commands.entity(pause.pause_shadow).despawn();
}

// fn get_monitor_size(
//     winit_windows: NonSend<WinitWindows>,
//     window_query: Query<Entity, With<PrimaryWindow>>,
// ) -> winit::dpi::PhysicalSize<u32> {
//     if let Some(monitor) = window_query
//         .get_single()
//         .ok()
//         .and_then(|entity| winit_windows.get_window(entity))
//         .and_then(|winit_window| winit_window.primary_monitor())
//     // .and_then(|winit_window| winit_window.current_monitor())
//     {
//         monitor.size()
//     }
// }
