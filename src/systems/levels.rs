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
        match event {
            AssetEvent::Created {..}| AssetEvent::Modified { .. } => {
                level_info.current_level = (opts.level - 1).max(0);
                game_events.send(GameEvent::ReloadLevel(0));
            }
            _ => continue,
        };
    }
}

pub fn level_setup(asset_server: Res<AssetServer>, mut level_info: ResMut<LevelInfo>, opts: Res<crate::Opts>) {
    //asset_server.watch_for_changes().unwrap();
    level_info.level_set_handle = asset_server.load(opts.levelset_path.clone());
}
