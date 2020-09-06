use crate::components::{Bomb, Destroyable, Position, Robbo, Teleport, Undestroyable, Usable};
use crate::entities::{create_robbo, create_small_explosion, spawn_robbo};
use crate::frame_cnt::FrameCnt;
use crate::game_events::{GameEvent, GameEvents};
use crate::inventory::Inventory;
use crate::levels::{create_level, LevelSet, LevelInfo};
use crate::systems::utils::{process_damage, teleport_dest_position};
use crate::sounds;
use bevy::prelude::*;
use bevy::render::pass::ClearColor;
use std::collections::HashSet;

pub fn game_event_system(
    mut commands: Commands,
    (frame_cnt, mut game_events, mut inventory, mut level_info, level_sets, _clear_color, audio_output, asset_server): (
        Res<FrameCnt>,
        ResMut<GameEvents>,
        ResMut<Inventory>,
        ResMut<LevelInfo>,
        Res<Assets<LevelSet>>,
        ResMut<ClearColor>,
        Res<AudioOutput>,
        Res<AssetServer>,
    ),
    mut items: Query<Without<Undestroyable, (Entity, &Position)>>,
    bombs: Query<&Bomb>,
    destroyable: Query<&Destroyable>,
    mut teleports: Query<(&Teleport, &Position)>,
    mut robbo: Query<With<Robbo, (Entity, &mut Position)>>,
    mut all_positions: Query<(Entity, &Position)>,
) {
    if !frame_cnt.do_it() {
        return;
    }
    let mut despawned = HashSet::new();

    let events = game_events.take();

    // separate step for ReloadLevel event
    // because we want to process it first
    // to make sure no despawns are queued
    for event in &events {
        if let GameEvent::ReloadLevel(k) = *event {
            if let Some(level_set) = level_sets.get(&level_info.level_set_handle) {
                let level = level_info.inc_current_level(k, level_set);
                println!("{:?}", *level_info);
                level_info.screws = level.screw_count;
                level_info.width = level.height;
                level_info.height = level.width;
                create_level(&mut commands, &mut all_positions, level);
                *inventory = Inventory::default();
                inventory.show();
                return;
            }

        }
    }

    for event in &events {
        match *event {
            GameEvent::PlaySound(path) =>
            {
                println!("playing {:?}", path);
                if let Some(handle) = asset_server.get_handle(path) {
                    audio_output.play(handle)
                } else {
                    println!("{:?} not found", path);
                }
            }
            GameEvent::Damage(position, is_bomb) => {
                process_damage(
                    &mut commands,
                    &mut game_events,
                    position,
                    is_bomb,
                    &mut items,
                    &bombs,
                    &destroyable,
                    &mut despawned,
                );
            }
            GameEvent::SpawnRobbo(pos) => {
                create_robbo(&mut commands).with(pos);
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
                        game_events.send(GameEvent::PlaySound(sounds::DOOR));
                    }
                    Usable::Capsule => game_events.send(GameEvent::ReloadLevel(1)),
                    Usable::Teleport => {
                        let occupied: HashSet<_> =
                            all_positions.iter().iter().map(|(_, pos)| *pos).collect();
                        let dest_robbo_pos =
                            teleport_dest_position(&level_info, &occupied, entity, direction, &mut teleports);
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
}
