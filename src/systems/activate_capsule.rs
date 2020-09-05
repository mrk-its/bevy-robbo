use bevy::prelude::*;
use crate::inventory::Inventory;
use crate::levels::Level;
use crate::components::prelude::*;
use crate::entities::repair_capsule;

pub fn activate_capsule_system(
    mut commands: Commands,
    inventory: Res<Inventory>,
    levels: Res<Assets<Level>>,
    current_level_handle: Res<Option<Handle<Level>>>,
    mut query: Query<With<Capsule, Without<Usable, Entity>>>,
) {
    for capsule in &mut query.iter() {
        if let Some(handle) = *current_level_handle {
            if let Some(level) = levels.get(&handle) {
                if inventory.screws >= level.screw_count {
                    println!("activating capsule");
                    repair_capsule(&mut commands, capsule);
                }
            }
        }
    }
}
