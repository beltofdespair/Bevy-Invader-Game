use crate::components::*;
use crate::types::*;
use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;

const BRIGAND_XSEP: f32 = 120.0;
const BRIGAND_YSEP: f32 = 175.0;
const CAMERA_SCALE: f32 = 3.0;

pub fn camera_setup(
    window: Query<&Window>,
    mut commands: Commands,
    mut cam_q: Query<Entity, With<Cam>>,
) {
    for cam in &mut cam_q {
        commands.entity(cam).despawn();
    }
    let camera_2d = Camera2d {
        clear_color: bevy::core_pipeline::clear_color::ClearColorConfig::Custom(
            Color::BLACK,
        ),
    };
    let camera = Camera2dBundle {
        projection: OrthographicProjection {
            scale: CAMERA_SCALE,
            near: -10.0,
            scaling_mode: bevy::render::camera::ScalingMode::Fixed {
                width: window.single().resolution.width(),
                height: window.single().resolution.height(),
            },
            ..default()
        },
        camera_2d,
        ..default()
    };
    commands.spawn((camera, Cam));
}

pub fn player_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let transform = Transform {
        translation: Vec3 {
            y: -1050.0,
            x: -600.0,
            ..default()
        },
        ..default()
    };
    let player = SpriteBundle {
        sprite: Sprite {
            color: Color::GREEN,
            ..default()
        },
        texture: asset_server.load("playerShip1_green.png"),
        transform,
        ..default()
    };
    commands.spawn((player, Player, PlayerSize(Vec2::new(75.0, 60.0))));
}

pub fn show_lives(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    lives: ResMut<Lives>,
) {
    for i in 1..lives.0 {
        let player = SpriteBundle {
            texture: asset_server.load("playerShip1_green.png"),
            sprite: Sprite {
                color: Color::GREEN,
                ..default()
            },
            transform: Transform {
                translation: Vec3 {
                    y: -1250.0,
                    x: -700.0 + (i as f32 * 125.0),
                    ..default()
                },
                ..default()
            },
            ..default()
        };
        commands.spawn((player, LifeSprite(i)));
    }
}

pub fn barriers_setup(
    mut barriers: Query<Entity, With<Barrier>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut commands: Commands,
) {
    for barrier in &mut barriers {
        commands.entity(barrier).despawn();
    }
    let mut topstartx = -525;
    let topstarty = -850;
    let mut botstartx = -525;
    let botstarty = -900;
    let step = 30;
    for _ in 0..4 {
        for x in (topstartx..topstartx + 150).step_by(step) {
            for y in (topstarty..topstarty + 150).step_by(step) {
                let block = MaterialMesh2dBundle {
                    mesh: meshes.add(shape::Quad::default().into()).into(),
                    material: materials.add(ColorMaterial::from(Color::GREEN)),
                    transform: Transform {
                        translation: Vec3 {
                            x: x as f32,
                            y: y as f32,
                            ..default()
                        },
                        scale: Vec3::new(30.0, 30.0, 1.0),
                        ..default()
                    },
                    ..default()
                };
                commands.spawn((
                    block,
                    Barrier,
                    BarrierSize(Vec2::new(30.0, 30.0)),
                ));
            }
        }
        topstartx += 300;
        for x in (botstartx..botstartx + step as i32).step_by(step) {
            for y in (botstarty..botstarty + 50).step_by(step) {
                let block = MaterialMesh2dBundle {
                    mesh: meshes.add(shape::Quad::default().into()).into(),
                    material: materials.add(ColorMaterial::from(Color::GREEN)),
                    transform: Transform {
                        translation: Vec3 {
                            x: x as f32,
                            y: y as f32,
                            ..default()
                        },
                        scale: Vec3::new(30.0, 30.0, 1.0),
                        ..default()
                    },
                    ..default()
                };
                commands.spawn((
                    block,
                    Barrier,
                    BarrierSize(Vec2::new(30.0, 30.0)),
                ));
            }
        }
        botstartx += 120;
        for x in (botstartx..botstartx + step as i32).step_by(30) {
            for y in (botstarty..botstarty + 50).step_by(step) {
                let block = MaterialMesh2dBundle {
                    mesh: meshes.add(shape::Quad::default().into()).into(),
                    material: materials.add(ColorMaterial::from(Color::GREEN)),
                    transform: Transform {
                        translation: Vec3 {
                            x: x as f32,
                            y: y as f32,
                            ..default()
                        },
                        scale: Vec3::new(30.0, 30.0, 1.0),
                        ..default()
                    },
                    ..default()
                };
                commands.spawn((
                    block,
                    Barrier,
                    BarrierSize(Vec2::new(30.0, 30.0)),
                ));
            }
        }
        botstartx += 180;
    }
}

