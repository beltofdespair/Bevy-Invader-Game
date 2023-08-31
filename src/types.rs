use bevy::prelude::*;

#[derive(Copy, Clone, Component, Deref, DerefMut)]
pub struct Velocity(pub Vec2);

#[derive(Component)]
pub struct Ball;

#[derive(Component, Deref, DerefMut)]
pub struct Dir(pub Vec2);

#[derive(Component)]
pub struct Wall;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Opponent;

#[derive(Component)]
pub struct Camera;

#[derive(Component)]
pub struct Divider;

#[derive(Component)]
pub struct Background;

#[derive(Component)]
pub struct Collider;

#[derive(Resource)]
pub struct Scoreboard {
    pub player_score: usize,
    pub opponent_score: usize,
}

#[derive(Component)]
pub struct PlayerScore;

#[derive(Component)]
pub struct OpponentScore;

#[derive(Default, States, Debug, Clone, Eq, PartialEq, Hash)]
pub enum AppState {
    #[default]
    MainMenu,
    InGame,
    Paused,
}

#[derive(Resource)]
pub struct MenuData {
    pub menu_text: Entity,
    pub enter_text: Entity,
}

#[derive(Resource)]
pub struct PauseData {
    pub pause_text: Entity,
    pub pause_shadow: Entity,
}
