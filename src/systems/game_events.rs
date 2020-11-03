use crate::components::{Position, Robbo, Teleport, Usable};
use crate::entities::*;
use crate::frame_cnt::FrameCnt;
use crate::game_events::GameEvent;
use crate::inventory::Inventory;
use crate::levels::{create_level, LevelInfo, LevelSet};
use crate::plugins::audio::Sound;
use crate::resources::DamageMap;
use crate::systems::utils::teleport_dest_position;
use rand::random;

use bevy::prelude::*;
use std::collections::HashSet;

#[derive(Default)]
pub struct State {
    pub events: EventReader<GameEvent>,
}

pub fn update_game_events(frame_cnt: Res<FrameCnt>, events: ResMut<Events<GameEvent>>) {
    if frame_cnt.is_keyframe() {
        Events::<GameEvent>::update_system(events)
    }
}

#[derive(Default)]
pub struct ReloadLevelState {
    pub events: EventReader<GameEvent>,
}

pub fn reload_level_system(
    mut commands: Commands,
    mut state: Local<ReloadLevelState>,
    game_events: ResMut<Events<GameEvent>>,
    frame_cnt: Res<FrameCnt>,
    level_sets: Res<Assets<LevelSet>>,
    mut level_info: ResMut<LevelInfo>,
    mut inventory: ResMut<Inventory>,
    mut all_positions: Query<(Entity, &Position)>,
) {
    if !frame_cnt.is_keyframe() {
        return;
    }
    for event in state.events.iter(&game_events) {
        if let GameEvent::ReloadLevel(k) = *event {
            info!("ReloadLevel({})", k);
            if let Some(level_set) = level_sets.get(&level_info.level_set_handle) {
                let level = level_info.inc_current_level(k, level_set);
                level_info.missing_robbo_ticks = 0;
                level_info.screws = level.screw_count;
                level_info.width = level.height;
                level_info.height = level.width;
                create_level(&mut commands, &mut all_positions, level, &mut level_info);
                *inventory = Inventory::default();
                return;
            }
        }
    }
}

pub fn game_event_system(
    mut commands: Commands,
    mut state: Local<State>,
    (
        frame_cnt,
        game_events,
        mut damage_map,
        mut sounds,
    ): (
        Res<FrameCnt>,
        ResMut<Events<GameEvent>>,
        ResMut<DamageMap>,
        ResMut<Events<Sound>>,
    ),
    mut robbo: Query<With<Robbo, (Entity, &mut Position)>>,
) {
    if !frame_cnt.is_keyframe() {
        return;
    }
    // let mut despawned = HashSet::new();

    for event in state.events.iter(&game_events) {
        log::info!("game_event: {:?}", event);
        match *event {
            GameEvent::SpawnRobbo(pos) => {
                create_robbo(&mut commands).with(pos);
                return;
            }
            GameEvent::PreSpawnRobbo(pos) => {
                sounds.send(Sound::SPAWN);
                spawn_robbo(&mut commands, pos);
            }
            GameEvent::SpawnRandom(pos) => {
                let create_item = [
                    create_small_explosion,
                    create_push_box,
                    create_screw,
                    create_ammo,
                    create_key,
                    create_bomb,
                    create_ground,
                    create_eyes,
                    create_questionmark_gun,
                    create_questionmark,
                ];
                create_item[random::<usize>() % create_item.len()](&mut commands).with(pos);
            }
            GameEvent::KillRobbo => {
                for (_, pos) in robbo.iter_mut() {
                    damage_map.do_damage(&*pos, false);
                }
            }
            _ => (),
        }
    }
}
#[derive(Default)]
pub struct UseItemState {
    pub events: EventReader<GameEvent>,
}

pub fn game_event_use_item(
    mut commands: Commands,
    mut state: Local<UseItemState>,
    (
        frame_cnt,
        game_events,
        mut inventory,
        mut sounds,
    ): (
        Res<FrameCnt>,
        ResMut<Events<GameEvent>>,
        ResMut<Inventory>,
        ResMut<Events<Sound>>,
    ),
    usable: Query<&Usable>,
    mut robbo: Query<With<Robbo, (Entity, &mut Position)>>,
) {
    if !frame_cnt.is_keyframe() {
        return;
    }
    let mut despawned = HashSet::new();
    for event in state.events.iter(&game_events) {
        match *event {
            GameEvent::Use(entity, pos, _) => {
                let usable = usable.get_component::<Usable>(entity).unwrap();
                match *usable {
                    Usable::Door => {
                        if inventory.keys > 0 {
                            inventory.keys -= 1;
                            commands.despawn(entity);
                            despawned.insert(entity);
                            sounds.send(Sound::DOOR);
                        }
                    }
                    Usable::Capsule => {
                        for (robbo_entity, _) in robbo.iter_mut() {
                            commands.despawn(robbo_entity);
                            sounds.send(Sound::CAPSULE);
                            fly_away(&mut commands, pos);
                        }
                    }
                    _ => ()
                }
            }
            _ => (),
        }
    }
}
#[derive(Default)]
pub struct UseTeleportState {
    pub events: EventReader<GameEvent>,
}

pub fn game_event_use_teleport(
    mut commands: Commands,
    mut state: Local<UseTeleportState>,
    (
        frame_cnt,
        game_events,
        level_info,
        mut sounds,
    ): (
        Res<FrameCnt>,
        ResMut<Events<GameEvent>>,
        Res<LevelInfo>,
        ResMut<Events<Sound>>,
    ),
    usable: Query<&Usable>,
    mut queries: QuerySet<(
        Query<(&Teleport, &Position)>,
        Query<With<Robbo, (Entity, &mut Position)>>,
        Query<(Entity, &Position)>,
    )>
) {
    if !frame_cnt.is_keyframe() {
        return;
    }
    for event in state.events.iter(&game_events) {
        match *event {
            GameEvent::Use(entity, _, direction) => {
                let usable = usable.get_component::<Usable>(entity).unwrap();
                match *usable {
                    Usable::Teleport => {
                        let occupied: HashSet<_> =
                            queries.q2().iter().map(|(_, pos)| *pos).collect();
                        let dest_robbo_pos = teleport_dest_position(
                            &level_info,
                            &occupied,
                            entity,
                            direction,
                            queries.q0_mut(),
                        );
                        if let Some(dest_robbo_pos) = dest_robbo_pos {
                            for (robbo_entity, robbo_pos) in queries.q1_mut().iter_mut() {
                                commands.despawn(robbo_entity);
                                create_small_explosion(&mut commands).with(*robbo_pos);
                                spawn_robbo(&mut commands, dest_robbo_pos);
                                sounds.send(Sound::TELEPORT);
                                return;
                            }
                        }
                    },
                    _ => (),
                }
            }
            _ => (),
        }
    }
}
