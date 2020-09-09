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
    mut all_query: Query<Without<Wall, (&Position, Entity)>>,
    mut birds: Query<With<Bird, (&mut Position, &mut MovingDir)>>,
) {
    if !frame_cnt.is_keyframe() {
        return;
    }
    let mut occupancy = level_info.get_occupancy(&mut all_query);
    for (mut position, mut dir) in &mut birds.iter() {
        if damage_map.is_damaged(&*position) {
            continue;
        }
        let new_pos = position.add(&*dir);
        if occupancy.is_occupied(&new_pos) {
            *dir = dir.neg()
        } else {
            occupancy.mv(&*position, &new_pos);
            *position = new_pos;
        }
    }
}
