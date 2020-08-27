use crate::components::Position;
use bevy::prelude::*;

pub enum Event {
    Damage(Position),
    Remove(Position),
}

#[derive(Default)]
pub struct DamageState {
    pub damage_reader: EventReader<Event>,
}
