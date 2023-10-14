use bevy::prelude::*;

#[derive(Copy, Clone, Component, Deref, DerefMut)]
pub struct PlayerProjectileVelocity(pub Vec2);

#[derive(Copy, Clone, Component, Deref, DerefMut)]
pub struct BrigandProjectileVelocity(pub Vec2);

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct LifeSprite(pub u8);

#[derive(Component)]
pub struct PlayerProjectile;

#[derive(Component)]
pub struct Brigand;

#[derive(Component)]
pub struct BrigandProjectile;

#[derive(Component)]
pub struct Cam;

#[derive(Component)]
pub struct Barrier;

#[derive(Component)]
pub struct BarrierCollider;

#[derive(Component)]
pub struct Projectile;

#[derive(Component)]
pub struct ProjectileSize(pub Vec2);

#[derive(Component)]
pub struct BrigandSize(pub Vec2);

#[derive(Component)]
pub struct BarrierSize(pub Vec2);

#[derive(Component)]
pub struct PlayerSize(pub Vec2);

#[derive(Component)]
pub struct PlayerProjectileCollider;

#[derive(Component)]
pub struct RightWall;

#[derive(Component)]
pub struct LeftWall;

#[derive(Component)]
pub struct PlayerScore;

#[derive(Component, Deref, DerefMut)]
pub struct PPDelayTimer(pub Timer);

#[derive(Component, Deref, DerefMut)]
pub struct NextLevelTimer(pub Timer);

#[derive(Component, Deref, DerefMut)]
pub struct BPDelayTimer(pub Timer);

#[derive(Component, Deref, DerefMut)]
pub struct DownTimer(pub Timer);

#[derive(Component)]
pub struct RowCol {
    pub row: u8,
    pub col: u8,
}
