#[macro_use]
extern crate log;
mod components;
mod entities;
mod game_events;
mod inventory;
mod levels;
mod plugins;
mod resources;
mod systems;

use bevy::prelude::*;
use game_events::GameEvent;
use inventory::Inventory;
use levels::{LevelInfo, LevelSet, LevelSetLoader};
use plugins::frame_cnt;
use plugins::{AudioPlugin, FrameCnt, FrameCntPlugin, FrameLimiterPlugin, KeyboardPlugin};
use resources::DamageMap;
use structopt::StructOpt;
use systems::*;

mod consts {
    pub const MAX_WIDTH: i32 = 31;
    pub const MAX_HEIGHT: i32 = 16;
}

#[derive(StructOpt, Debug, Default, Clone)]
#[structopt(name = "basic")]
pub struct Opts {
    #[structopt(short, long)]
    pub benchmark_mode: bool,

    #[structopt(short, long)]
    pub debug: bool,

    #[structopt(long)]
    pub no_render: bool,

    #[structopt(long)]
    pub no_audio: bool,

    #[structopt(short, long, default_value = "1")]
    pub level: usize,

    #[structopt(short, long, default_value = "8")]
    pub key_frame_interval: usize,

    #[structopt(short, long, default_value = "60")]
    pub fps: usize,

    #[structopt(long, default_value = "assets/original.txt")]
    pub levelset_path: std::path::PathBuf,
}

fn regular_main() {
    env_logger::init();
    let opts = Opts::from_args();
    let vsync = opts.fps == 60 && !opts.benchmark_mode;
    let mut builder = App::build();
    builder
        .add_resource(Inventory::default())
        .add_resource(LevelInfo::default())
        .add_resource(DamageMap::default())
        .add_resource(Events::<GameEvent>::default())
        .add_resource(opts.clone())
        .add_default_plugins()
        .add_asset::<LevelSet>()
        .add_asset_loader::<LevelSet, LevelSetLoader>()
        .add_plugin(FrameCntPlugin::new(opts.key_frame_interval))
        .add_plugin(KeyboardPlugin)
        .add_plugin(AudioPlugin)
        .add_stage_before(stage::UPDATE, "move")
        .add_stage_before(stage::UPDATE, "move_robbo")
        .add_stage_before(stage::POST_UPDATE, "reload_level")
        .add_stage_before(stage::POST_UPDATE, "shots")
        .add_stage_before(stage::POST_UPDATE, "process_damage")
        .add_stage_before(stage::POST_UPDATE, "game_events")
        .add_stage_after("keyboard", "magnetic_field")
        .add_stage_after("frame_cnt", "tick")
        .add_startup_system(level_setup.system())
        .add_system_to_stage(stage::EVENT_UPDATE, update_game_events.system())
        .add_system_to_stage(stage::EVENT_UPDATE, asset_events.system())
        .add_system_to_stage("magnetic_field", magnetic_field_system.system())
        .add_system_to_stage("process_damage", process_damage.system())
        .add_system_to_stage("move", move_laser_head.system())
        .add_system_to_stage("move", move_bear.system())
        .add_system_to_stage("move", move_bird.system())
        .add_system_to_stage("move", move_pushbox.system())
        .add_system_to_stage("move", move_bullet.system())
        .add_system_to_stage("move", move_blaster_head.system())
        .add_system_to_stage("move", eyes_system.system())
        .add_system_to_stage("move", force_field_system.system())
        .add_system_to_stage("move_robbo", move_robbo.system())
        .add_system_to_stage("shots", shot_system.system())
        .add_system_to_stage("game_events", game_event_system.system())
        .add_system_to_stage("tick", activate_capsule_system.system())
        .add_system_to_stage("tick", tick_system.system())
        .add_system_to_stage("tick", damage_system.system());

    if opts.debug {
        builder
            .add_plugin(bevy::diagnostic::FrameTimeDiagnosticsPlugin::default())
            .add_plugin(bevy::diagnostic::PrintDiagnosticsPlugin::default());
    }

    if !opts.benchmark_mode {
        #[cfg(feature = "render")]
        builder.add_plugin(plugins::RenderPlugin { vsync });
        builder.add_system_to_stage("reload_level", reload_level.system());
        if !vsync {
            builder.add_plugin(FrameLimiterPlugin {
                fps: opts.fps as f32,
            });
        }
    } else {
        if !opts.no_render {
            #[cfg(feature = "render")]
            builder.add_plugin(plugins::RenderPlugin { vsync });
        }
        builder.add_system_to_stage("reload_level", benchmark_reload_level.system());
    }
    #[cfg(feature="wasm")]
    builder.set_runner(|mut app| {
        for _ in 0..10000 {
            app.update();
        }
    });
    builder.run();
}

