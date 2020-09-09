use crate::components::prelude::*;
use crate::entities::create_small_explosion;
use crate::frame_cnt::FrameCnt;
use crate::levels::LevelInfo;
use crate::resources::DamageMap;
use bevy::prelude::*;

pub fn move_bullet(
    mut commands: Commands,
    frame_cnt: Res<FrameCnt>,
    level_info: ResMut<LevelInfo>,
    mut damage_map: ResMut<DamageMap>,
    mut bullets: Query<With<Bullet, (Entity, &mut Position, &mut MovingDir)>>,
    mut all_query: Query<Without<Wall, (&Position, Entity)>>,
) {
    if !frame_cnt.is_keyframe() {
        return;
    }
    let mut occupancy = level_info.get_occupancy(&mut all_query);
    for (entity, mut position, mut dir) in &mut bullets.iter() {
        let new_pos = position.add(&*dir);
        if occupancy.is_occupied(&new_pos) {
            *dir = MovingDir::zero();
            commands.despawn(entity);
            create_small_explosion(&mut commands).with(*position);
            damage_map.do_damage(&new_pos, false);
        } else {
            occupancy.mv(&*position, &new_pos);
            *position = new_pos;
        }
    }
}
