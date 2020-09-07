use crate::components::Collectable;
use crate::game_events::GameEvent;
use crate::sounds;
use bevy::prelude::*;

#[derive(Default, Debug)]
pub struct Inventory {
    pub keys: usize,
    pub screws: usize,
    pub bullets: usize,
}

impl Inventory {
    pub fn collect(&mut self, item: Collectable, events: &mut ResMut<Events<GameEvent>>) {
        match item {
            Collectable::Key => {
                self.keys += 1;
                events.send(GameEvent::PlaySound(sounds::KEY));
            }
            Collectable::Screw => {
                self.screws += 1;
                events.send(GameEvent::PlaySound(sounds::SCREW));
            }
            Collectable::Ammo => {
                self.bullets += 9;
                events.send(GameEvent::PlaySound(sounds::AMMO));
            }
        }
        self.show();
    }
    pub fn show(&self) {
        println!("{:?}", self);
    }
}
