use crate::components::{Int2Ops, Magnet, MovingDir, Position, Robbo, Wall};
use crate::levels::LevelInfo;
use bevy::prelude::*;

pub fn magnetic_field_system(
    level_info: Res<LevelInfo>,
    mut robbo: Query<With<Robbo, (&mut Position, &mut MovingDir)>>,
    mut all: Query<Without<Wall, (&Position, Entity)>>,
    mut magnets_query: Query<(&Magnet, &Position)>,
) {
    let occupancy = level_info.get_occupancy(&mut all);
    for (robbo_pos, mut moving_dir) in &mut robbo.iter() {
        let attracts = |dir: i32, a: i32, b: i32| dir == (b - a).signum();
        for (magnet_dir, magnet_pos) in magnets_query.iter().iter().filter(|(magnet, pos)| {
            (pos.x() == robbo_pos.x()) && attracts(magnet.y(), pos.y(), robbo_pos.y())
                || (pos.y() == robbo_pos.y()) && attracts(magnet.x(), pos.x(), robbo_pos.x())
        }) {
            let mut pos = magnet_pos.add(magnet_dir);
            while pos != *robbo_pos && occupancy.is_free(&pos) {
                pos = pos.add(magnet_dir);
            }
            if pos == *robbo_pos {
                let (dx, dy) = magnet_dir.as_tuple();
                *moving_dir = MovingDir::new(-dx, -dy);
            }
        }
    }
}
