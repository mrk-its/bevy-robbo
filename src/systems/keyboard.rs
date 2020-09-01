use crate::components::{Int2Ops, MovingDir, Position, Robbo, ShootingDir, Tiles};
use bevy::prelude::*;

pub fn keyboard_system(
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(Entity, &Robbo, &Position, &mut MovingDir, &mut Tiles)>,
) {
    let is_shift =
        keyboard_input.pressed(KeyCode::LShift) || keyboard_input.pressed(KeyCode::RShift);

    for (entity, _, pos, mut moving_dir, mut tiles) in &mut query.iter() {
        if is_shift {
            let kx = (keyboard_input.just_pressed(KeyCode::Right) as i32)
                - (keyboard_input.just_pressed(KeyCode::Left) as i32);
            let ky = (keyboard_input.just_pressed(KeyCode::Up) as i32)
                - (keyboard_input.just_pressed(KeyCode::Down) as i32);
            if kx != 0 || ky != 0 {
                *moving_dir = MovingDir::zero();
                commands.insert_one(entity, ShootingDir::new(kx, ky).with_propability(1.0));
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
