use crate::components::prelude::*;
use crate::frame_cnt::FrameCnt;
use crate::game_events::GameEvent;
use crate::inventory::Inventory;
use crate::levels::LevelInfo;
use crate::plugins::audio::Sound;
use bevy::prelude::*;
use std::collections::{HashMap, HashSet};

pub fn move_robbo(
    commands: &mut Commands,
    (mut inventory, mut events, mut sounds, frame_cnt, level_info): (
        ResMut<Inventory>,
        ResMut<Events<GameEvent>>,
        ResMut<Events<Sound>>,
        Res<FrameCnt>,
        Res<LevelInfo>,
    ),
    mut robbo: Query<(&Robbo, &mut Position, &MovingDir)>,
    mut all: Query<(&mut Position, Entity), (Without<Wall>, Without<Robbo>)>,
    collectables: Query<&Collectable>,
    moveable: Query<&Moveable>,
    usable: Query<&Usable>,
) {
    if !frame_cnt.is_keyframe() {
        return;
    }
    for (_, mut position, dir) in robbo.iter_mut() {
        let mut occupied = HashSet::new();
        let mut entities = HashMap::new();
        for (pos, entity) in all.iter_mut() {
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
                sounds.send(Sound::WALK);
                return;
            } else {
                if let Some(&entity) = entities.get(&new_pos) {
                    if let Ok(collectable) = collectables.get_component::<Collectable>(entity) {
                        inventory.collect(*collectable, &mut sounds);
                        commands.despawn(entity);
                        *position = new_pos;
                        sounds.send(Sound::WALK);
                        return;
                    } else if moveable.get_component::<Moveable>(entity).is_ok() && is_free(&new_pos2) {
                        // investigate why I cannot do all.get_mut<MovingDir>
                        // when &mut Position is replaced with &Position in query
                        let x = all.get_component_mut::<Position>(entity);
                        if let Ok(mut pos) = x {
                            *pos = new_pos2;
                            *position = new_pos;
                            let is_pushbox = all.get_component::<PushBox>(entity).is_ok();
                            if let Ok(mut mdir) = all.get_component_mut::<MovingDir>(entity) {
                                if is_pushbox {
                                    *mdir = *dir
                                }
                            }
                            sounds.send(Sound::WALK);
                            return;
                        }
                    } else if usable.get_component::<Usable>(entity).is_ok() {
                        events.send(GameEvent::Use(entity, new_pos, *dir));
                        return;
                    }
                }
            }
        }
    }
}
