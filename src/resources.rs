
use std::collections::HashMap;
use crate::components::Position;

#[derive(Default)]
pub struct DamageMap(pub HashMap<Position, bool>);

impl DamageMap {
    pub fn take(&mut self) -> HashMap<Position, bool> {
        std::mem::take(&mut self.0)
    }
    pub fn do_damage(&mut self, pos: &Position, is_bomb: bool) {
        self.0.insert(*pos, is_bomb);
    }
    pub fn is_damaged(&self, pos: &Position) -> bool {
        return self.0.contains_key(pos)
    }
}