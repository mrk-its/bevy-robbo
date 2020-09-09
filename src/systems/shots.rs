use crate::components::prelude::*;
use crate::entities::*;
use crate::frame_cnt::FrameCnt;
use crate::levels::LevelInfo;
use crate::resources::DamageMap;
use bevy::prelude::*;
use rand::random;

pub fn shot_system(
    mut commands: Commands,
    level_info: ResMut<LevelInfo>,
    mut damage_map: ResMut<DamageMap>,
    frame_cnt: Res<FrameCnt>,
    mut items: Query<Without<Wall, (&Position, Entity)>>,
    mut shooting_items: Query<(&Position, &ShootingDir, &Gun, &ShootingProp)>,
    mut robbo_query: Query<With<Robbo, Entity>>,
) {
    if !frame_cnt.is_keyframe() {
        return;
    }
    let occupancy = level_info.get_occupancy(&mut items);
    for (pos, dir, gun_type, prop) in &mut shooting_items.iter() {
        if random::<f32>() >= prop.0 {
            continue;
        }
        let bullet_pos = pos.add(dir);
        if occupancy.is_free(&bullet_pos) {
            match *gun_type {
                Gun::Solid => {
                    create_laser_head(&mut commands, dir.x(), dir.y()).with(bullet_pos);
                }
                Gun::Blaster => {
                    create_blaster_head(&mut commands, dir.x(), dir.y()).with(bullet_pos);
                }
                Gun::Burst => {
                    create_bullet(&mut commands, dir.x(), dir.y()).with(bullet_pos);
                }
            }
        } else {
            damage_map.do_damage(&bullet_pos, false);
        }
    }
    for entity in &mut robbo_query.iter() {
        commands.remove_one::<ShootingDir>(entity);
    }
}
