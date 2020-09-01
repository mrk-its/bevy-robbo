use crate::components::{
    Collectable, Int2Ops, Kind, LaserTail, Moveable, MovingDir, Position, Robbo, Usable,
};
use crate::entities;
use crate::frame_cnt::FrameCnt;
use crate::game_events::{GameEvent, GameEvents};
use crate::inventory::Inventory;

use bevy::prelude::*;
use std::collections::{HashMap, HashSet};

pub fn move_robbo(
    mut commands: Commands,
    (mut inventory, mut events, frame_cnt): (ResMut<Inventory>, ResMut<GameEvents>, Res<FrameCnt>),
    mut robbo: Query<(&Robbo, &mut Position, &MovingDir)>,
    mut items: Query<Without<Robbo, (Entity, &Position)>>,
    positions: Query<&mut Position>,
    moving_dirs: Query<&mut MovingDir>,
    collectables: Query<&Collectable>,
    moveables: Query<&Moveable>,
) {
    if !frame_cnt.do_it() {
        return;
    }
    let mut occupied = HashSet::new();
    let mut entities = HashMap::new();
    for (entity, pos) in &mut items.iter() {
        occupied.insert(*pos);
        entities.insert(*pos, entity);
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
            if let Some(&entity) = entities.get(&new_pos) {
                if let Ok(collectable) = collectables.get::<Collectable>(entity) {
                    inventory.collect(*collectable);
                    commands.despawn(entity);
                    *position = new_pos;
                } else if moveables.get::<Moveable>(entity).is_ok() && !occupied.contains(&new_pos2)
                {
                    if let Ok(mut pos) = positions.get_mut::<Position>(entity) {
                        *pos = new_pos2;
                        *position = new_pos;
                        if let Ok(mut mdir) = moving_dirs.get_mut::<MovingDir>(entity) {
                            // moving box
                            *mdir = *dir
                        }
                    }
                } else if positions.get::<Usable>(entity).is_ok() {
                    events.send(GameEvent::Use(new_pos))
                }
            }
        }
    }
}

fn move_bear(
    occupied: &mut HashMap<Position, Entity>,
    kind: &Kind,
    mut position: Mut<Position>,
    mut dir: Mut<MovingDir>,
) {
    let r1 = |dir: MovingDir| {
        if let Kind::Bear(true) = *kind {
            dir.rotate_counter_clockwise()
        } else {
            dir.rotate_clockwise()
        }
    };
    let r2 = |dir: MovingDir| {
        if let Kind::Bear(true) = *kind {
            dir.rotate_clockwise()
        } else {
            dir.rotate_counter_clockwise()
        }
    };

    let new_dir = r1(*dir);
    let new_dir2 = r2(*dir);
    let new_dir3 = r2(new_dir2);
    if !occupied.contains_key(&(position.add(&new_dir))) {
        *dir = new_dir;
        let entity = occupied.remove(&position).unwrap();
        *position = position.add(&new_dir);
        occupied.insert(*position, entity);
    } else if !occupied.contains_key(&(position.add(&*dir))) {
        let entity = occupied.remove(&position).unwrap();
        *position = position.add(&*dir);
        occupied.insert(*position, entity);
    } else if !occupied.contains_key(&(position.add(&new_dir2))) {
        *dir = new_dir2
    } else if !occupied.contains_key(&(position.add(&new_dir3))) {
        *dir = new_dir3
    }
}

fn move_entity(
    mut position: &mut Mut<Position>,
    new_position: &Position,
    occupied: &mut HashMap<Position, Entity>,
) {
    let entity = occupied.remove(&position).unwrap();
    **position = *new_position;
    occupied.insert(**position, entity);
}

fn move_bird(
    occupied: &mut HashMap<Position, Entity>,
    mut position: Mut<Position>,
    mut dir: Mut<MovingDir>,
) {
    let new_pos = position.add(&*dir);
    if occupied.contains_key(&new_pos) {
        *dir = dir.neg()
    } else {
        move_entity(&mut position, &new_pos, occupied);
    }
}

