use crate::components::{MovingDir, Position};
use crate::levels::Level;
use bevy::ecs::Entity;
use bevy::asset::Handle;
use std::mem::take;

#[derive(Copy, Clone)]
pub enum GameEvent {
    Damage(Position, bool),
    RemoveEntity(Entity),
    Use(Entity, MovingDir),
    ReloadLevel(Handle<Level>),
    SpawnRobbo(Position),
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
