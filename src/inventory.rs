use crate::components::Collectable;
#[derive(Default, Debug)]
pub struct Inventory {
    pub keys: usize,
    pub screws: usize,
    pub bullets: usize,
}

impl Inventory {
    pub fn collect(&mut self, item: Collectable) {
        match item {
            Collectable::Key => self.keys += 1,
            Collectable::Screw => self.screws += 1,
            Collectable::Ammo => self.bullets += 9,
        }
        println!("inventory: {:?}", self);
    }
}

