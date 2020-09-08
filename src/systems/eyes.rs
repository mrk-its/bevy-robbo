use crate::components::prelude::*;
use crate::frame_cnt::FrameCnt;
use bevy::prelude::*;
use rand::random;
use std::collections::HashSet;

const RANDOM_MOVE_PROP: f32 = 0.5;

pub fn eyes_system(
    frame_cnt: Res<FrameCnt>,
    mut eyes_query: Query<With<Eyes, &mut Position>>,
    mut all_query: Query<&Position>,
    mut robbo_query: Query<With<Robbo, &Position>>,
) {
    if !frame_cnt.is_keyframe() {
        return;
    }
    let occupied: HashSet<Position> = all_query.iter().into_iter().cloned().collect();

    for mut eyes_pos in &mut eyes_query.iter() {
        for robbo_pos in &mut robbo_query.iter() {
            let (dx, dy) = if random::<f32>() < RANDOM_MOVE_PROP {
                MovingDir::by_index(random::<usize>() % 4).as_tuple()
            } else {
                robbo_pos.add(&eyes_pos.neg()).as_tuple()
            };
            let dst1 = eyes_pos.add(&MovingDir::new(dx.signum(), 0));
            let dst2 = eyes_pos.add(&MovingDir::new(0, dy.signum()));
            if !occupied.contains(&dst1) {
                *eyes_pos = dst1
            } else if !occupied.contains(&dst2) {
                *eyes_pos = dst2
            }
        }
    }
}
