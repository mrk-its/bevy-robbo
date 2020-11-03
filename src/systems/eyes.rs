use crate::components::prelude::*;
use crate::frame_cnt::FrameCnt;
use crate::levels::LevelInfo;
use bevy::prelude::*;
use rand::random;
use std::collections::HashSet;

const RANDOM_MOVE_PROP: f32 = 0.5;

pub fn eyes_system(
    frame_cnt: Res<FrameCnt>,
    level_info: Res<LevelInfo>,
    mut queries: QuerySet<(
       Query<Without<Wall, &Position>>,
       Query<With<Robbo, &Position>>,
       Query<With<Eyes, &mut Position>>,
    )>
) {
    if !frame_cnt.is_keyframe() {
        return;
    }
    let occupied: HashSet<Position> = queries.q0().iter().cloned().collect();
    if let Some(robbo_pos) = queries.q1().iter().cloned().next() {
        for mut eyes_pos in queries.q2_mut().iter_mut() {
            let (dx, dy) = if random::<f32>() < RANDOM_MOVE_PROP {
                MovingDir::by_index(random::<usize>() % 4).as_tuple()
            } else {
                robbo_pos.add(&eyes_pos.neg()).as_tuple()
            };
            let dst1 = eyes_pos.add(&MovingDir::new(dx.signum(), 0));
            let dst2 = eyes_pos.add(&MovingDir::new(0, dy.signum()));
            if !occupied.contains(&dst1) && !level_info.is_occupied(&dst1) {
                *eyes_pos = dst1
            } else if !occupied.contains(&dst2) && !level_info.is_occupied(&dst2) {
                *eyes_pos = dst2
            }
        }
    }
}
