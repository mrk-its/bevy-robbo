use crate::components::{MovingDir, Position};
use bevy::ecs::Entity;

#[derive(Copy, Clone, Debug)]
pub enum GameEvent {
    Use(Entity, MovingDir),
    SpawnRobbo(Position),
    KillRobbo,
    PlaySound(&'static str),
}

#[derive(Copy, Clone, Debug)]
pub struct ReloadLevel(pub i32);
