use crate::components::{MovingDir, Robbo, Tiles, Int2Ops};
use bevy::prelude::*;

pub fn keyboard_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&Robbo, &mut MovingDir, &mut Tiles)>,
) {
    let is_shift =
        keyboard_input.pressed(KeyCode::LShift) || keyboard_input.pressed(KeyCode::RShift);

    for (_, mut moving_dir, mut tiles) in &mut query.iter() {
        if is_shift {
            let kx = (keyboard_input.just_pressed(KeyCode::Right) as i32)
                - (keyboard_input.just_pressed(KeyCode::Left) as i32);
            let ky = (keyboard_input.just_pressed(KeyCode::Down) as i32)
                - (keyboard_input.just_pressed(KeyCode::Up) as i32);
            if kx != 0 || ky != 0 {
                println!("shot to {} {}", kx, ky);
                *moving_dir = MovingDir::zero();
            }
        } else {
            let kx = (keyboard_input.pressed(KeyCode::Right) as i32)
                - (keyboard_input.pressed(KeyCode::Left) as i32);
            let ky = (keyboard_input.pressed(KeyCode::Down) as i32)
                - (keyboard_input.pressed(KeyCode::Up) as i32);
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
