use crate::components::prelude::*;
use crate::entities::create_small_explosion;
use crate::frame_cnt::FrameCnt;
use crate::game_events::GameEvent;
use crate::levels::{LevelInfo, LevelSet};
use crate::plugins::audio::Sound;
use std::time::Instant;

use bevy::app::AppExit;
use bevy::prelude::*;

pub fn reload_level(
    mut commands: Commands,
    mut level_info: ResMut<LevelInfo>,
    frame_cnt: Res<FrameCnt>,
    mut game_events: ResMut<Events<GameEvent>>,
    mut sounds: ResMut<Events<Sound>>,
    robbo_query: Query<With<Robbo, Entity>>,
    all: Query<Without<Wall, (Entity, &Position)>>,
) {
    if !frame_cnt.is_keyframe() {
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
            sounds.send(Sound::BOMB);
        }
    } else if level_info.missing_robbo_ticks == 20 {
        game_events.send(GameEvent::ReloadLevel(0));
    }
}

pub struct BenchmarkData {
    pub frame_cnt: usize,
    pub start: Instant,
}

impl Default for BenchmarkData {
    fn default() -> Self {
        Self {
            frame_cnt: 0,
            start: Instant::now(),
        }
    }
}

pub fn benchmark_reload_level(
    mut state: Local<BenchmarkData>,
    level_info: ResMut<LevelInfo>,
    level_sets: Res<Assets<LevelSet>>,
    mut game_events: ResMut<Events<GameEvent>>,
    mut app_exit_events: ResMut<Events<AppExit>>,
) {
    if let Some(level_set) = level_sets.get(&level_info.level_set_handle) {
        if state.frame_cnt == 0 {
            *state = BenchmarkData::default();
        }
        if state.frame_cnt % 32 == 0 {
            if level_info.current_level == level_set.levels.len() - 1 {
                let dur = state.start.elapsed();
                println!("number of frames: {}", state.frame_cnt);
                println!("duration: {:?}", dur);
                println!("FPS: {:.2}", (state.frame_cnt as f32) / dur.as_secs_f32());
                app_exit_events.send(AppExit);
            } else {
                game_events.send(GameEvent::ReloadLevel(1));
            }
        }
        state.frame_cnt += 1;
    }
}
