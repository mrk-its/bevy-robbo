use crate::game_events::GameEvent;
use crate::levels::{LevelInfo, LevelSet};
use bevy::prelude::*;

#[derive(Default)]
pub struct AssetEventsState {
    reader: EventReader<AssetEvent<LevelSet>>,
}

#[cfg(not(feature = "wasm"))]
pub fn asset_events(
    mut game_events: ResMut<Events<GameEvent>>,
    opts: Res<crate::Opts>,
    mut level_info: ResMut<LevelInfo>,
    mut state: Local<AssetEventsState>,
    events: Res<Events<AssetEvent<LevelSet>>>,
) {
    for event in state.reader.iter(&events) {
        let handle = match event {
            AssetEvent::Created { handle } => handle,
            AssetEvent::Modified { handle } => handle,
            _ => continue,
        };
        level_info.level_set_handle = *handle;
        level_info.current_level = (opts.level - 1).max(0);
        game_events.send(GameEvent::ReloadLevel(0));
    }
}

#[cfg(not(feature = "wasm"))]
pub fn level_setup(asset_server: Res<AssetServer>, opts: Res<crate::Opts>) {
    info!("loading levelset");
    asset_server.watch_for_changes().unwrap();
    asset_server
        .load::<Handle<LevelSet>, _>(opts.levelset_path.clone())
        .unwrap();
}

#[cfg(feature = "wasm")]
pub fn level_setup(
    mut game_events: ResMut<Events<GameEvent>>,
    mut level_sets: ResMut<Assets<LevelSet>>,
    mut level_info: ResMut<LevelInfo>,
) {
    info!("wasm levelset setup");
    let handle: Handle<LevelSet> = Handle::from_u128(0xbfff9d8b3f27461cac5355577f40120b);
    let level_data = include_str!("../../assets/original.txt");
    level_sets.set(handle, LevelSet::new(level_data));
    level_info.level_set_handle = handle;
    level_info.current_level = 0;
    game_events.send(GameEvent::ReloadLevel(0));
}

#[cfg(feature = "wasm")]
pub fn asset_events() {}

