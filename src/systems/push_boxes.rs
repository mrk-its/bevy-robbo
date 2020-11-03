use crate::components::prelude::*;
use crate::frame_cnt::FrameCnt;
use crate::levels::LevelInfo;
use crate::resources::DamageMap;
use bevy::prelude::*;

pub fn move_pushbox(
    frame_cnt: Res<FrameCnt>,
    level_info: Res<LevelInfo>,
    mut damage_map: ResMut<DamageMap>,
    mut queries: QuerySet<(
        Query<Without<Wall, (&Position, Entity)>>,
        Query<With<PushBox, (&mut Position, &mut MovingDir)>>,
    )>
) {
    if !frame_cnt.is_keyframe() {
        return;
    }
    let mut occupied = level_info.get_occupied(queries.q0());
    for (mut position, mut dir) in queries.q1_mut().iter_mut() {
        if damage_map.is_damaged(&*position) || dir.is_empty() {
            continue;
        }
        let new_pos = position.add(&*dir);
        if occupied.is_occupied(&new_pos) {
            damage_map.do_damage(&new_pos, false);
            *dir = MovingDir::zero();
        } else {
            occupied.mv(&*position, &new_pos);
            *position = new_pos;
        }
    }
}
