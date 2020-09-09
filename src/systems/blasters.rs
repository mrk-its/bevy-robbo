use crate::components::prelude::*;
use crate::entities::create_blaster_tail;
use crate::frame_cnt::FrameCnt;
use crate::levels::LevelInfo;
use bevy::prelude::*;

pub fn move_blaster_head(
    mut commands: Commands,
    frame_cnt: Res<FrameCnt>,
    level_info: Res<LevelInfo>,
    mut items: Query<With<BlasterHead, (Entity, &mut Position, &MovingDir)>>,
    mut all_query: Query<Without<Wall, (&Position, Entity)>>,
    destroyable: Query<&Destroyable>,
) {
    if !frame_cnt.is_keyframe() {
        return;
    }
    let mut occupancy = level_info.get_occupancy(&mut all_query);

    for (entity, mut position, dir) in &mut items.iter() {
        let new_pos = position.add(&*dir);
        let old_pos = *position;
        let is_wall = level_info.is_occupied(&new_pos);
        if is_wall {
            commands.despawn(entity);
        } else if let Some(&entity) = occupancy.get_entity(&new_pos) {
            let is_destroyable = destroyable.get::<Destroyable>(entity).is_ok();
            if is_destroyable {
                commands.despawn(entity);
                occupancy.remove(&new_pos);
                occupancy.mv(&*position, &new_pos);
                *position = new_pos;
            } else {
                let entity = occupancy.get_entity(&position).unwrap();
                commands.despawn(*entity);
            }
        } else {
            occupancy.mv(&*position, &new_pos);
            *position = new_pos;
        }
        create_blaster_tail(&mut commands).with(old_pos);
    }
}
