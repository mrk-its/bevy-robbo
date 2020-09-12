use crate::game_events::GameEvent;
use crate::levels::{LevelInfo, LevelSet};
use bevy::prelude::*;

#[derive(Default)]
pub struct AssetEventsState {
    reader: EventReader<AssetEvent<LevelSet>>,
}

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

pub fn level_setup(asset_server: Res<AssetServer>, opts: Res<crate::Opts>) {
    asset_server.watch_for_changes().unwrap();
    asset_server.load_asset_folder("assets/sounds/").unwrap();
    asset_server
        .load::<Handle<LevelSet>, _>(opts.levelset_path.clone())
        .unwrap();
}
