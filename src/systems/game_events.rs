use crate::components::{Bomb, Destroyable, Position, Usable, Robbo, Teleport, Undestroyable};
use crate::frame_cnt::FrameCnt;
use crate::game_events::{GameEvent, GameEvents};
use crate::inventory::Inventory;
use crate::levels::{create_level, Level};
use crate::systems::utils::{process_damage, teleport_dest_position};
use bevy::prelude::*;
use std::collections::HashSet;
use bevy::render::pass::ClearColor;

pub fn game_event_system(
    mut commands: Commands,
    (frame_cnt, mut events, mut inventory, levels, _clear_color): (
        Res<FrameCnt>,
        ResMut<GameEvents>,
        ResMut<Inventory>,
        Res<Assets<Level>>,
        ResMut<ClearColor>,
),
    mut items: Query<Without<Undestroyable, (Entity, &Position)>>,
    bombs: Query<&Bomb>,
    destroyable: Query<&Destroyable>,
    mut teleports: Query<(&Teleport, &Position)>,
    mut robbo: Query<With<Robbo, &mut Position>>,
    mut all_positions: Query<(Entity, &Position)>,
) {
    if !frame_cnt.do_it() {
        return;
    }
    let mut despawned = HashSet::new();

    let _events = events.take();

    // separate step for ReloadLevel event
    // because we want to process it first
    // to make sure no despawns are queued
    for event in &_events {
        if let GameEvent::ReloadLevel(handle) = event {
            if let Some(level) = levels.get(&handle) {
                create_level(&mut commands, &mut all_positions, level);
                return;
            }
        }
    }

    for event in &_events {
        match *event {
            GameEvent::Damage(position, is_bomb) => {
                process_damage(
                    &mut commands,
                    &mut events,
                    position,
                    is_bomb,
                    &mut items,
                    &bombs,
                    &destroyable,
                    &mut despawned,
                );
            }
            GameEvent::RemoveEntity(entity) => {
                commands.despawn(entity);
                despawned.insert(entity);
            }
            GameEvent::Use(entity, direction) => {
                let usable = items.get::<Usable>(entity).unwrap();
                match *usable {
                    Usable::Door => {
                        if inventory.keys > 0 {
                            inventory.keys -= 1;
                            commands.despawn(entity);
                            despawned.insert(entity);
                        }
                    }
                    Usable::Capsule => println!("Bye! Going to next level!"),
                    Usable::Teleport => {
                        let occupied: HashSet<_> = all_positions.iter().iter().map(|(_, pos)| *pos).collect();
                        let dest_robbo_pos =
                            teleport_dest_position(&occupied, entity, direction, &mut teleports);
                        if let Some(dest_robbo_pos) = dest_robbo_pos {
                            for mut robbo_pos in &mut robbo.iter() {
                                *robbo_pos = dest_robbo_pos;
                                return;
                            }
                        }
                    }
                }
            },
            _ => ()
        }
    }
}
