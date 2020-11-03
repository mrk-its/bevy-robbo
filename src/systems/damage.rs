use crate::components::prelude::*;
use crate::entities::{create_explosion, spawn_random};
use crate::frame_cnt::FrameCnt;
use crate::resources::DamageMap;
use crate::plugins::audio::Sound;

use crate::levels::LevelInfo;
use bevy::prelude::*;
use std::collections::HashSet;

pub fn damage_system(
    mut commands: Commands,
    frame_cnt: Res<FrameCnt>,
    mut sounds: ResMut<Events<Sound>>,
    robbo: Query<With<Robbo, (Entity, &Position)>>,
    deadly: Query<With<Deadly, (Entity, &Position)>>,
    all: Query<&Position>,
) {
    if !frame_cnt.is_keyframe() {
        return;
    }
    for (robbo_entity, robbo_pos) in robbo.iter() {
        for (entity, deadly_pos) in deadly.iter() {
            let dx = robbo_pos.x() - deadly_pos.x();
            let dy = robbo_pos.y() - deadly_pos.y();

            if (dx.abs(), dy.abs()) == (0, 1) || (dx.abs(), dy.abs()) == (1, 0) {
                if let Ok(magnet) = all.get_component::<Magnet>(entity) {
                    if (dx, dy) != magnet.as_tuple() {
                        // magnet is deadly on the front only
                        continue;
                    }
                }
                commands.despawn(robbo_entity);
                create_explosion(&mut commands).with(*robbo_pos);
                sounds.send(Sound::BURN);
                return;
            }
        }
    }
}

pub fn process_damage(
    mut commands: Commands,
    frame_cnt: Res<FrameCnt>,
    level_info: Res<LevelInfo>,
    mut sounds: ResMut<Events<Sound>>,
    mut damage_map: ResMut<DamageMap>,
    items: Query<Without<Undestroyable, (Entity, &Position)>>,
    mut bombs: Query<&mut Bomb>,
    destroyable: Query<&Destroyable>,
) {
    if !frame_cnt.is_keyframe() {
        return;
    }
    let damage = damage_map.take();
    if damage.is_empty() {
        return;
    }
    let mut damaged_entities: HashSet<Position> = HashSet::new();
    for (entity, pos) in items.iter() {
        if let Some(is_bomb_damage) = damage.get(pos) {
            damaged_entities.insert(*pos);
            let mut do_damage =
                |kx, ky| damage_map.do_damage(&pos.add(&MovingDir::new(kx, ky)), true);
            let is_bomb_entity = if let Ok(mut bomb) = bombs.get_mut(entity) {
                if !bomb.0 {
                    bomb.0 = true;
                    do_damage(0, 0);
                    do_damage(1, 1);
                    do_damage(-1, -1);
                    do_damage(1, -1);
                    do_damage(-1, 1);
                    sounds.send(Sound::BOMB);
                    continue;
                } else {
                    do_damage(0, 1);
                    do_damage(0, -1);
                    do_damage(1, 0);
                    do_damage(-1, 0);
                }
                true
            } else {
                false
            };
            if destroyable.get_component::<Destroyable>(entity).is_ok() || *is_bomb_damage {
                commands.despawn(entity);

                if destroyable.get_component::<QuestionMark>(entity).is_ok() {
                    spawn_random(&mut commands, *pos).with(*pos);
                } else {
                    create_explosion(&mut commands).with(*pos);
                }
                if !is_bomb_entity && !is_bomb_damage {
                    sounds.send(Sound::BURN);
                }
            }
        }
    }
    for (&pos, &is_bomb_damage) in damage.iter() {
        if is_bomb_damage && !damaged_entities.contains(&pos) && !level_info.is_occupied(&pos) {
            create_explosion(&mut commands).with(pos);
        }
    }
}
