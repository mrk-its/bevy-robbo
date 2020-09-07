use crate::components::prelude::*;
use crate::entities::repair_capsule;
use crate::inventory::Inventory;
use crate::levels::LevelInfo;
use crate::game_events::GameEvent;
use crate::sounds;

use bevy::prelude::*;

pub fn activate_capsule_system(
    mut commands: Commands,
    inventory: Res<Inventory>,
    level_info: Res<LevelInfo>,
    mut game_events: ResMut<Events<GameEvent>>,
    mut query: Query<With<Capsule, Without<Usable, Entity>>>,
) {
    for capsule in &mut query.iter() {
        if inventory.screws >= level_info.screws {
            println!("activating capsule");
            repair_capsule(&mut commands, capsule);
            game_events.send(GameEvent::PlaySound(sounds::BOMB));
        }
    }
}