pub fn brigands_setup(
    mut brigands_q: Query<Entity, With<Brigand>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    level: Res<Level>,
) {
    for brigand in &mut brigands_q {
        commands.entity(brigand).despawn();
    }
    let mut y = -100.0 - (level.0 as f32 * 75.0);
    for row in 0..5 {
        let mut x = -650.0;
        for col in 0..11 {
            let brigand = SpriteBundle {
                transform: Transform {
                    translation: Vec3 { y, x, ..default() },
                    ..default()
                },
                texture: match row {
                    0 => asset_server.load("enemyBlack4.png"),
                    1 => asset_server.load("enemyBlack3.png"),
                    2 => asset_server.load("enemyBlack2.png"),
                    3 => asset_server.load("enemyBlack1.png"),
                    _ => asset_server.load("enemyBlack5.png"),
                },
                sprite: Sprite {
                    color: Color::WHITE,
                    ..default()
                },
                ..default()
            };
            commands.spawn((
                brigand,
                (
                    Brigand,
                    RowCol { row, col },
                    PlayerProjectileCollider,
                    BrigandSize(Vec2::new(40.0, 40.0)),
                ),
            ));
            x += BRIGAND_XSEP;
        }
        y += BRIGAND_YSEP;
    }
}

pub fn _score_setup(mut commands: Commands) {
    let player_text_style = TextStyle {
        font_size: 20.0,
        ..default()
    };
    let player_style = Style {
        position_type: PositionType::Absolute,
        top: Val::Px(10.0),
        left: Val::Px(10.0),
        ..default()
    };
    let player_score_text =
        TextBundle::from_section("Score: ", player_text_style)
            .with_style(player_style);
    commands.spawn((player_score_text, PlayerScore));
}

pub fn menu_setup(mut commands: Commands) {
    let menu_text_style = TextStyle {
        font_size: 60.0,
        color: Color::WHITE,
        ..default()
    };
    let menu_style = Style {
        position_type: PositionType::Absolute,
        left: Val::Vw(10.0),
        // left: Val::Percent(50.0),
        top: Val::Px(103.0),
        // left: Val::Px(570.0),
        ..default()
    };
    let menu_text =
        TextBundle::from_section("Local Area Brigands", menu_text_style)
            .with_style(menu_style);

    let enter_text_style = TextStyle {
        font_size: 35.0,
        color: Color::WHITE,
        ..default()
    };
    let enter_style = Style {
        position_type: PositionType::Absolute,
        top: Val::Px(403.0),
        left: Val::Px(240.0),
        ..default()
    };
    let enter_text = TextBundle::from_section("Press Enter", enter_text_style)
        .with_style(enter_style);

    let menu_text_id = commands.spawn(menu_text).id();
    let menu_enter_id = commands.spawn(enter_text).id();
    commands.insert_resource(MenuData {
        menu_text: menu_text_id,
        enter_text: menu_enter_id,
    });
}

pub fn pause_setup(mut commands: Commands) {
    let text_style = TextStyle {
        font_size: 40.0,
        color: Color::WHITE,
        ..default()
    };
    let style = Style {
        position_type: PositionType::Absolute,
        top: Val::Px(103.0),
        left: Val::Px(278.0),
        ..default()
    };
    let exit_text_bundle =
        TextBundle::from_section("Exit? y/n", text_style).with_style(style);

    let text_style_shadow = TextStyle {
        font_size: 40.0,
        color: Color::DARK_GRAY,
        ..default()
    };
    let style_shadow = Style {
        position_type: PositionType::Absolute,
        top: Val::Px(105.0),
        left: Val::Px(280.0),
        ..default()
    };
    let exit_text_bundle_shadow =
        TextBundle::from_section("Exit? y/n", text_style_shadow)
            .with_style(style_shadow);
    let pause_shadow_id = commands.spawn(exit_text_bundle_shadow).id();
    let pause_text_id = commands.spawn(exit_text_bundle).id();
    commands.insert_resource(PauseData {
        pause_shadow: pause_shadow_id,
        pause_text: pause_text_id,
    });
}
