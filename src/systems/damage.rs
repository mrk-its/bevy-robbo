use crate::components::prelude::*;
use crate::entities::create_explosion;
use crate::frame_cnt::FrameCnt;
use bevy::prelude::*;
pub fn damage_system(
    mut commands: Commands,
    frame_cnt: Res<FrameCnt>,
    mut robbo: Query<With<Robbo, (Entity, &Position)>>,
    mut deadly: Query<With<Deadly, (Entity, &Position)>>,
    all: Query<&Position>,
) {
    if !frame_cnt.do_it() {
        return;
    }
    for (robbo_entity, robbo_pos) in &mut robbo.iter() {
        for (entity, deadly_pos) in &mut deadly.iter() {
            let dx = robbo_pos.x() - deadly_pos.x();
            let dy = robbo_pos.y() - deadly_pos.y();

            if (dx.abs(), dy.abs()) == (0, 1) || (dx.abs(), dy.abs()) == (1, 0) {
                if let Ok(magnet) = all.get::<Magnet>(entity) {
                    if (dx, dy) != magnet.as_tuple() {
                        // magnet is deadly on the front only
                        continue;
                    }
                }
                commands.despawn(robbo_entity);
                create_explosion(&mut commands).with(*robbo_pos);
                return;
            }
        }
    }
}
