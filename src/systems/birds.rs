use crate::components::prelude::*;
use crate::frame_cnt::FrameCnt;
use crate::levels::LevelInfo;
use crate::resources::DamageMap;
use bevy::prelude::*;
// use std::collections::HashMap;

// fn is_free(pos: &Position, occupied: &HashMap<Position, Entity>, level_info: &LevelInfo) -> bool {
//     !occupied.contains_key(pos) && !level_info.is_occupied(&pos)
// }

pub fn move_bird(
    frame_cnt: Res<FrameCnt>,
    level_info: Res<LevelInfo>,
    damage_map: Res<DamageMap>,
    mut queries: QuerySet<(
        Query<Without<Wall, (&Position, Entity)>>,
        Query<With<MovingBetweenWalls, (&mut Position, &mut MovingDir)>>,
    )>,
) {
    if !frame_cnt.is_keyframe() {
        return;
    }
    let mut occupied = level_info.get_occupied(queries.q0());
    for (mut position, mut dir) in queries.q1_mut().iter_mut() {
        if damage_map.is_damaged(&*position) {
            continue;
        }
        let new_pos = position.add(&*dir);
        if occupied.is_occupied(&new_pos) {
            *dir = dir.neg()
        } else {
            occupied.mv(&*position, &new_pos);
            *position = new_pos;
        }
    }
}
