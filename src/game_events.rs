use crate::components::{MovingDir, Position};
use bevy::ecs::Entity;

#[derive(Copy, Clone, Debug)]
pub enum GameEvent {
    Use(Entity, Position, MovingDir),
    ReloadLevel(i32),
    SpawnRobbo(Position),
    PreSpawnRobbo(Position),
    SpawnRandom(Position),
    KillRobbo,
    PlaySound(&'static str),
}
