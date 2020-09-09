use crate::components::prelude::*;
use crate::entities::{create_laser_tail, create_small_explosion};
use crate::frame_cnt::FrameCnt;
use crate::levels::LevelInfo;
use crate::resources::DamageMap;
use bevy::prelude::*;

pub fn move_laser_head(
    mut commands: Commands,
    frame_cnt: Res<FrameCnt>,
    level_info: ResMut<LevelInfo>,
    mut damage_map: ResMut<DamageMap>,
    mut laser_heads: Query<(&mut LaserHead, &mut Position, &mut MovingDir)>,
    mut all_query: Query<Without<Wall, (&Position, Entity)>>,
) {
    if !frame_cnt.is_keyframe() {
        return;
    }
    let mut occupancy = level_info.get_occupancy(&mut all_query);
    for (mut laser_head, mut position, mut dir) in &mut laser_heads.iter() {
        let new_pos = position.add(&*dir);
        let is_laser_tail_in_front = occupancy.get_entity(&new_pos)
            .map(|entity| all_query.get::<LaserTail>(*entity).is_ok())
            .unwrap_or_default();
        if occupancy.is_free(&new_pos) {
            let old_pos = *position;
            occupancy.mv(&*position, &new_pos);
            *position = new_pos;
            create_laser_tail(&mut commands, &*dir).with(old_pos);
            occupancy.put_entity(&old_pos, commands.current_entity().unwrap());
        } else if laser_head.is_moving_back && is_laser_tail_in_front {
            let entity = occupancy.remove(&new_pos).unwrap();
            commands.despawn(entity);
            occupancy.mv(&*position, &new_pos);
            *position = new_pos;
        } else if !laser_head.is_moving_back {
            laser_head.is_moving_back = true;
            *dir = dir.neg();
            damage_map.do_damage(&new_pos, false);
        } else {
            *dir = MovingDir::zero();
            let entity = occupancy.remove(&position).unwrap();
            commands.despawn(entity);
            create_small_explosion(&mut commands).with(*position);
        }
    }
}
