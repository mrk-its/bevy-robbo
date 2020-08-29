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
use components::{Int2Ops, Kind, Position, StartPosition, Tiles};
use frame_cnt::{FrameCnt, FrameCntPlugin};
use frame_limiter::FrameLimiterPlugin;
use systems::{event_system, keyboard_system, move_robbo, move_system};

const WIDTH: i32 = 32;
const HEIGHT: i32 = 16;
const SCALE: f32 = 1.5;
const FPS: f32 = 30.0;
const BOX_SIZE: f32 = 32.0 * SCALE;

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

pub struct TextureAtlasHandle(pub Option<Handle<TextureAtlas>>);

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
        .add_resource(TextureAtlasHandle(None))
        .add_default_plugins()
        .add_asset::<Level>()
        .add_asset_loader::<Level, LevelLoader>()
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugin(PrintDiagnosticsPlugin::default())
        .add_plugin(FrameLimiterPlugin { fps: FPS })
        .add_plugin(FrameCntPlugin)
        .add_startup_system(setup.system())
        //.add_startup_system_to_stage("post_startup", post_setup.system())
        .add_system(keyboard_system.system())
        .add_system(move_system.system())
        .add_system(move_robbo.system()) // it must be after move_system
        .add_system(event_system.system())
        .add_system(asset_events.system())
        .add_system(prepare_render.system())
        .add_event::<events::Event>()
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut texture_atlas_handle: ResMut<TextureAtlasHandle>,
) {
    asset_server.watch_for_changes().unwrap();

    let texture_handle = asset_server.load("assets/icons32.png").unwrap();
    let mut texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(384.0, 256.0), 12, 8);
    texture_atlas_handle.0 = Some(texture_atlases.add(texture_atlas));

    let _level_handle: Handle<Level> = asset_server.load("assets/level.txt").unwrap();

    let prepare = |commands: &mut Commands| {
        commands
            .with(Position::new(-100, -100))
            .with(StartPosition::new(-100, -100))
            .with_bundle(SpriteSheetComponents {
                texture_atlas: texture_atlas_handle.0.unwrap(),
                scale: Scale(SCALE),
                ..Default::default()
            });
    };
    use bevy::render::camera::{OrthographicProjection, WindowOrigin};

    commands.spawn(Camera2dComponents {
        translation: Translation::new(-BOX_SIZE / 2.0, BOX_SIZE / 2.0, 0.0),
        orthographic_projection: OrthographicProjection {
            bottom: 0.0,
            top: HEIGHT as f32 * BOX_SIZE / 2.0,
            left: 0.0,
            right: WIDTH as f32 * BOX_SIZE / 2.0,
            window_origin: WindowOrigin::BottomLeft,
            ..Default::default()
        },
        ..Default::default()
    });
}

pub fn prepare_render(
    frame_cnt: Res<FrameCnt>,
    position: &Position,
    mut tiles: Mut<Tiles>,
    mut translation: Mut<Translation>,
    mut sprite: Mut<TextureAtlasSprite>,
) {
    const STEPS: usize = 4;
    const MIN_STEP: f32 = BOX_SIZE / (STEPS as f32);

    let steps_left = (STEPS - (frame_cnt.value() % STEPS)) as f32;
    let dest = Vec3::new(
        position.x() as f32 * BOX_SIZE,
        (HEIGHT - position.y()) as f32 * BOX_SIZE,
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

pub fn create_level(
    commands: &mut Commands,
    texture_atlas_handle: &Res<TextureAtlasHandle>,
    items: &mut Query<(Entity, &Kind, &StartPosition)>,
    level: &Level,
) {
    for (y, line) in level.0.split("\n").enumerate() {
        for (x, c) in line.chars().enumerate() {
            let (x, y) = (x as i32, y as i32);
            match c {
                'Q' => commands.spawn(entities::wall()),
                'R' => commands.spawn(entities::robbo()),
                '#' => commands.spawn(entities::static_box()),
                '~' => commands.spawn(entities::push_box()),
                '^' => commands.spawn(entities::bird()),
                '@' => commands.spawn(entities::lbear(-1, 0)),
                '*' => commands.spawn(entities::rbear(1, 0)),
                _ => {
                    for (entity, kind, pos) in &mut items.iter() {
                        if pos.as_tuple() == (x, y) {
                            commands.despawn(entity);
                        }
                    }
                    continue;
                }
            };
            let entity = items
                .iter()
                .into_iter()
                .find(|(_, kind, pos)| pos.as_tuple() == (x, y))
                .map(|t| t)
                .is_some();

            if entity {
                commands.despawn(commands.current_entity().unwrap());
                continue;
            }

            commands
                .with(Position::new(x, y))
                .with(StartPosition::new(x, y))
                .with_bundle(SpriteSheetComponents {
                    texture_atlas: texture_atlas_handle.0.unwrap(),
                    scale: Scale(SCALE),
                    ..Default::default()
                });
        }
    }
}

pub fn asset_events(
    mut commands: Commands,
    mut state: Local<AssetEventsState>,
    texture_atlas_handle: Res<TextureAtlasHandle>,
    levels: ResMut<Assets<Level>>,
    events: Res<Events<AssetEvent<Level>>>,
    mut items: Query<(Entity, &Kind, &StartPosition)>,
) {
    //let level_handle: Handle<Level> = asset_server.load("assets/level.txt").unwrap();
    //let level_handle: Handle<Level> = asset_server.load_sync(&mut levels, "assets/level.txt").unwrap();
    //let level = levels.get(&level_handle);
    //println!("level: {:?}", level);

    for event in state.reader.iter(&events) {
        let handle = match event {
            AssetEvent::Created { handle } => handle,
            AssetEvent::Modified { handle } => handle,
            _ => continue,
        };
        if let Some(level) = levels.get(handle) {
            println!("level: {:?}", level);
            create_level(&mut commands, &texture_atlas_handle, &mut items, level);
        }
    }
}
