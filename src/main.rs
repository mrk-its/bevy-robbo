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
use bevy::render::renderer::{HeadlessRenderResourceContext, RenderResourceContext};
use bevy::render::render_graph::RenderGraph;

// use std::alloc::System;
// use wasm_tracing_allocator::WasmTracingAllocator;

// #[global_allocator]
// static GLOBAL_ALLOCATOR: WasmTracingAllocator<System> = WasmTracingAllocator(System);

mod consts {
    pub const MAX_BOARD_WIDTH: i32 = 31;
    pub const MAX_BOARD_HEIGHT: i32 = 16;
    pub const STATUS_HEIGHT: i32 = 2;
}

#[derive(StructOpt, Debug, Default, Clone)]
#[structopt(name = "basic")]
pub struct Opts {
    #[structopt(short, long)]
    pub benchmark_mode: bool,

    #[structopt(short, long)]
    pub debug: bool,

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

pub fn render_graph_debug_system(render_ctx: Res<Box<dyn RenderResourceContext>>, render_graph: Res<RenderGraph>) {
    let _ = render_ctx.as_any().downcast_ref::<HeadlessRenderResourceContext>();
    info!("render graph: {:?}", *render_graph);
}

pub fn debug_system(shaders: Res<Assets<Shader>>, render_graph: Res<RenderGraph>) {
    log::info!("num shaders: {}", &shaders.iter().count());
    log::info!("render_graph: {:#?}", *render_graph);
}

fn main() {
    let opts = Opts::from_args();
    info!("opts: {:?}", opts);

    let vsync = opts.fps == 60 && !opts.benchmark_mode;
    let mut builder = App::build();

    builder.add_plugin(plugins::RenderPlugin { vsync });

    #[cfg(not(target_arch = "wasm32"))]
    {
        env_logger::init();
    }
    #[cfg(target_arch = "wasm32")]
    if false {
        extern crate console_error_panic_hook;
        std::panic::set_hook(Box::new(console_error_panic_hook::hook));
        console_log::init_with_level(log::Level::Debug).expect("cannot initialize console_log");
    }
    builder
        .add_resource(WindowDescriptor {
            title: "Robbo".to_string(),
            width: ((32 * consts::MAX_BOARD_WIDTH) as f32) as u32,
            height: ((32 * (consts::MAX_BOARD_HEIGHT + consts::STATUS_HEIGHT)) as f32) as u32,
            resizable: true,
            // mode: window::WindowMode::Fullscreen {use_size: false},
            mode: bevy::window::WindowMode::Windowed,
            #[cfg(target_arch = "wasm32")]
            canvas: Some("#bevy-canvas".to_string()),
            vsync: vsync,
            ..Default::default()
        })
        .add_default_plugins();

    builder
        .add_resource(Inventory::default())
        .add_resource(LevelInfo::default())
        .add_resource(DamageMap::default())
        .add_resource(Events::<GameEvent>::default())
        .add_resource(opts.clone())
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
        builder.add_system_to_stage("reload_level", reload_level.system());
        if !vsync {
            #[cfg(not(target_arch = "wasm32"))]
            builder.add_plugin(FrameLimiterPlugin {
                fps: opts.fps as f32,
            });
        }
    } else {
        builder.add_system_to_stage("reload_level", benchmark_reload_level.system());
    }
    builder.run();
}
