use crate::components::prelude::*;
use crate::entities::{create_laser_tail, create_small_explosion};
use crate::frame_cnt::FrameCnt;
use crate::levels::LevelInfo;
use crate::resources::DamageMap;
use bevy::prelude::*;

pub fn move_laser_head(
    commands: &mut Commands,
    frame_cnt: Res<FrameCnt>,
    level_info: ResMut<LevelInfo>,
    mut damage_map: ResMut<DamageMap>,
    mut queries: QuerySet<(
        Query<(&mut LaserHead, &mut Position, &mut MovingDir)>,
        Query<(&Position, Entity), Without<Wall>>,
    )>
) {
    if !frame_cnt.is_keyframe() {
        return;
    }
    let mut occupied = level_info.get_occupied(&queries.q1());
    for (mut laser_head, mut position, mut dir) in queries.q0_mut().iter_mut() {
        let new_pos = position.add(&*dir);
        if occupied.is_free(&new_pos) {
            let old_pos = *position;
            occupied.mv(&*position, &new_pos);
            *position = new_pos;
            create_laser_tail(commands, &*dir).with(old_pos);
            occupied.put_entity(&old_pos, commands.current_entity().unwrap());
        } else if laser_head.is_moving_back && laser_head.gun_pos != new_pos {
            let entity = occupied.remove(&new_pos).unwrap();
            commands.despawn(entity);
            occupied.mv(&*position, &new_pos);
            *position = new_pos;
        } else if !laser_head.is_moving_back {
            laser_head.is_moving_back = true;
            *dir = dir.neg();
            damage_map.do_damage(&new_pos, false);
        } else {
            *dir = MovingDir::zero();
            let entity = occupied.remove(&position).unwrap();
            commands.despawn(entity);
            create_small_explosion(commands).with(*position);
        }
    }
}
