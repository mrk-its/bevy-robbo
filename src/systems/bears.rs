use crate::components::prelude::*;
use crate::frame_cnt::FrameCnt;
use crate::levels::LevelInfo;
use crate::resources::DamageMap;
use bevy::prelude::*;

pub fn move_bear(
    frame_cnt: Res<FrameCnt>,
    level_info: Res<LevelInfo>,
    damage_map: Res<DamageMap>,
    mut queries: QuerySet<(
        Query<(&Bear, &mut Position, &mut MovingDir)>,
        Query<Without<Wall, (&Position, Entity)>>,
    )>,
) {
    if !frame_cnt.is_keyframe() {
        return;
    }
    let mut occupied = level_info.get_occupied(queries.q1());
    for (bear, mut position, mut dir) in queries.q0_mut().iter_mut() {
        if damage_map.is_damaged(&*position) {
            continue;
        }
        let r1 = |dir: MovingDir| {
            if !bear.0 {
                dir.rotate_clockwise()
            } else {
                dir.rotate_counter_clockwise()
            }
        };
        let r2 = |dir: MovingDir| {
            if !bear.0 {
                dir.rotate_counter_clockwise()
            } else {
                dir.rotate_clockwise()
            }
        };

        let new_dir = r1(*dir);
        let new_dir2 = r2(*dir);
        let new_dir3 = r2(new_dir2);
        if occupied.is_free(&(position.add(&new_dir))) {
            let new_pos = position.add(&new_dir);
            occupied.mv(&position, &new_pos);
            *position = new_pos;
            *dir = new_dir;
        } else if occupied.is_free(&(position.add(&*dir))) {
            let new_pos = position.add(&*dir);
            occupied.mv(&position, &new_pos);
            *position = new_pos;
        } else if occupied.is_free(&(position.add(&new_dir2))) {
            *dir = new_dir2
        } else if occupied.is_free(&(position.add(&new_dir3))) {
            *dir = new_dir3
        }
    }
}
