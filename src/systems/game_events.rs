use crate::components::{Position, Robbo, Teleport, Undestroyable, Usable};
use crate::entities::{create_robbo, create_small_explosion, spawn_robbo};
use crate::frame_cnt::FrameCnt;
use crate::game_events::GameEvent;
use crate::inventory::Inventory;
use crate::levels::{create_level, LevelInfo, LevelSet};
use crate::sounds;
use crate::systems::utils::teleport_dest_position;
use crate::Opts;

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

pub fn game_event_system(
    mut commands: Commands,
    mut state: Local<State>,
    (
        frame_cnt,
        mut game_events,
        mut inventory,
        mut level_info,
        opt,
        level_sets,
        audio_output,
        asset_server,
    ): (
        Res<FrameCnt>,
        ResMut<Events<GameEvent>>,
        ResMut<Inventory>,
        ResMut<LevelInfo>,
        Res<Opts>,
        Res<Assets<LevelSet>>,
        Res<AudioOutput>,
        Res<AssetServer>,
    ),
    items: Query<Without<Undestroyable, (Entity, &Position)>>,
    mut teleports: Query<(&Teleport, &Position)>,
    mut robbo: Query<With<Robbo, (Entity, &mut Position)>>,
    mut all_positions: Query<(Entity, &Position)>,
) {
    if !frame_cnt.is_keyframe() {
        return;
    }
    let mut despawned = HashSet::new();

    let events: Vec<GameEvent> = state.events.iter(&game_events).map(|e| *e).collect();
    // separate step for ReloadLevel event
    // because we want to process it first
    // to make sure no despawns are queued
    for event in &events {
        if let GameEvent::ReloadLevel(k) = *event {
            if let Some(level_set) = level_sets.get(&level_info.level_set_handle) {
                let level = level_info.inc_current_level(k, level_set);
                println!("Level: {:?}", level_info.current_level);
                level_info.missing_robbo_ticks = 0;
                level_info.screws = level.screw_count;
                level_info.width = level.height;
                level_info.height = level.width;
                create_level(&mut commands, &mut all_positions, level, &mut level_info);
                *inventory = Inventory::default();
                inventory.show();
                game_events.send(GameEvent::PlaySound(sounds::SPAWN));
                return;
            }
        }
    }

    // deduplicate sounds
    let mut sounds_to_play: HashSet<Handle<AudioSource>> = HashSet::new();

    for event in &events {
        match *event {
            GameEvent::PlaySound(path) => {
                sounds_to_play.insert(asset_server.get_handle(path).unwrap());
            }
            GameEvent::SpawnRobbo(pos) => {
                create_robbo(&mut commands).with(pos);
            }
            GameEvent::KillRobbo => {
                for (_, pos) in &mut robbo.iter() {
                    game_events.send(GameEvent::Damage(*pos, false));
                }
            }
            GameEvent::Use(entity, direction) => {
                let usable = items.get::<Usable>(entity).unwrap();
                match *usable {
                    Usable::Door => {
                        if inventory.keys > 0 {
                            inventory.keys -= 1;
                            commands.despawn(entity);
                            despawned.insert(entity);
                            game_events.send(GameEvent::PlaySound(sounds::DOOR));
                        }
                    }
                    Usable::Capsule => game_events.send(GameEvent::ReloadLevel(1)),
                    Usable::Teleport => {
                        let occupied: HashSet<_> =
                            all_positions.iter().iter().map(|(_, pos)| *pos).collect();
                        let dest_robbo_pos = teleport_dest_position(
                            &level_info,
                            &occupied,
                            entity,
                            direction,
                            &mut teleports,
                        );
                        if let Some(dest_robbo_pos) = dest_robbo_pos {
                            for (robbo_entity, robbo_pos) in &mut robbo.iter() {
                                commands.despawn(robbo_entity);
                                create_small_explosion(&mut commands).with(*robbo_pos);
                                spawn_robbo(&mut commands, dest_robbo_pos);
                                game_events.send(GameEvent::PlaySound(sounds::TELEPORT));
                                return;
                            }
                        }
                    }
                }
            }
            _ => (),
        }
    }
    for handle in &sounds_to_play {
        if !opt.no_audio {
            audio_output.play(*handle);
        }
    }
}
