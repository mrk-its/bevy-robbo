use crate::components::prelude::*;
use crate::frame_cnt::FrameCnt;
use crate::levels::LevelInfo;
use crate::entities::create_small_explosion;
use crate::game_events::GameEvent;
use crate::sounds;
use bevy::prelude::*;

pub fn reload_level(
    mut commands: Commands,
    mut level_info: ResMut<LevelInfo>,
    frame_cnt: Res<FrameCnt>,
    mut game_events: ResMut<Events<GameEvent>>,
    mut robbo_query: Query<With<Robbo, Entity>>,
    mut all: Query<Without<Wall, (Entity, &Position)>>,
) {
    if !frame_cnt.do_it() {
        return;
    }
    for _ in &mut robbo_query.iter() {
        level_info.missing_robbo_ticks = 0;
        return;
    }
    level_info.missing_robbo_ticks += 1;
    if level_info.missing_robbo_ticks == 10 {
        for (entity, pos) in &mut all.iter() {
            commands.despawn(entity);
            create_small_explosion(&mut commands).with(*pos);
            game_events.send(GameEvent::PlaySound(sounds::BOMB));
        }
    } else if level_info.missing_robbo_ticks == 20 {
        game_events.send(GameEvent::ReloadLevel(0));
    }
}
