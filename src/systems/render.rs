use crate::components::{Int2Ops, Position, Tiles};
use crate::consts::*;
use crate::frame_cnt::FrameCnt;
use bevy::asset::Handle;
use bevy::prelude::*;
use bevy::render::camera::{OrthographicProjection, WindowOrigin};
use bevy::sprite::TextureAtlas;

const TEXTURE_ATLAS_HANDLE: Handle<TextureAtlas> =
    Handle::from_u128(0xfa86671bbf3b4a72a6f36eb2e29432c3);

pub fn render_setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load("assets/icons32.png").unwrap();
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(384.0, 256.0), 12, 8);
    texture_atlases.set(TEXTURE_ATLAS_HANDLE, texture_atlas);

    commands.spawn(Camera2dComponents {
        translation: Translation::new(-BOX_SIZE / 2.0, -BOX_SIZE / 2.0, 0.0),
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

pub fn create_sprites(
    mut commands: Commands,
    mut missing_sprites: Query<Without<Translation, With<Position, Entity>>>,
) {
    for entity in &mut missing_sprites.iter() {
        commands.insert(
            entity,
            SpriteSheetComponents {
                texture_atlas: TEXTURE_ATLAS_HANDLE,
                scale: Scale(SCALE),
                translation: Translation(Vec3::new(-1000.0, -1000.0, 0.0)),
                ..Default::default()
            },
        );
    }
}

pub fn prepare_render(
    frame_cnt: Res<FrameCnt>,
    mut items: Query<(
        &Position,
        &mut Tiles,
        &mut Translation,
        &mut TextureAtlasSprite,
    )>,
    // position: &Position,
    // mut tiles: Mut<Tiles>,
    // mut translation: Mut<Translation>,
    // mut sprite: Mut<TextureAtlasSprite>,
) {
    const STEPS: usize = 4;
    const MIN_STEP: f32 = BOX_SIZE / (STEPS as f32);

    for (position, mut tiles, mut translation, mut sprite) in &mut items.iter() {
        let steps_left = (STEPS - ((frame_cnt.value()) % STEPS)) as f32;
        let dest = Vec3::new(position.x() as f32, (position.y()) as f32, 0.0) * BOX_SIZE;
        let cur = translation.0;
        let step = (dest - cur) / steps_left;
        if step.x().abs() > 0.01 || step.y().abs() > 0.01 {
            translation.0 = if step.x().abs() <= MIN_STEP && step.y().abs() <= MIN_STEP {
                cur + step
            } else {
                dest
            };
        }
        sprite.index = tiles.tiles[tiles.current];
        if frame_cnt.do_it() {
            tiles.current = (tiles.current + 1) % tiles.tiles.len();
        }
    }
}
