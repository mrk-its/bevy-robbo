use crate::components::{Int2Ops, Kind, Moveable, MovingDir, Position, Robbo};
use crate::events::Event;
use crate::frame_cnt::FrameCnt;
use bevy::prelude::*;
use std::collections::{HashMap, HashSet};

pub fn move_robbo(
    frame_cnt: Res<FrameCnt>,
    mut robbo: Query<(&Robbo, &mut Position, &MovingDir)>,
    mut items: Query<Without<Robbo, Entity>>,
    moveables: Query<&Moveable>,
    positions: Query<&mut Position>,
    moving_dirs: Query<&mut MovingDir>,
) {
    // println!("move_robbo");
    if !frame_cnt.do_it() {
        return;
    }
    let mut occupied = HashSet::new();
    let mut entities = HashMap::new();

    let mut q = items.iter();
    for entity in &mut q {
        if let Ok(pos) = positions.get::<Position>(entity) {
            occupied.insert(*pos);
            entities.insert(*pos, entity);
        }
    }
    for (_, mut position, dir) in &mut robbo.iter() {
        if *dir == MovingDir::zero() {
            continue;
        }
        let new_pos = position.add(dir);
        let new_pos2 = new_pos.add(dir);
        if !occupied.contains(&new_pos) {
            *position = new_pos;
        } else {
            if let Some(entity) = entities.get(&new_pos) {
                if moveables.get::<Moveable>(*entity).is_ok() && !occupied.contains(&new_pos2) {
                    if let Ok(mut pos) = positions.get_mut::<Position>(*entity) {
                        *pos = new_pos2;
                        *position = new_pos;
                        if let Ok(mut mdir) = moving_dirs.get_mut::<MovingDir>(*entity) {
                            *mdir = *dir
                        }
                    }
                }
            }
        }
    }
}

fn move_bear(
    occupied: &mut HashSet<Position>,
    kind: &Kind,
    mut position: Mut<Position>,
    mut dir: Mut<MovingDir>,
) {
    let r1 = |dir: MovingDir| {
        if *kind == Kind::LBear {
            dir.rotate_counter_clockwise()
        } else {
            dir.rotate_clockwise()
        }
    };
    let r2 = |dir: MovingDir| {
        if *kind == Kind::LBear {
            dir.rotate_counter_clockwise()
        } else {
            dir.rotate_clockwise()
        }
    };

    let new_dir = r1(*dir);
    let new_dir2 = r2(*dir);
    let new_dir3 = r2(new_dir2);
    if !occupied.contains(&(position.add(&new_dir))) {
        *dir = new_dir;
        occupied.remove(&position);
        *position = position.add(&new_dir);
        occupied.insert(*position);
    } else if !occupied.contains(&(position.add(&*dir))) {
        occupied.remove(&position);
        *position = position.add(&*dir);
        occupied.insert(*position);
    } else if !occupied.contains(&(position.add(&new_dir2))) {
        *dir = new_dir2
    } else if !occupied.contains(&(position.add(&new_dir3))) {
        *dir = new_dir3
    }
}

fn move_bird(
    occupied: &mut HashSet<Position>,
    mut position: Mut<Position>,
    mut dir: Mut<MovingDir>,
) {
    let new_pos = position.add(&*dir);
    if occupied.contains(&new_pos) {
        *dir = dir.neg()
    } else {
        occupied.remove(&position);
        *position = new_pos;
        occupied.insert(*position);
    }
}

fn move_box(
    damage_events: &mut ResMut<Events<Event>>,
    occupied: &mut HashSet<Position>,
    mut position: Mut<Position>,
    mut dir: Mut<MovingDir>,
) {
    let new_pos = position.add(&*dir);
    if occupied.contains(&new_pos) {
        *dir = MovingDir::zero();
        damage_events.send(Event::Damage(new_pos));
    } else {
        occupied.remove(&position);
        *position = new_pos;
        occupied.insert(*position);
    }
}
fn move_bullet(
    damage_events: &mut ResMut<Events<Event>>,
    occupied: &mut HashSet<Position>,
    mut position: Mut<Position>,
    mut dir: Mut<MovingDir>,
) {
    let new_pos = position.add(&*dir);
    if occupied.contains(&new_pos) {
        *dir = MovingDir::zero();
        damage_events.send(Event::Remove(*position));
        damage_events.send(Event::Damage(new_pos));
    } else {
        occupied.remove(&position);
        *position = new_pos;
        occupied.insert(*position);
    }
}
pub fn move_system(
    mut damage_events: ResMut<Events<Event>>,
    frame_cnt: Res<FrameCnt>,
    mut moving_items: Query<Without<Robbo, (&Kind, &mut Position, &mut MovingDir)>>,
    mut others: Query<&Position>,
) {
    // println!("move_system");

    if !frame_cnt.do_it() {
        return;
    }
    let mut occupied = HashSet::new();
    for pos in &mut others.iter() {
        occupied.insert(*pos);
    }
    for (_, pos, _) in &mut moving_items.iter() {
        occupied.insert(*pos);
    }
    for (kind, position, dir) in &mut moving_items.iter() {
        if *dir == MovingDir::zero() {
            continue;
        }
        match kind {
            Kind::Bird => move_bird(&mut occupied, position, dir),
            Kind::LBear | Kind::RBear => move_bear(&mut occupied, kind, position, dir),
            Kind::MovingBox => move_box(&mut damage_events, &mut occupied, position, dir),
            Kind::Bullet => move_bullet(&mut damage_events, &mut occupied, position, dir),
            _ => continue,
        }
    }
}
