
use crate::components::{Bomb, Destroyable, Kind, Position, Teleport, Wall, Robbo};
use crate::frame_cnt::FrameCnt;
use crate::game_events::{GameEvent, GameEvents};
use crate::inventory::Inventory;
use crate::systems::utils::{process_damage, teleport_dest_position};
use bevy::prelude::*;
use std::collections::HashSet;


pub fn game_event_system(
    mut commands: Commands,
    (frame_cnt, mut events, mut inventory): (Res<FrameCnt>, ResMut<GameEvents>, ResMut<Inventory>),
    mut items: Query<Without<Wall, (Entity, &Position, &Kind)>>,
    bombs: Query<&Bomb>,
    destroyable: Query<&Destroyable>,
    mut teleports: Query<(&Teleport, &Position)>,
    mut robbo: Query<With<Robbo, &mut Position>>,
    mut all_positions: Query<&Position>,
) {
    if !frame_cnt.do_it() {
        return;
    }

    for event in events.take().iter() {
        match *event {
            GameEvent::Damage(position, is_bomb) => {
                process_damage(&mut commands, &mut events, position, is_bomb, &mut items, &bombs, &destroyable);
            }
            GameEvent::RemoveEntity(entity) => {
                commands.despawn(entity);
            }
            GameEvent::Use(entity, direction) => {
                let kind = *items.get::<Kind>(entity).unwrap();
                match kind {
                    Kind::Door => {
                        if inventory.keys > 0 {
                            inventory.keys -= 1;
                            commands.despawn(entity);
                        }
                    }
                    Kind::Capsule => {
                        println!("Bye! Going to next level!")
                    }
                    Kind::Teleport => {
                        let occupied: HashSet<_> = all_positions.iter().iter().cloned().collect();
                        let dest_robbo_pos = teleport_dest_position(&occupied, entity, direction, &mut teleports);
                        if let Some(dest_robbo_pos) = dest_robbo_pos {
                            for mut robbo_pos in &mut robbo.iter() {
                                *robbo_pos = dest_robbo_pos;
                                return;
                            }
                        }
                    }
                    _ => (),
                }
            }
        }
    }
}

