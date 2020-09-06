
use crate::components::{Bomb, Destroyable, Int2Ops, MovingDir, Position, Teleport, Undestroyable};
use crate::game_events::{GameEvent, GameEvents};
use bevy::prelude::*;
use std::collections::HashSet;
use crate::entities::create_explosion;
use crate::levels::LevelInfo;
use crate::sounds;

pub fn process_damage(
    commands: &mut Commands,
    events: &mut ResMut<GameEvents>,
    position: Position,
    is_bomb: bool,
    items: &mut Query<Without<Undestroyable, (Entity, &Position)>>,
    bombs: &Query<&Bomb>,
    destroyable: &Query<&Destroyable>,
    despawned: &mut HashSet<Entity>,
) {
    for (entity, pos) in &mut items.iter() {
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
            if !despawned.contains(&entity) && (destroyable.get::<Destroyable>(entity).is_ok() || is_bomb) {
                despawned.insert(entity);
                commands.despawn(entity);
                create_explosion(commands).with(*pos);
                events.send(GameEvent::PlaySound(if is_bomb {sounds::BOMB} else {sounds::BURN}));
            }
        }
    }
}

pub fn teleport_dest_position(
    level_info: &LevelInfo,
    occupied: &HashSet<Position>,
    teleport_entity: Entity,
    direction: MovingDir,
    teleports: &mut Query<(&Teleport, &Position)>,
) -> Option<Position> {
    let teleport = *teleports.get::<Teleport>(teleport_entity).unwrap();
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
            if !occupied.contains(&dest_robbo_pos) && !level_info.is_occupied(&dest_robbo_pos) {
                return Some(dest_robbo_pos);
            }
            dir = if cc {
                dir.rotate_counter_clockwise()
            } else {
                dir.rotate_clockwise()
            }
        }
    }
    None
}
