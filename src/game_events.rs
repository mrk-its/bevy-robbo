use crate::components::{MovingDir, Position};
use bevy::ecs::Entity;
use std::mem::take;

pub enum GameEvent {
    Damage(Position, bool),
    RemoveEntity(Entity),
    Use(Entity, MovingDir),
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
    pub fn flush(&mut self) {
        self.take();
    }
}
