use crate::components::prelude::*;
use crate::entities::*;
use crate::frame_cnt::FrameCnt;
use crate::levels::LevelInfo;
use crate::resources::DamageMap;
use bevy::prelude::*;
use rand::random;

pub fn shot_system(
    commands: &mut Commands,
    level_info: ResMut<LevelInfo>,
    mut damage_map: ResMut<DamageMap>,
    frame_cnt: Res<FrameCnt>,
    items: Query<(&Position, Entity), Without<Wall>>,
    shooting_items: Query<(&Position, &ShootingDir, &Gun, &ShootingProp)>,
    robbo_query: Query<Entity, With<Robbo>>,
) {
    if !frame_cnt.is_keyframe() {
        return;
    }
    let occupied = level_info.get_occupied(&items);
    for (pos, dir, gun_type, prop) in &mut shooting_items.iter() {
        if random::<f32>() >= prop.0 {
            continue;
        }
        let bullet_pos = pos.add(dir);
        if occupied.is_free(&bullet_pos) {
            match *gun_type {
                Gun::Solid => {
                    create_laser_head(commands, *pos, dir.x(), dir.y()).with(bullet_pos);
                }
                Gun::Blaster => {
                    create_blaster_head(commands, dir.x(), dir.y()).with(bullet_pos);
                }
                Gun::Burst => {
                    create_bullet(commands, dir.x(), dir.y()).with(bullet_pos);
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
