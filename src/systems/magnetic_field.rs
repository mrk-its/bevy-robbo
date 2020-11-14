use crate::components::{Int2Ops, Magnet, MovingDir, Position, Robbo, Wall};
use crate::levels::LevelInfo;
use bevy::prelude::*;

pub fn magnetic_field_system(
    level_info: Res<LevelInfo>,
    mut queries: QuerySet<(
        Query<(&Position, Entity), Without<Wall>>,
        Query<(&Magnet, &Position)>,
        Query<(&mut Position, &mut MovingDir), With<Robbo>>,
    )>,
) {
    let occupied = level_info.get_occupied(queries.q0());
    let magnets: Vec<(Magnet, Position)> = queries
        .q1()
        .iter()
        .map(|(a, b)| (*a, *b))
        .collect();

    for (robbo_pos, mut moving_dir) in queries.q2_mut().iter_mut() {
        let attracts = |dir: i32, a: i32, b: i32| dir == (b - a).signum();
        for (magnet_dir, magnet_pos) in magnets.iter().filter(|(magnet, pos)| {
            (pos.x() == robbo_pos.x()) && attracts(magnet.y(), pos.y(), robbo_pos.y())
                || (pos.y() == robbo_pos.y()) && attracts(magnet.x(), pos.x(), robbo_pos.x())
        }) {
            let mut pos = magnet_pos.add(magnet_dir);
            while pos != *robbo_pos && occupied.is_free(&pos) {
                pos = pos.add(magnet_dir);
            }
            if pos == *robbo_pos {
                let (dx, dy) = magnet_dir.as_tuple();
                *moving_dir = MovingDir::new(-dx, -dy);
            }
        }
    }
}
