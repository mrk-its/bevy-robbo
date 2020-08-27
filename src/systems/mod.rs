use crate::components::{
    Destroyable, Kind, Moveable, MovingDir, Position,
    Robbo, Int2Ops, Tiles, IntVec2
};
use crate::events::{Event, DamageState};
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

pub fn keyboard_system(
    commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&Robbo, &mut MovingDir, &mut Tiles)>,
) {
    // println!("keyboard_system");

    let is_shift =
        keyboard_input.pressed(KeyCode::LShift) || keyboard_input.pressed(KeyCode::RShift);

    for (_, mut moving_dir, mut tiles) in &mut query.iter() {
        if is_shift {
            let kx = (keyboard_input.just_pressed(KeyCode::Right) as i32)
                - (keyboard_input.just_pressed(KeyCode::Left) as i32);
            let ky = (keyboard_input.just_pressed(KeyCode::Up) as i32)
                - (keyboard_input.just_pressed(KeyCode::Down) as i32);
            if kx != 0 || ky != 0 {
                println!("shot to {} {}", kx, ky);
                *moving_dir = MovingDir::zero();
            }
        } else {
            let kx = (keyboard_input.pressed(KeyCode::Right) as i32)
                - (keyboard_input.pressed(KeyCode::Left) as i32);
            let ky = (keyboard_input.pressed(KeyCode::Up) as i32)
                - (keyboard_input.pressed(KeyCode::Down) as i32);
            if moving_dir.as_tuple() != (kx, ky) {
                *moving_dir = MovingDir::new(kx, ky);
                *tiles = match (kx, ky) {
                    (-1, 0) => Tiles::new(&[64, 65]),
                    (1, 0) => Tiles::new(&[60, 61]),
                    (0, -1) => Tiles::new(&[62, 63]),
                    (0, 1) => Tiles::new(&[66, 67]),
                    _ => Tiles::new(&tiles.tiles[0..1]),
                }
            }
        }
    }
}

pub fn event_system(
    mut commands: Commands,
    mut state: Local<DamageState>,
    damage_events: Res<Events<Event>>,
    mut destroyable_items: Query<With<Destroyable, (Entity, &Position)>>,
    mut all_items: Query<(Entity, &Position)>,
) {
    // for (entity, pos, sprite) in &mut items.iter() {
    //     println!("entity at: {:?}", pos);
    // }
    // println!("#################");
    // println!("damage_system");
    for event in state.damage_reader.iter(&damage_events) {
        match event {
            Event::Damage(position) => {
                println!("damage at: {:?}", position);
                for (entity, pos) in &mut destroyable_items.iter() {
                    if position == pos {
                        println!("destroying entity at: {:?}", pos);
                        commands.despawn(entity);
                        //commands.remove_one::<Position>(entity);
                        //*translation = Translation(Vec3::new(-10000.0, -10000.0, -10000.0));
                    }
                }

            },
            Event::Remove(position) => {
                for (entity, pos) in &mut all_items.iter() {
                    if position == pos {
                        println!("destroying entity at: {:?}", pos);
                        commands.despawn(entity);
                        //commands.remove_one::<Position>(entity);
                        //*translation = Translation(Vec3::new(-10000.0, -10000.0, -10000.0));
                    }
                }
            }
            _ => ()
        }
    }
}
