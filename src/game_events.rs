use crate::components::Position;
use std::mem::take;
use bevy::ecs::Entity;

pub enum GameEvent {
    Damage(Position, bool),
    Remove(Position),
    RemoveEntity(Entity),
    Use(Position),
}

#[derive(Default)]
pub struct GameEvents {
    events: Vec<GameEvent>
}

impl GameEvents {
    pub fn send(&mut self, event: GameEvent) {
        self.events.push(event);
    }
    pub fn take(&mut self) -> Vec<GameEvent> {
        take(&mut self.events)
    }
}
