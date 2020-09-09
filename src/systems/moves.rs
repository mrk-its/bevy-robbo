use crate::components::prelude::*;
use crate::entities::{create_blaster_tail, create_laser_tail, create_small_explosion};
use crate::frame_cnt::FrameCnt;
use crate::levels::LevelInfo;
use bevy::prelude::*;
use std::collections::{HashMap, HashSet};

fn is_free(pos: &Position, occupied: &HashMap<Position, Entity>, level_info: &LevelInfo) -> bool {
    !occupied.contains_key(pos) && !level_info.is_occupied(&pos)
}

fn is_free2(pos: &Position, occupied: &HashSet<Position>, level_info: &LevelInfo) -> bool {
    !occupied.contains(pos) && !level_info.is_occupied(&pos)
}

pub fn move_bear(
    frame_cnt: Res<FrameCnt>,
    level_info: Res<LevelInfo>,
    mut bear_query: Query<(&Bear, &mut Position, &mut MovingDir)>,
    mut all_query: Query<Without<Wall, &Position>>,
) {
    if !frame_cnt.is_keyframe() {
        return;
    }
    let mut occupied: HashSet<Position> = all_query.iter().into_iter().cloned().collect();

    for (bear, mut position, mut dir) in &mut bear_query.iter() {
        if level_info.is_damaged(&*position) {
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
        if is_free2(&(position.add(&new_dir)), &occupied, &level_info) {
            *dir = new_dir;
            occupied.remove(&position);
            *position = position.add(&new_dir);
            occupied.insert(*position);
        } else if !occupied.contains(&(position.add(&*dir)))
            && !level_info.is_occupied(&(position.add(&*dir)))
        {
            occupied.remove(&position);
            *position = position.add(&*dir);
            occupied.insert(*position);
        } else if !occupied.contains(&(position.add(&new_dir2)))
            && !level_info.is_occupied(&(position.add(&new_dir2)))
        {
            *dir = new_dir2
        } else if !occupied.contains(&(position.add(&new_dir3)))
            && !level_info.is_occupied(&(position.add(&new_dir3)))
        {
            *dir = new_dir3
        }
    }
}

fn move_entity(
    position: &mut Mut<Position>,
    new_position: &Position,
    occupied: &mut HashMap<Position, Entity>,
) {
    let entity = occupied.remove(&position).unwrap();
    **position = *new_position;
    occupied.insert(**position, entity);
}

fn move_entity2(
    position: &mut Position,
    new_position: &Position,
    occupied: &mut HashSet<Position>,
) {
    occupied.remove(&position);
    *position = *new_position;
    occupied.insert(*position);
}

pub fn move_bird(
    frame_cnt: Res<FrameCnt>,
    level_info: Res<LevelInfo>,
    mut all_query: Query<Without<Wall, &Position>>,
    mut birds: Query<With<Bird, (&mut Position, &mut MovingDir)>>,
) {
    if !frame_cnt.is_keyframe() {
        return;
    }
    let mut occupied: HashSet<Position> = all_query.iter().into_iter().cloned().collect();
    for (mut position, mut dir) in &mut birds.iter() {
        if level_info.is_damaged(&*position) {
            continue;
        }
        let new_pos = position.add(&*dir);
        if !is_free2(&new_pos, &occupied, &level_info) {
            *dir = dir.neg()
        } else {
            move_entity2(&mut *position, &new_pos, &mut occupied);
        }
    }
}

pub fn move_box(
    frame_cnt: Res<FrameCnt>,
    mut level_info: ResMut<LevelInfo>,
    mut all_query: Query<Without<Wall, &Position>>,
    mut boxes: Query<With<PushBox, (&mut Position, &mut MovingDir)>>,
) {
    if !frame_cnt.is_keyframe() {
        return;
    }
    let mut occupied: HashSet<Position> = all_query.iter().into_iter().cloned().collect();
    for (mut position, mut dir) in &mut boxes.iter() {
        if level_info.is_damaged(&*position) || dir.is_empty() {
            continue;
        }
        let new_pos = position.add(&*dir);
        if !is_free2(&new_pos, &occupied, &level_info) {
            level_info.do_damage(&new_pos, false);
            *dir = MovingDir::zero();
        } else {
            move_entity2(&mut position, &new_pos, &mut occupied);
        }
    }
}

pub fn move_bullet(
    mut commands: Commands,
    frame_cnt: Res<FrameCnt>,
    mut level_info: ResMut<LevelInfo>,
    mut bullets: Query<With<Bullet, (Entity, &mut Position, &mut MovingDir)>>,
    mut all_query: Query<Without<Wall, &Position>>,
) {
    if !frame_cnt.is_keyframe() {
        return;
    }
    let mut occupied: HashSet<Position> = all_query.iter().into_iter().cloned().collect();
    for (entity, mut position, mut dir) in &mut bullets.iter() {
        let new_pos = position.add(&*dir);
        if !is_free2(&new_pos, &occupied, &level_info) {
            *dir = MovingDir::zero();
            commands.despawn(entity);
            create_small_explosion(&mut commands).with(*position);
            level_info.do_damage(&new_pos, false);
        } else {
            move_entity2(&mut position, &new_pos, &mut occupied);
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
    let mut occupied: HashMap<Position, Entity> = all_query
        .iter()
        .into_iter()
        .map(|(pos, entity)| (*pos, entity))
        .collect();
    for (entity, mut position, dir) in &mut items.iter() {
        let new_pos = position.add(&*dir);
        let old_pos = *position;
        let is_wall = level_info.is_occupied(&new_pos);
        if is_wall {
            commands.despawn(entity);
        } else if let Some(&entity) = occupied.get(&new_pos) {
            let is_destroyable = destroyable.get::<Destroyable>(entity).is_ok();
            if is_destroyable {
                commands.despawn(entity);
                occupied.remove(&new_pos);
                move_entity(&mut position, &new_pos, &mut occupied);
            } else {
                let entity = occupied.get(&position).unwrap();
                commands.despawn(*entity);
            }
        } else {
            move_entity(&mut position, &new_pos, &mut occupied);
        }
        create_blaster_tail(&mut commands).with(old_pos);
    }
}

pub fn move_laser_head(
    mut commands: Commands,
    frame_cnt: Res<FrameCnt>,
    mut level_info: ResMut<LevelInfo>,
    mut laser_heads: Query<(&mut LaserHead, &mut Position, &mut MovingDir)>,
    mut all_query: Query<Without<Wall, (&Position, Entity)>>,
) {
    if !frame_cnt.is_keyframe() {
        return;
    }
    let mut occupied: HashMap<Position, Entity> = all_query
        .iter()
        .into_iter()
        .map(|(pos, entity)| (*pos, entity))
        .collect();
    for (mut laser_head, mut position, mut dir) in &mut laser_heads.iter() {
        let new_pos = position.add(&*dir);
        let is_laser_tail_in_front = occupied
            .get(&new_pos)
            .map(|entity| all_query.get::<LaserTail>(*entity).is_ok())
            .unwrap_or_default();
        if is_free(&new_pos, &occupied, &level_info) {
            let old_pos = *position;
            move_entity(&mut position, &new_pos, &mut occupied);
            create_laser_tail(&mut commands, &*dir).with(old_pos);
            occupied.insert(old_pos, commands.current_entity().unwrap());
        } else if laser_head.is_moving_back && is_laser_tail_in_front {
            let entity = occupied.remove(&new_pos).unwrap();
            commands.despawn(entity);
            move_entity(&mut position, &new_pos, &mut occupied);
        } else if !laser_head.is_moving_back {
            laser_head.is_moving_back = true;
            *dir = dir.neg();
            level_info.do_damage(&new_pos, false);
        } else {
            *dir = MovingDir::zero();
            let entity = occupied.remove(&position).unwrap();
            commands.despawn(entity);
            create_small_explosion(&mut commands).with(*position);
        }
    };
}

