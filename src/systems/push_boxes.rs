use crate::components::prelude::*;
use crate::frame_cnt::FrameCnt;
use crate::levels::LevelInfo;
use crate::resources::DamageMap;
use bevy::prelude::*;


pub fn move_pushbox(
    frame_cnt: Res<FrameCnt>,
    level_info: Res<LevelInfo>,
    mut damage_map: ResMut<DamageMap>,
    mut all_query: Query<Without<Wall, (&Position, Entity)>>,
    mut boxes: Query<With<PushBox, (&mut Position, &mut MovingDir)>>,
) {
    if !frame_cnt.is_keyframe() {
        return;
    }
    let mut occupancy = level_info.get_occupancy(&mut all_query);
    for (mut position, mut dir) in &mut boxes.iter() {
        if damage_map.is_damaged(&*position) || dir.is_empty() {
            continue;
        }
        let new_pos = position.add(&*dir);
        if occupancy.is_occupied(&new_pos) {
            damage_map.do_damage(&new_pos, false);
            *dir = MovingDir::zero();
        } else {
            occupancy.mv(&*position, &new_pos);
            *position = new_pos;
        }
    }
}

