use bevy::prelude::*;

#[derive(Component, Default)]
pub struct Enemy {
    pub direction: Vec3,
    pub health: i32,
    pub max_health: i32,
    pub speed: f32,
    pub enemy_type: EnemyType,
    pub state: EnemyState,
    pub attack_active: bool,
    pub damage: i32,
}

#[derive(Default)]
pub enum EnemyType {
    #[default]
    Other, // Other in any type will crash the game. Do not instantiate anything with an Other type.
    Swordsman,
    Archer,
    Mage,
}

use bevy::{prelude::Resource, time::Timer, time::TimerMode};
pub const ENEMY_SPAWN_TIME: u8 = 2;

#[derive(Resource)]
pub struct EnemySpawnTimer {
    pub timer: Timer,
}
impl Default for EnemySpawnTimer {
    fn default() -> EnemySpawnTimer {
        EnemySpawnTimer {
            timer: Timer::from_seconds(ENEMY_SPAWN_TIME as f32, TimerMode::Repeating),
        }
    }
}

#[derive(Component)]
pub struct AnimationIndices {
    pub first: usize,
    pub last: usize,
    pub delete_on_end: bool,
}

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(pub Timer);

#[derive(Default)]
pub enum EnemyState {
    #[default]
    Moving,
    Attacking,
    AttackPossible,
}

#[derive(Component)]
pub struct DeleteAfterAnimation {}

#[derive(Component, Deref, DerefMut)]
pub struct CooldownTimer(pub Timer);

#[derive(Component)]
pub struct SwordsmanMarker;

#[derive(Component)]
pub struct ArcherMarker;

#[derive(Resource, Default)]
pub struct TotalEnemySpawns {
    pub total_spawns: u64,
}

#[derive(Component)]
pub struct EnemyAnimationMarker;