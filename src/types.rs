use bevy::prelude::*;

#[derive(Default, States, Debug, Clone, Eq, PartialEq, Hash)]
pub enum AppState {
    #[default]
    MainMenu,
    InGame,
    Paused,
    NextLevel,
    EndGame,
}

#[derive(Resource)]
pub struct Scoreboard {
    pub player_score: usize,
}

#[derive(Resource)]
pub struct PlayerCanShoot(pub bool);

#[derive(Resource)]
pub struct LowRowRandCol {
    pub low_row: u8,
    pub rand_col: u8,
}

#[derive(Resource)]
pub struct BrigandCanShoot(pub bool);

#[derive(Resource)]
pub struct Level(pub u8);

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

#[derive(Resource)]
pub struct MovingDown(pub bool);

#[derive(Resource)]
pub struct MovingRight(pub bool);

#[derive(Resource)]
pub struct Lives(pub u8);
