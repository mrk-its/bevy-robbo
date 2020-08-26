use crate::components::Position;
use bevy::prelude::*;

pub struct DamageEvent {
    pub position: Position
}

#[derive(Default)]
pub struct DamageState {
    pub damage_reader: EventReader<DamageEvent>,
}
