use crate::components::{Bomb, Destroyable, Int2Ops, Kind, MovingDir, Position, Teleport, Wall, Robbo};
use crate::frame_cnt::FrameCnt;
use crate::game_events::{GameEvent, GameEvents};
use crate::inventory::Inventory;
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
                for (entity, pos, _) in &mut items.iter() {
                    if position == *pos {
                        if bombs.get::<Bomb>(entity).is_ok() {
                            for ky in -1..=1 {
                                for kx in -1..=1 {
                                    if kx != 0 || ky != 0 {
                                        let damage_pos = pos.add(&MovingDir::new(kx, ky));
                                        events.send(GameEvent::Damage(damage_pos, true));
                                    }
                                }
                            }
                        }
                        if destroyable.get::<Destroyable>(entity).is_ok() || is_bomb {
                            commands.despawn(entity);
                        }
                    }
                }
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

                        let teleport = *teleports.get::<Teleport>(entity).unwrap();
                        let Teleport(group, position_in_group) = teleport;
                        let mut t_query = teleports.iter();
                        let mut teleports_group: Vec<_> =
                            t_query.iter().filter(|t| t.0.0 == group).collect();
                        teleports_group.sort_by_key(|t| t.0.1);
                        let len = teleports_group.len();
                        let index = teleports_group
                            .iter()
                            .enumerate()
                            .find(|(_, v)| (v.0.1 % len) == (position_in_group % len))
                            .map(|(i, _)| i)
                            .unwrap();
                        teleports_group.rotate_left(index + 1);
                        let dest_teleport_positions = teleports_group
                            .iter()
                            .map(|t| t.1)
                            .collect::<Vec<_>>();
                        for teleport_pos in dest_teleport_positions {
                            let mut dir = direction;
                            let cc = dir.x() != 0; // hack for level 16
                            for _ in 0..4 {
                                let dest_robbo_pos = teleport_pos.add(&dir);
                                if !occupied.contains(&dest_robbo_pos) {
                                    // teleport robbo to dest_robbo_pos
                                    for mut robbo_pos in &mut robbo.iter() {
                                        *robbo_pos = dest_robbo_pos;
                                        return;
                                    }
                                }
                                dir = if cc {
                                    dir.rotate_counter_clockwise()
                                } else {
                                    dir.rotate_clockwise()
                                }
                            }
                        }
                    }
                    _ => (),
                }
            }
        }
    }
}
