use crate::types::*;
use crate::INITIAL_BALL_SPEED;
use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;
use rand::prelude::*;

pub fn camera_setup(mut commands: Commands) {
    let camera_2d = Camera2d {
        clear_color: bevy::core_pipeline::clear_color::ClearColorConfig::Custom(
            Color::BLACK,
        ),
    };
    let camera = Camera2dBundle {
        camera_2d,
        ..default()
    };
    commands.spawn((camera, Camera));
}

pub fn player_setup(mut commands: Commands) {
    let transform = Transform {
        translation: Vec3 {
            x: -450.0,
            z: 1.0,
            ..default()
        },
        scale: Vec3 {
            x: 15.0,
            y: 80.0,
            ..default()
        },
        ..default()
    };
    let paddle = SpriteBundle {
        transform,
        ..default()
    };
    commands.spawn((paddle, (Player, Collider)));
}

pub fn opponent_setup(mut commands: Commands) {
    let transform = Transform {
        translation: Vec3 {
            x: 450.0,
            z: 1.0,
            ..default()
        },
        scale: Vec3 {
            x: 15.0,
            y: 80.0,
            ..default()
        },
        ..default()
    };
    let paddle = SpriteBundle {
        transform,
        ..default()
    };
    commands.spawn((paddle, (Opponent, Collider)));
}

pub fn divider_setup(mut commands: Commands) {
    for y_pos in (-340..380).step_by(40) {
        let transform = Transform {
            translation: Vec3 {
                y: y_pos as f32,
                z: 1.0,
                ..default()
            },
            ..default()
        };
        let divider = SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(2.0, 20.0)),
                ..default()
            },
            transform,
            ..default()
        };
        commands.spawn((divider, Divider));
    }
}

pub fn ball_setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
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
    let ball_dir = Dir(Vec2::new(1.0 * x_sign as f32, y * y_sign as f32));
    let ball = MaterialMesh2dBundle {
        mesh: meshes.add(shape::Circle::default().into()).into(),
        material: materials.add(ColorMaterial::from(Color::WHITE)),
        transform: Transform::from_translation(Vec3::default()).with_scale(
            Vec3 {
                x: 20.0,
                y: 20.0,
                z: 0.0,
            },
        ),
        ..default()
    };
    let velocity = Velocity(ball_dir.0.normalize() * INITIAL_BALL_SPEED);
    commands.spawn((ball, (Ball, velocity, ball_dir)));
}

pub fn wall_setup(mut commands: Commands) {
    let top_wall = SpriteBundle {
        transform: Transform {
            scale: Vec3 {
                x: 1400.0,
                y: 15.0,
                ..default()
            },
            translation: Vec3 {
                y: 364.0,
                ..default()
            },
            ..default()
        },
        ..default()
    };
    let bottom_wall = SpriteBundle {
        transform: Transform {
            scale: Vec3 {
                x: 1400.0,
                y: 15.0,
                ..default()
            },
            translation: Vec3 {
                y: -364.0,
                ..default()
            },
            ..default()
        },
        ..default()
    };
    commands.spawn((top_wall, (Wall, Collider)));
    commands.spawn((bottom_wall, (Wall, Collider)));
}

pub fn score_setup(mut commands: Commands) {
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

    let opponent_text_style = TextStyle {
        font_size: 20.0,
        ..default()
    };
    let opponent_style = Style {
        position_type: PositionType::Absolute,
        top: Val::Px(10.0),
        right: Val::Px(10.0),
        ..default()
    };
    let opponent_score_text =
        TextBundle::from_section("Score: ", opponent_text_style)
            .with_style(opponent_style);
    commands.spawn((opponent_score_text, OpponentScore));
}

pub fn menu_setup(mut commands: Commands) {
    let menu_text_style = TextStyle {
        font_size: 80.0,
        color: Color::WHITE,
        ..default()
    };
    let menu_style = Style {
        position_type: PositionType::Absolute,
        top: Val::Px(103.0),
        left: Val::Px(570.0),
        ..default()
    };
    let menu_text = TextBundle::from_section("Bong", menu_text_style)
        .with_style(menu_style);

    let enter_text_style = TextStyle {
        font_size: 40.0,
        color: Color::WHITE,
        ..default()
    };
    let enter_style = Style {
        position_type: PositionType::Absolute,
        top: Val::Px(203.0),
        left: Val::Px(540.0),
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
        top: Val::Px(203.0),
        left: Val::Px(578.0),
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
        top: Val::Px(205.0),
        left: Val::Px(580.0),
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