fn move_box(
    events: &mut ResMut<GameEvents>,
    occupied: &mut HashMap<Position, Entity>,
    processed: &mut HashSet<Position>,
    mut position: Mut<Position>,
    mut dir: Mut<MovingDir>,
) {
    let new_pos = position.add(&*dir);
    if occupied.contains_key(&new_pos) {
        *dir = MovingDir::zero();
        events.send(GameEvent::Damage(new_pos, false));
        processed.insert(new_pos);
    } else {
        move_entity(&mut position, &new_pos, occupied);
    }
}

fn move_bullet(
    events: &mut ResMut<GameEvents>,
    occupied: &mut HashMap<Position, Entity>,
    mut position: Mut<Position>,
    mut dir: Mut<MovingDir>,
) {
    let new_pos = position.add(&*dir);
    if occupied.contains_key(&new_pos) {
        *dir = MovingDir::zero();
        events.send(GameEvent::Remove(*position));
        events.send(GameEvent::Damage(new_pos, false));
    } else {
        move_entity(&mut position, &new_pos, occupied);
    }
}

fn move_laser_head(
    commands: &mut Commands,
    events: &mut ResMut<GameEvents>,
    occupied: &mut HashMap<Position, Entity>,
    mut position: Mut<Position>,
    mut dir: Mut<MovingDir>,
    mut kind: Mut<Kind>,
    others: &Query<(&Position, Entity)>,
) {
    let new_pos = position.add(&*dir);
    let is_moving_back = match *kind {
        Kind::LaserHead { moving_back } => moving_back,
        _ => false,
    };
    let is_laser_tail_in_front = occupied
        .get(&new_pos)
        .map(|entity| others.get::<LaserTail>(*entity).is_ok())
        .unwrap_or_default();
    if !occupied.contains_key(&new_pos) {
        let (tx, ty) = position.as_tuple();
        move_entity(&mut position, &new_pos, occupied);
        entities::laser_tail(commands, dir.x(), dir.y()).with(Position::new(tx, ty));
        occupied.insert(*position, commands.current_entity().unwrap());
    } else if is_moving_back && is_laser_tail_in_front {
        let entity = occupied.remove(&new_pos).unwrap();
        events.send(GameEvent::RemoveEntity(entity));
        // commands.despawn(entity);
        move_entity(&mut position, &new_pos, occupied);
    } else if !is_moving_back {
        *kind = Kind::LaserHead { moving_back: true };
        *dir = dir.neg();
        events.send(GameEvent::Damage(new_pos, false));
    } else {
        *dir = MovingDir::zero();
        events.send(GameEvent::Remove(*position));
        // events.send(GameEvent::Damage(new_pos, false));
    }
}

pub fn move_system(
    mut commands: Commands,
    mut events: ResMut<GameEvents>,
    frame_cnt: Res<FrameCnt>,
    mut moving_items: Query<Without<Robbo, (Entity, &mut Kind, &mut Position, &mut MovingDir)>>,
    mut others: Query<(&Position, Entity)>,
) {
    if !frame_cnt.do_it() {
        return;
    }
    let mut processed = HashSet::new();
    let mut occupied = HashMap::new();
    for (pos, entity) in &mut others.iter() {
        occupied.insert(*pos, entity);
    }
    for (entity, kind, pos, _) in &mut moving_items.iter() {
        occupied.insert(*pos, entity);
    }
    for (entity, kind, position, dir) in &mut moving_items.iter() {
        if *dir == MovingDir::zero() || processed.contains(&*position) {
            continue;
        }
        match *kind {
            Kind::Bird => move_bird(&mut occupied, position, dir),
            Kind::Bear(_) => move_bear(&mut occupied, &kind, position, dir),
            Kind::MovingBox => move_box(&mut events, &mut occupied, &mut processed, position, dir),
            Kind::Bullet => move_bullet(&mut events, &mut occupied, position, dir),
            Kind::LaserHead { moving_back } => move_laser_head(
                &mut commands,
                &mut events,
                &mut occupied,
                position,
                dir,
                kind,
                &others,
            ),
            _ => continue,
        }
    }
}
