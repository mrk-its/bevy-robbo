use crate::components::prelude::*;
use crate::frame_cnt::FrameCnt;
use crate::game_events::GameEvent;
use crate::inventory::Inventory;
use crate::levels::LevelInfo;
use crate::sounds;
use bevy::prelude::*;
use std::collections::{HashMap, HashSet};

pub fn move_robbo(
    mut commands: Commands,
    (mut inventory, mut events, frame_cnt, level_info): (
        ResMut<Inventory>,
        ResMut<Events<GameEvent>>,
        Res<FrameCnt>,
        Res<LevelInfo>,
    ),
    mut robbo: Query<(&Robbo, &mut Position, &MovingDir)>,
    mut all: Query<Without<Wall, Without<Robbo, (&mut Position, Entity)>>>,
) {
    if !frame_cnt.is_keyframe() {
        return;
    }
    for (_, mut position, dir) in &mut robbo.iter() {
        let mut occupied = HashSet::new();
        let mut entities = HashMap::new();
        for (pos, entity) in &mut all.iter() {
            occupied.insert(*pos);
            entities.insert(*pos, entity);
        }
        let is_free = |pos: &Position| !occupied.contains(pos) && !level_info.is_occupied(pos);

        let dirs = [MovingDir::new(dir.x(), 0), MovingDir::new(0, dir.y())];
        for dir in dirs.iter() {
            if *dir == MovingDir::zero() {
                continue;
            }
            let new_pos = position.add(dir);
            let new_pos2 = new_pos.add(dir);
            if is_free(&new_pos) {
                *position = new_pos;
                events.send(GameEvent::PlaySound(sounds::WALK));
                return;
            } else {
                if let Some(&entity) = entities.get(&new_pos) {
                    if let Ok(collectable) = all.get::<Collectable>(entity) {
                        inventory.collect(*collectable, &mut events);
                        commands.despawn(entity);
                        *position = new_pos;
                        events.send(GameEvent::PlaySound(sounds::WALK));
                        return;
                    } else if all.get::<Moveable>(entity).is_ok() && is_free(&new_pos2) {
                        // investigate why I cannot do all.get_mut<MovingDir>
                        // when &mut Position is replaced with &Position in query
                        let x = all.get_mut::<Position>(entity);
                        if let Ok(mut pos) = x {
                            *pos = new_pos2;
                            *position = new_pos;
                            if let Ok(mut mdir) = all.get_mut::<MovingDir>(entity) {
                                if let Ok(_) = all.get::<PushBox>(entity) {
                                    *mdir = *dir
                                }
                            }
                            events.send(GameEvent::PlaySound(sounds::WALK));
                            return;
                        }
                    } else if all.get::<Usable>(entity).is_ok() {
                        events.send(GameEvent::Use(entity, *dir));
                        return;
                    }
                }
            }
        }
    }
}
