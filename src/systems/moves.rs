use crate::components::prelude::*;
use crate::entities::create_laser_tail;
use crate::frame_cnt::FrameCnt;
use crate::game_events::{GameEvent, GameEvents};
use crate::inventory::Inventory;

use bevy::prelude::*;
use std::collections::{HashMap, HashSet};

pub fn move_robbo(
    mut commands: Commands,
    (mut inventory, mut events, frame_cnt): (ResMut<Inventory>, ResMut<GameEvents>, Res<FrameCnt>),
    mut robbo: Query<(&Robbo, &mut Position, &MovingDir)>,
    mut all: Query<(&mut Position, Entity)>,
) {
    if !frame_cnt.do_it() {
        return;
    }
    let mut occupied = HashSet::new();
    let mut entities = HashMap::new();
    for (pos, entity) in &mut all.iter() {
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
                if let Ok(collectable) = all.get::<Collectable>(entity) {
                    inventory.collect(*collectable);
                    commands.despawn(entity);
                    *position = new_pos;
                } else if all.get::<Moveable>(entity).is_ok() && !occupied.contains(&new_pos2)
                {
                    // investigate why I cannot do all.get_mut<MovingDir>
                    // when &mut Position is replaced with &Position in query
                    let x = all.get_mut::<Position>(entity);
                    if let Ok(mut pos) = x {
                        *pos = new_pos2;
                        *position = new_pos;
                        if let Ok(mut mdir) = all.get_mut::<MovingDir>(entity) {
                            if let Ok(_) = all.get::<PushBox>(entity) {
                                *mdir = *dir
                            }
                        }
                    }
                } else if all.get::<Usable>(entity).is_ok() {
                    events.send(GameEvent::Use(entity, *dir))
                }
            }
        }
    }
}

fn move_bear(
    occupied: &mut HashMap<Position, Entity>,
    is_black: bool,
    mut position: Mut<Position>,
    mut dir: Mut<MovingDir>,
) {
    let r1 = |dir: MovingDir| {
        if !is_black {
            dir.rotate_clockwise()
        } else {
            dir.rotate_counter_clockwise()
        }
    };
    let r2 = |dir: MovingDir| {
        if !is_black {
            dir.rotate_counter_clockwise()
        } else {
            dir.rotate_clockwise()
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
    position: &mut Mut<Position>,
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
        let entity = occupied.get(&position).unwrap();
        events.send(GameEvent::RemoveEntity(*entity));
        events.send(GameEvent::Damage(new_pos, false));
    } else {
        move_entity(&mut position, &new_pos, occupied);
    }
}

fn move_blaster_head(
    commands: &mut Commands,
    occupied: &mut HashMap<Position, Entity>,
    mut position: Mut<Position>,
    dir: Mut<MovingDir>,
    destroyable: &Query<&Destroyable>,
) {
    let new_pos = position.add(&*dir);
    if let Some(&entity) = occupied.get(&new_pos) {
        let is_destroyable = destroyable.get::<Destroyable>(entity).is_ok();
        if is_destroyable {
            commands.despawn(entity);
            occupied.remove(&new_pos);
            move_entity(&mut position, &new_pos, occupied);
        } else {
            let entity = occupied.get(&position).unwrap();
            commands.despawn(*entity);
        }
    } else {
        move_entity(&mut position, &new_pos, occupied);
    }
}
fn move_laser_head(
    commands: &mut Commands,
    events: &mut ResMut<GameEvents>,
    occupied: &mut HashMap<Position, Entity>,
    mut position: &mut Mut<Position>,
    dir: &mut Mut<MovingDir>,
    laser_head: &mut LaserHead,
    others: &Query<(&mut Position, Entity)>,
) {
    let new_pos = position.add(&**dir);
    let is_laser_tail_in_front = occupied
        .get(&new_pos)
        .map(|entity| others.get::<LaserTail>(*entity).is_ok())
        .unwrap_or_default();
    if !occupied.contains_key(&new_pos) {
        let (tx, ty) = position.as_tuple();
        move_entity(&mut position, &new_pos, occupied);
        create_laser_tail(commands, dir.x(), dir.y()).with(Position::new(tx, ty));
        occupied.insert(**position, commands.current_entity().unwrap());
    } else if laser_head.is_moving_back && is_laser_tail_in_front {
        let entity = occupied.remove(&new_pos).unwrap();
        //events.send(GameEvent::RemoveEntity(entity));
        commands.despawn(entity);
        move_entity(&mut position, &new_pos, occupied);
    } else if !laser_head.is_moving_back {
        laser_head.is_moving_back = true;
        **dir = dir.neg();
        events.send(GameEvent::Damage(new_pos, false));
    } else {
        **dir = MovingDir::zero();
        let entity = occupied.remove(&position).unwrap();
        events.send(GameEvent::RemoveEntity(entity));
    }
}

pub fn move_system(
    mut commands: Commands,
    mut events: ResMut<GameEvents>,
    frame_cnt: Res<FrameCnt>,
    mut moving_items: Query<Without<Robbo, (Entity, &mut Position, &mut MovingDir)>>,
    mut all: Query<(&mut Position, Entity)>,
    destroyable: Query<&Destroyable>,
) {
    if !frame_cnt.do_it() {
        return;
    }
    let mut processed: HashSet<Position> = HashSet::new();
    let mut occupied: HashMap<Position, Entity> = all
        .iter()
        .iter()
        .map(|(pos, entity)| (*pos, entity))
        .collect();
    occupied.extend(
        moving_items
            .iter()
            .iter()
            .map(|(entity, pos, _)| (*pos, entity)),
    );
    for (entity, mut position, mut dir) in &mut moving_items.iter() {
        if *dir == MovingDir::zero() || processed.contains(&*position) {
            continue;
        }
        if let Ok(_) = all.get::<Bird>(entity) {
            move_bird(&mut occupied, position, dir);
        } else if let Ok(bear) = all.get::<Bear>(entity) {
            move_bear(&mut occupied, bear.0, position, dir);
        } else if let Ok(_) = all.get::<Bullet>(entity) {
            move_bullet(&mut events, &mut occupied, position, dir);
        } else if let Ok(_) = all.get::<PushBox>(entity) {
            move_box(&mut events, &mut occupied, &mut processed, position, dir);
        } else if let Ok(mut laser_head) = all.get_mut::<LaserHead>(entity) {
            move_laser_head(
                &mut commands,
                &mut events,
                &mut occupied,
                &mut position,
                &mut dir,
                &mut *laser_head,
                &all,
            );
        } else if let Ok(_) = all.get::<BlasterHead>(entity) {
            move_blaster_head(&mut commands, &mut occupied, position, dir, &destroyable);
        }
    }
}
