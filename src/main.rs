mod components;
mod entities;
mod events;
mod frame_cnt;
mod frame_limiter;
mod systems;

use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, PrintDiagnosticsPlugin};
use bevy::prelude::*;
use bevy::sprite::{Rect, TextureAtlas};
use bevy::window;
use components::{Int2Ops, Position, Tiles};
use frame_cnt::{FrameCnt, FrameCntPlugin};
use frame_limiter::FrameLimiterPlugin;
use systems::{event_system, keyboard_system, move_robbo, move_system};

const WIDTH: i32 = 32;
const HEIGHT: i32 = 16;
const SCALE: f32 = 1.5;
const FPS: f32 = 30.0;

use bevy::asset::{AddAsset, AssetLoader};

#[derive(Debug)]
pub struct Level(String);

#[derive(Default)]
pub struct LevelLoader;
impl AssetLoader<Level> for LevelLoader {
    fn from_bytes(
        &self,
        _asset_path: &std::path::Path,
        bytes: Vec<u8>,
    ) -> Result<Level, anyhow::Error> {
        Ok(Level(String::from_utf8(bytes).unwrap()))
    }

    fn extensions(&self) -> &[&str] {
        static EXT: &[&str] = &["txt"];
        EXT
    }
}

fn main() {
    App::build()
        .add_resource(WindowDescriptor {
            title: "Robbo".to_string(),
            width: ((32 * WIDTH) as f32 * SCALE) as u32,
            height: ((32 * HEIGHT) as f32 * SCALE) as u32,
            vsync: true,
            resizable: true,
            mode: window::WindowMode::Windowed,
            ..Default::default()
        })
        .add_default_plugins()
        .add_asset::<Level>()
        .add_asset_loader::<Level, LevelLoader>()
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        //.add_plugin(PrintDiagnosticsPlugin::default())
        .add_plugin(FrameLimiterPlugin { fps: FPS })
        .add_plugin(FrameCntPlugin)
        .add_startup_system(setup.system())
        .add_startup_system_to_stage("post_startup", post_setup.system())
        .add_system(asset_events.system())
        .add_system(keyboard_system.system())
        .add_system(move_system.system())
        .add_system(move_robbo.system()) // it must be after move_system
        .add_system(event_system.system())
        .add_system(prepare_render.system())
        .add_event::<events::Event>()
        //.add_stage_before("update", "process_damage")
        //.add_system_to_stage("process_damage", event_system.system())
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    asset_server.watch_for_changes().unwrap();
    commands
        .spawn(Camera2dComponents {
            translation: Translation::new(
                16.0 * SCALE * ((WIDTH - 1) as f32),
                16.0 * SCALE * ((HEIGHT - 1) as f32),
                0.0,
            ),
            ..Default::default()
        })
        .spawn(entities::robbo(10, 10))
        .spawn(entities::bird(10, 5, 1, 0))
        .spawn(entities::bird(10, 7, 0, 1))
        .spawn(entities::lbear(1, 5, 0, -1))
        .spawn(entities::lbear(1, 10, 0, -1))
        .spawn(entities::moving_box(5, 5))
        .spawn(entities::static_box(4, 4))
        .spawn(entities::bullet(7, 12, 0, -1));

    for x in 0..WIDTH {
        commands
            .spawn(entities::wall(x, 0))
            .spawn(entities::wall(x, HEIGHT - 1));
    }
    for y in 1..HEIGHT - 1 {
        commands
            .spawn(entities::wall(0, y))
            .spawn(entities::wall(WIDTH - 1, y));
    }
}

fn post_setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut levels: ResMut<Assets<Level>>,
    mut items: Query<(Entity, &Position)>,
) {
    let texture_handle = asset_server.load("assets/icons32.png").unwrap();
    let level_handle: Handle<Level> = asset_server.load("assets/level.txt").unwrap();
    //let level_handle: Handle<Level> = asset_server.load_sync(&mut levels, "assets/level.txt").unwrap();
    let level = levels.get(&level_handle);
    println!("level: {:?}", level);
    let mut texture_atlas =
        TextureAtlas::new_empty(texture_handle, Vec2::new(12.0 * 34.0, 8.0 * 34.0));
    for y in 0..8 {
        for x in 0..12 {
            texture_atlas.add_texture(Rect {
                min: Vec2::new((2 + x * 34) as f32, (2 + y * 34) as f32),
                max: Vec2::new((2 + x * 34 + 31) as f32, (2 + y * 34 + 31) as f32),
            });
        }
    }
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    for (entity, _) in &mut items.iter() {
        commands.insert(
            entity,
            SpriteSheetComponents {
                texture_atlas: texture_atlas_handle,
                scale: Scale(SCALE),
                ..Default::default()
            },
        );
    }
}

pub fn prepare_render(
    frame_cnt: Res<FrameCnt>,
    position: &Position,
    mut tiles: Mut<Tiles>,
    mut translation: Mut<Translation>,
    mut sprite: Mut<TextureAtlasSprite>,
) {
    const BOX_SIZE: f32 = 32.0 * SCALE;
    const STEPS: usize = 4;
    const MIN_STEP: f32 = BOX_SIZE / (STEPS as f32);

    let steps_left = (STEPS - (frame_cnt.value() % STEPS)) as f32;
    let dest = Vec3::new(
        position.x() as f32 * BOX_SIZE,
        position.y() as f32 * BOX_SIZE,
        0.0,
    );
    let cur = translation.0;
    let step = Vec3::new(
        (dest.x() - cur.x()) / steps_left,
        (dest.y() - cur.y()) / steps_left,
        0.0,
    );
    if step.abs() <= Vec3::new(MIN_STEP, MIN_STEP, 0.0) {
        translation.set_x(cur.x() + step.x());
        translation.set_y(cur.y() + step.y());
    } else {
        translation.set_x(dest.x());
        translation.set_y(dest.y());
    }
    sprite.index = tiles.tiles[tiles.current];
    if frame_cnt.do_it() {
        tiles.current = (tiles.current + 1) % tiles.tiles.len();
    }
}

#[derive(Default)]
pub struct AssetEventsState {
    reader: EventReader<AssetEvent<Level>>,
}

pub fn asset_events(
    mut state: Local<AssetEventsState>,
    mut levels: ResMut<Assets<Level>>,
    events: Res<Events<AssetEvent<Level>>>,
) {
    for event in state.reader.iter(&events) {
        match event {
            AssetEvent::Created { handle: handle } => {
                println!("ASSET CREATED: {:?}", levels.get(handle));
            }
            AssetEvent::Modified { handle: handle } => {
                println!("ASSET MODIFIED: {:?}", levels.get(handle));
            }
            _ => {}
        }
    }
}