fn wasm_main() {
    extern crate console_error_panic_hook;
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    console_log::init_with_level(log::Level::Debug).expect("cannot initialize console_log");
    let mut builder = App::build();
    builder
    .add_plugin(bevy::app::ScheduleRunnerPlugin::run_loop(std::time::Duration::from_secs_f64(
        1.0 / 60.0,
    )))
    .add_startup_system(hello_world_system.system())
//    .add_system(counter.system())

    .add_plugin(bevy::type_registry::TypeRegistryPlugin::default())
    //.add_plugin(bevy::core::CorePlugin::default())
    .add_plugin(bevy::input::InputPlugin::default())
    .add_plugin(bevy::window::WindowPlugin::default())
    .add_plugin(bevy::asset::AssetPlugin::default())

    .add_resource(Inventory::default())
    .add_resource(LevelInfo::default())
    .add_resource(DamageMap::default())
    .add_resource(Events::<GameEvent>::default())
    .add_asset::<LevelSet>()
    .add_asset_loader::<LevelSet, LevelSetLoader>()
    .add_plugin(FrameCntPlugin::new(8))
    .add_plugin(KeyboardPlugin)
    .add_plugin(AudioPlugin)
    .add_stage_before(stage::UPDATE, "move")
    .add_stage_before(stage::UPDATE, "move_robbo")
    .add_stage_before(stage::POST_UPDATE, "reload_level")
    .add_stage_before(stage::POST_UPDATE, "shots")
    .add_stage_before(stage::POST_UPDATE, "process_damage")
    .add_stage_before(stage::POST_UPDATE, "game_events")
    .add_stage_after("keyboard", "magnetic_field")
    .add_stage_after("frame_cnt", "tick")

    .add_startup_system(level_setup.system())
    .add_system_to_stage(stage::EVENT_UPDATE, update_game_events.system())
    .add_system_to_stage("game_events", game_event_system.system())

    .add_system_to_stage("magnetic_field", magnetic_field_system.system())
    .add_system_to_stage("process_damage", process_damage.system())
    .add_system_to_stage("move", move_laser_head.system())
    .add_system_to_stage("move", move_bear.system())
    .add_system_to_stage("move", move_bird.system())
    .add_system_to_stage("move", move_pushbox.system())
    .add_system_to_stage("move", move_bullet.system())
    .add_system_to_stage("move", move_blaster_head.system())
    .add_system_to_stage("move", eyes_system.system())
    .add_system_to_stage("move", force_field_system.system())
    .add_system_to_stage("move_robbo", move_robbo.system())
    .add_system_to_stage("shots", shot_system.system())
    .add_system_to_stage("tick", activate_capsule_system.system())
    .add_system_to_stage("tick", tick_system.system())
    .add_system_to_stage("tick", damage_system.system())

    //.add_system_to_stage(stage::EVENT_UPDATE, update_game_events.system())

    .run();
}

fn main() {
    #[cfg(not(feature="wasm"))]
    regular_main();
    #[cfg(feature="wasm")]
    wasm_main();
}

fn hello_world_system() {
    log::info!("hello wasm: {}", unsafe {hello()});
}

fn counter(mut state: Local<CounterState>) {
    if state.count % 60 == 0 {
        log::info!("counter system: {}", state.count);
    }
    state.count += 1;
}

#[derive(Default)]
struct CounterState {
    count: u32,
}

use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/hello.js")]
extern "C" {
    fn hello() -> String;
}

