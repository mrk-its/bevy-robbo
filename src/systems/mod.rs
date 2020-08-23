use bevy::prelude::*;
use std::collections::HashSet;

use crate::components::{
    rotate_clockwise, rotate_counter_clockwise, Kind, MovingDir, Position, Robbo, Tile,
};

fn move_robbo(occupied: &mut HashSet<Position>, mut position: Mut<Position>, dir: Mut<MovingDir>) {
    let new_pos = *position + *dir;
    if !occupied.contains(&new_pos) {
        occupied.remove(&position);
        *position = new_pos;
        occupied.insert(*position);
    }
}

fn move_bear(
    occupied: &mut HashSet<Position>,
    kind: &Kind,
    mut position: Mut<Position>,
    mut dir: Mut<MovingDir>,
) {
    type RotateFn = dyn Fn(MovingDir) -> MovingDir;

    let (r1, r2): (&RotateFn, &RotateFn) = if *kind == Kind::LBear {
        (&rotate_counter_clockwise, &rotate_clockwise)
    } else {
        (&rotate_clockwise, &rotate_counter_clockwise)
    };

    let new_dir = r1(*dir);
    let new_dir2 = r2(*dir);
    let new_dir3 = r2(new_dir2);
    if !occupied.contains(&(*position + new_dir)) {
        *dir = new_dir;
        occupied.remove(&position);
        *position = *position + new_dir;
        occupied.insert(*position);
    } else if !occupied.contains(&(*position + *dir)) {
        occupied.remove(&position);
        *position = *position + *dir;
        occupied.insert(*position);
    } else if !occupied.contains(&(*position + new_dir2)) {
        *dir = new_dir2
    } else if !occupied.contains(&(*position + new_dir3)) {
        *dir = new_dir3
    }
}

fn move_bird(
    occupied: &mut HashSet<Position>,
    mut position: Mut<Position>,
    mut dir: Mut<MovingDir>,
) {
    let new_pos = *position + *dir;
    if occupied.contains(&new_pos) {
        *dir = -*dir
    } else {
        occupied.remove(&position);
        *position = new_pos;
        occupied.insert(*position);
    }
}

pub fn move_system(
    mut items: Query<(&Kind, &mut Position, &mut MovingDir)>,
    mut others: Query<Without<MovingDir, &Position>>,
) {
    let mut occupied = HashSet::new();

    for pos in &mut others.iter() {
        occupied.insert(*pos);
    }
    for (_, pos, _) in &mut items.iter() {
        occupied.insert(*pos);
    }
    for (kind, position, dir) in &mut items.iter() {
        match kind {
            Kind::Bird => move_bird(&mut occupied, position, dir),
            Kind::LBear | Kind::RBear => move_bear(&mut occupied, kind, position, dir),
            Kind::Robbo => move_robbo(&mut occupied, position, dir),
            _ => continue,
        }
    }
}

pub fn keyboard_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&Robbo, &mut MovingDir)>,
) {
    for (_, mut moving_dir) in &mut query.iter() {
        let kx = (keyboard_input.pressed(KeyCode::Right) as i32)
            - (keyboard_input.pressed(KeyCode::Left) as i32);
        let ky = (keyboard_input.pressed(KeyCode::Up) as i32)
            - (keyboard_input.pressed(KeyCode::Down) as i32);

        *moving_dir = MovingDir(kx, ky);
    }
}

pub fn prepare_render(
    position: &Position,
    tile: &Tile,
    mut translation: Mut<Translation>,
    mut sprite: Mut<TextureAtlasSprite>,
) {
    translation.set_x((position.0 as f32) * 32.0);
    translation.set_y((position.1 as f32) * 32.0);
    translation.set_z(0.0);
    sprite.index = tile.0;
}
