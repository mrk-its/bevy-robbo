
use crate::components::{Int2Ops, MovingDir, Position, Teleport};
use bevy::prelude::*;
use std::collections::HashSet;
use crate::levels::LevelInfo;


pub fn teleport_dest_position(
    level_info: &LevelInfo,
    occupied: &HashSet<Position>,
    teleport_entity: Entity,
    direction: MovingDir,
    teleports: &mut Query<(&Teleport, &Position)>,
) -> Option<Position> {
    let teleport = *teleports.get_component::<Teleport>(teleport_entity).unwrap();
    let Teleport(group, position_in_group) = teleport;
    let t_query = teleports.iter();
    let mut teleports_group: Vec<_> =
        t_query.filter(|t| t.0.0 == group).collect();
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
