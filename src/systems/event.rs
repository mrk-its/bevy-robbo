use crate::components::{Destroyable, Position};
use crate::events::{DamageState, Event};
use bevy::prelude::*;

pub fn event_system(
    mut commands: Commands,
    mut state: Local<DamageState>,
    damage_events: Res<Events<Event>>,
    mut destroyable_items: Query<With<Destroyable, (Entity, &Position)>>,
    mut all_items: Query<(Entity, &Position)>,
) {
    // for (entity, pos, sprite) in &mut items.iter() {
    //     println!("entity at: {:?}", pos);
    // }
    // println!("#################");
    // println!("damage_system");
    for event in state.damage_reader.iter(&damage_events) {
        match event {
            Event::Damage(position) => {
                println!("damage at: {:?}", position);
                for (entity, pos) in &mut destroyable_items.iter() {
                    if position == pos {
                        println!("destroying entity at: {:?}", pos);
                        commands.despawn(entity);
                        //commands.remove_one::<Position>(entity);
                        //*translation = Translation(Vec3::new(-10000.0, -10000.0, -10000.0));
                    }
                }
            }
            Event::Remove(position) => {
                for (entity, pos) in &mut all_items.iter() {
                    if position == pos {
                        println!("destroying entity at: {:?}", pos);
                        commands.despawn(entity);
                        //commands.remove_one::<Position>(entity);
                        //*translation = Translation(Vec3::new(-10000.0, -10000.0, -10000.0));
                    }
                }
            }
            _ => (),
        }
    }
}
