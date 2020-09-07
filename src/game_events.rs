use crate::components::{MovingDir, Position};
use bevy::ecs::Entity;
use std::mem::take;

#[derive(Copy, Clone, Debug)]
pub enum GameEvent {
    Damage(Position, bool),
    Use(Entity, MovingDir),
    ReloadLevel(i32),
    SpawnRobbo(Position),
    KillRobbo,
    PlaySound(&'static str),
}
