use crate::components::Collectable;
use crate::plugins::audio::Sound;
use bevy::prelude::*;

#[derive(Default, Debug)]
pub struct Inventory {
    pub keys: usize,
    pub screws: usize,
    pub bullets: usize,
}

impl Inventory {
    pub fn collect(&mut self, item: Collectable, events: &mut ResMut<Events<Sound>>) {
        match item {
            Collectable::Key => {
                self.keys += 1;
                events.send(Sound::KEY);
            }
            Collectable::Screw => {
                self.screws += 1;
                events.send(Sound::SCREW);
            }
            Collectable::Ammo => {
                self.bullets += 9;
                events.send(Sound::AMMO);
            }
        }
    }
}
