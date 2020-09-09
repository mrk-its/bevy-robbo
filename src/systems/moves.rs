use crate::components::prelude::*;
use crate::entities::{create_blaster_tail, create_laser_tail, create_small_explosion};
use crate::frame_cnt::FrameCnt;
use crate::levels::LevelInfo;
use crate::resources::DamageMap;
use bevy::prelude::*;
// use std::collections::HashMap;

// fn is_free(pos: &Position, occupied: &HashMap<Position, Entity>, level_info: &LevelInfo) -> bool {
//     !occupied.contains_key(pos) && !level_info.is_occupied(&pos)
// }

pub fn move_bear(
    frame_cnt: Res<FrameCnt>,
    level_info: Res<LevelInfo>,
    damage_map: Res<DamageMap>,
    mut bear_query: Query<(&Bear, &mut Position, &mut MovingDir)>,
    mut all_query: Query<Without<Wall, (&Position, Entity)>>,
) {
    if !frame_cnt.is_keyframe() {
        return;
    }
    let mut occupancy = level_info.get_occupancy(&mut all_query);
    for (bear, mut position, mut dir) in &mut bear_query.iter() {
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
        if occupancy.is_free(&(position.add(&new_dir))) {
            let new_pos = position.add(&new_dir);
            occupancy.mv(&position, &new_pos);
            *position = new_pos;
            *dir = new_dir;
        } else if occupancy.is_free(&(position.add(&*dir))) {
            let new_pos = position.add(&*dir);
            occupancy.mv(&position, &new_pos);
            *position = new_pos;
        } else if occupancy.is_free(&(position.add(&new_dir2)))
        {
            *dir = new_dir2
        } else if occupancy.is_free(&(position.add(&new_dir3)))
        {
            *dir = new_dir3
        }
    }
}

// fn move_entity(
//     position: &mut Mut<Position>,
//     new_position: &Position,
//     occupied: &mut HashMap<Position, Entity>,
// ) {
//     let entity = occupied.remove(&position).unwrap();
//     **position = *new_position;
//     occupied.insert(**position, entity);
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

pub fn move_box(
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
