use crate::components::{MovingDir, Position};
use bevy::ecs::Entity;
use std::mem::take;

#[derive(Copy, Clone)]
pub enum GameEvent {
    Damage(Position, bool),
    Use(Entity, MovingDir),
    ReloadLevel(i32),
    SpawnRobbo(Position),
    KillRobbo,
    PlaySound(&'static str),
}

#[derive(Default)]
pub struct GameEvents {
    events: Vec<GameEvent>,
}

impl GameEvents {
    pub fn send(&mut self, event: GameEvent) {
        self.events.push(event);
    }
    pub fn take(&mut self) -> Vec<GameEvent> {
        take(&mut self.events)
    }
}
