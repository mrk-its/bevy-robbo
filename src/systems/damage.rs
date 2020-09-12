use crate::components::prelude::*;
use crate::entities::{create_explosion, spawn_random};
use crate::frame_cnt::FrameCnt;
use crate::game_events::GameEvent;
use crate::resources::DamageMap;
use crate::sounds;

use crate::levels::LevelInfo;
use std::collections::HashSet;
use bevy::prelude::*;

pub fn damage_system(
    mut commands: Commands,
    frame_cnt: Res<FrameCnt>,
    mut events: ResMut<Events<GameEvent>>,
    mut robbo: Query<With<Robbo, (Entity, &Position)>>,
    mut deadly: Query<With<Deadly, (Entity, &Position)>>,
    all: Query<&Position>,
) {
    if !frame_cnt.is_keyframe() {
        return;
    }
    for (robbo_entity, robbo_pos) in &mut robbo.iter() {
        for (entity, deadly_pos) in &mut deadly.iter() {
            let dx = robbo_pos.x() - deadly_pos.x();
            let dy = robbo_pos.y() - deadly_pos.y();

            if (dx.abs(), dy.abs()) == (0, 1) || (dx.abs(), dy.abs()) == (1, 0) {
                if let Ok(magnet) = all.get::<Magnet>(entity) {
                    if (dx, dy) != magnet.as_tuple() {
                        // magnet is deadly on the front only
                        continue;
                    }
                }
                commands.despawn(robbo_entity);
                create_explosion(&mut commands).with(*robbo_pos);
                events.send(GameEvent::PlaySound(sounds::BURN));
                return;
            }
        }
    }
}

pub fn process_damage(
    mut commands: Commands,
    frame_cnt: Res<FrameCnt>,
    level_info: Res<LevelInfo>,
    mut events: ResMut<Events<GameEvent>>,
    mut damage_map: ResMut<DamageMap>,
    mut items: Query<Without<Undestroyable, (Entity, &Position)>>,
    bombs: Query<&mut Bomb>,
    destroyable: Query<&Destroyable>,
) {
    if !frame_cnt.is_keyframe() {
        return;
    }
    let damage = damage_map.take();
    let mut damaged_entities: HashSet<Position> = HashSet::new();
    for (entity, pos) in &mut items.iter() {
        if let Some(is_bomb_damage) = damage.get(pos) {
            damaged_entities.insert(*pos);
            let mut do_damage =
                |kx, ky| damage_map.do_damage(&pos.add(&MovingDir::new(kx, ky)), true);
            let is_bomb_entity = if let Ok(mut bomb) = bombs.entity(entity) {
                if let Some(mut bomb) = bomb.get() {
                    if !bomb.0 {
                        bomb.0 = true;
                        do_damage(0, 0);
                        do_damage(1, 1);
                        do_damage(-1, -1);
                        do_damage(1, -1);
                        do_damage(-1, 1);
                        events.send(GameEvent::PlaySound(sounds::BOMB));
                        continue;
                    } else {
                        do_damage(0, 1);
                        do_damage(0, -1);
                        do_damage(1, 0);
                        do_damage(-1, 0);
                    }
                }
                true
            } else {
                false
            };
            if destroyable.get::<Destroyable>(entity).is_ok() || *is_bomb_damage {
                commands.despawn(entity);

                if destroyable.get::<QuestionMark>(entity).is_ok() {
                    spawn_random(&mut commands, *pos).with(*pos);
                } else {
                    create_explosion(&mut commands).with(*pos);
                }
                if !is_bomb_entity && !is_bomb_damage {
                    events.send(GameEvent::PlaySound(sounds::BURN));
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
