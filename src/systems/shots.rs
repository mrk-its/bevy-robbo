use crate::components::prelude::*;
use crate::entities::*;
use crate::frame_cnt::FrameCnt;
use crate::game_events::GameEvent;

use bevy::prelude::*;
use rand::random;
use std::collections::HashSet;

pub fn shot_system(
    mut commands: Commands,
    mut game_events: ResMut<Events<GameEvent>>,
    frame_cnt: Res<FrameCnt>,
    mut items: Query<&Position>,
    mut shooting_items: Query<(&Position, &ShootingDir, &GunType, &ShootingProp)>,
    mut robbo_query: Query<With<Robbo, Entity>>,
) {
    if !frame_cnt.is_keyframe() {
        return;
    }
    let mut occupied = HashSet::new();
    for pos in &mut items.iter() {
        occupied.insert(*pos);
    }
    for (pos, dir, gun_type, prop) in &mut shooting_items.iter() {
        if random::<f32>() >= prop.0 {
            continue;
        }
        let bullet_pos = pos.add(dir);
        if !occupied.contains(&bullet_pos) {
            match *gun_type {
                GunType::Solid => {
                    create_laser_head(&mut commands, dir.x(), dir.y()).with(bullet_pos);
                }
                GunType::Blaster => {
                    create_blaster_head(&mut commands, dir.x(), dir.y()).with(bullet_pos);
                }
                GunType::Burst => {
                    create_bullet(&mut commands, dir.x(), dir.y()).with(bullet_pos);
                }
            }
        } else {
            game_events.send(GameEvent::Damage(bullet_pos, false));
        }
    }
    for entity in &mut robbo_query.iter() {
        commands.remove_one::<ShootingDir>(entity);
    }
}
