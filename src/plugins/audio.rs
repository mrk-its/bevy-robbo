use bevy::prelude::*;
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub enum Sound {
    AMMO,
    KEY,
    SCREW,
    BOMB,
    WALK,
    TELEPORT,
    SHOT,
    SPAWN,
    DOOR,
    BURN,
    CAPSULE,
}

pub struct AudioPlugin;

impl Plugin for AudioPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_resource(Events::<Sound>::default());
        app.add_system_to_stage(stage::EVENT, Events::<Sound>::update_system.system());
        #[cfg(feature="audio")]
        {
            app.add_startup_system(audio_setup.system());
            app.add_system_to_stage(stage::POST_UPDATE, play_sounds_system.system());
        }
    }
}
#[derive(Default)]
pub struct State {
    pub event_reader: EventReader<Sound>,
}

#[cfg(feature="audio")]
pub fn play_sounds_system(
    mut state: Local<State>,
    audio_output: Res<AudioOutput>,
    asset_server: Res<AssetServer>,
    mut events: ResMut<Events<Sound>>,
    opts: Res<crate::Opts>,
) {
    static SOUND_FILES: &[&str] = &[
        "ammo.ogg",
        "key.ogg",
        "screw.ogg",
        "bomb.ogg",
        "walk.ogg",
        "teleport.ogg",
        "shot.ogg",
        "spawn.ogg",
        "door.ogg",
        "burn.ogg",
        "capsule.ogg",
    ];

    if opts.no_audio {
        return;
    }
    let mut played = std::collections::HashSet::new();

    for sound in state.event_reader.iter(&mut events) {
        if played.contains(sound) {
            continue;
        }
        if let Some(&filename) = SOUND_FILES.get(*sound as usize) {
            let path = format!("sounds/{}", filename);
            let handle = asset_server.get_handle(path).unwrap();
            audio_output.play(handle);
        }
        played.insert(*sound);
    }
}

#[cfg(feature="audio")]
pub fn audio_setup(asset_server: Res<AssetServer>, opts: Res<crate::Opts>) {
    if !opts.no_audio {
        asset_server.load_asset_folder("sounds/").unwrap();
    }
}
