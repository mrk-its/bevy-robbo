use crate::components::prelude::*;
use crate::consts::*;
use crate::frame_cnt::FrameCnt;
use crate::inventory::Inventory;
use crate::levels::LevelInfo;
use bevy::asset::Handle;
use bevy::prelude::*;
use bevy::render::camera::{OrthographicProjection, WindowOrigin};
use bevy::sprite::TextureAtlas;
use std::collections::HashSet;

const TEXTURE_ATLAS_HANDLE: Handle<TextureAtlas> =
    Handle::from_u128(0xfa86671bbf3b4a72a6f36eb2e29432c3);
const DIGITS_ATLAS_HANDLE: Handle<TextureAtlas> =
    Handle::from_u128(0xc5de37f40bcd4614bb544ac824d69f2a);

fn spawn_counter<T>(
    commands: &mut Commands,
    component: T,
    translation: Vec3,
    x_offset: u32,
    n_digits: u32,
    icon_index: u32,
    zoom: f32,
) where
    T: Send + Sync + Copy + 'static,
{
    commands.spawn(SpriteSheetComponents {
        texture_atlas: TEXTURE_ATLAS_HANDLE,
        scale: Scale(zoom),
        translation: Translation(translation + Vec3::new(zoom * (x_offset * 16) as f32, 0.0, 0.0)),
        sprite: TextureAtlasSprite {
            index: icon_index,
            color: Color::rgb_u8(0x40, 0x40, 0x40),
            ..Default::default()
        },
        ..Default::default()
    });
    for k in 0..n_digits {
        commands
            .spawn(SpriteSheetComponents {
                texture_atlas: DIGITS_ATLAS_HANDLE,
                scale: Scale(zoom),
                translation: Translation(translation + Vec3::new(
                    zoom * (((x_offset + n_digits - k - 1) * 16) + 32 - 8) as f32,
                    0.0,
                    0.0,
                )),
                sprite: TextureAtlasSprite {
                    index: 8,
                    color: Color::rgb_u8(0x40, 0x40, 0x40),
                    ..Default::default()
                },
                ..Default::default()
            })
            .with_bundle((component, Digit(k)));
    }
}

fn set_sprite_digit(sprite: &mut TextureAtlasSprite, value: u32, nth: u32) {
    sprite.index = ((value as u32) / (10 as u32).pow(nth)) % 10;
}

fn set_digits<T>(digits_query: &mut Query<With<T, (&Digit, &mut TextureAtlasSprite)>>, value: u32)
where
    T: Send + Sync + 'static,
{
    for (digit, mut sprite) in &mut digits_query.iter() {
        set_sprite_digit(&mut *sprite, value, digit.0);
    }
}

pub fn render_setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    opts: Res<crate::Opts>,
    window: Res<WindowDescriptor>,
    mut clear_color: ResMut<ClearColor>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    *clear_color = ClearColor(Color::rgb_u8(16, 16, 16));

    let texture_handle = asset_server.load("assets/icons32.png").unwrap();
    let digits_handle = asset_server.load("assets/digits2.png").unwrap();
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(384.0, 256.0), 12, 8);
    texture_atlases.set(TEXTURE_ATLAS_HANDLE, texture_atlas);

    let digits_atlas = TextureAtlas::from_grid(digits_handle, Vec2::new(160.0, 32.0), 10, 1);
    texture_atlases.set(DIGITS_ATLAS_HANDLE, digits_atlas);

    let box_size = opts.zoom * 32.0;

    commands.spawn(Camera2dComponents {
        translation: Translation::new(-box_size / 2.0, -box_size / 2.0, 0.0),
        orthographic_projection: OrthographicProjection {
            bottom: 0.0,
            top: MAX_HEIGHT as f32 * box_size / 2.0,
            left: 0.0,
            right: MAX_WIDTH as f32 * box_size / 2.0,
            window_origin: WindowOrigin::BottomLeft,
            ..Default::default()
        },
        ..Default::default()
    });

    let status_width = 23.0 * 16.0 * opts.zoom;
    let translation = Vec3::new((window.width as f32 - status_width) / 2.0, 16.0 * opts.zoom, 0.0);

    spawn_counter(&mut commands, LevelNumber, translation, 0, 3, 71, opts.zoom);
    spawn_counter(&mut commands, ScrewCounter, translation, 6, 3, 83, opts.zoom);
    spawn_counter(&mut commands, KeyCounter, translation, 12, 3, 95, opts.zoom);
    spawn_counter(&mut commands, AmmoCounter, translation, 18, 3, 91, opts.zoom);
}
pub fn update_status_bar(
    level_info: Res<LevelInfo>,
    inventory: Res<Inventory>,
    mut level_digits: Query<With<LevelNumber, (&Digit, &mut TextureAtlasSprite)>>,
    mut screw_digits: Query<With<ScrewCounter, (&Digit, &mut TextureAtlasSprite)>>,
    mut key_digits: Query<With<KeyCounter, (&Digit, &mut TextureAtlasSprite)>>,
    mut ammo_digits: Query<With<AmmoCounter, (&Digit, &mut TextureAtlasSprite)>>,
) {
    let screws_left = (level_info.screws - inventory.screws).max(0);
    set_digits(&mut level_digits, (level_info.current_level + 1) as u32);
    set_digits(&mut screw_digits, screws_left as u32);
    set_digits(&mut key_digits, inventory.keys as u32);
    set_digits(&mut ammo_digits, inventory.bullets as u32);
}

pub fn create_sprites(
    mut commands: Commands,
    opts: Res<crate::Opts>,
    mut missing_sprites: Query<Without<Translation, With<Position, Entity>>>,
) {
    for entity in &mut missing_sprites.iter() {
        commands.insert(
            entity,
            SpriteSheetComponents {
                texture_atlas: TEXTURE_ATLAS_HANDLE,
                scale: Scale(opts.zoom),
                translation: Translation(Vec3::new(-1000.0, -1000.0, 0.0)),
                ..Default::default()
            },
        );
    }
}

pub fn prepare_render(
    frame_cnt: Res<FrameCnt>,
    opts: Res<crate::Opts>,
    mut items: Query<(
        Entity,
        &Position,
        &mut Tiles,
        &mut Translation,
        &mut TextureAtlasSprite,
    )>,
    mut smooth_update_items: Query<Without<RoughUpdate, With<MovingDir, Entity>>>,
    mut smooth_update_items2: Query<Without<RoughUpdate, With<Moveable, Entity>>>,
) {
    let to_smooth_update: HashSet<Entity> = smooth_update_items
        .iter()
        .into_iter()
        .chain(smooth_update_items2.iter().into_iter())
        .collect();
    let trans = Vec3::new(0.0, 2.0 * 32.0 * opts.zoom, 0.0);
    let box_size = opts.zoom * 32.0;
    let min_step = box_size / (KEYFRAME_INTERVAL as f32);
    for (entity, position, tiles, mut translation, mut sprite) in &mut items.iter() {
        let dest = trans + Vec3::new(position.x() as f32, position.y() as f32, 0.0) * box_size;
        if to_smooth_update.contains(&entity) {
            let steps_left = (KEYFRAME_INTERVAL - ((frame_cnt.value()) % KEYFRAME_INTERVAL)) as f32;
            let cur = translation.0;
            let step = (dest - cur) / steps_left;
            if step.x().abs() > 0.01 || step.y().abs() > 0.01 {
                translation.0 = if step.x().abs() <= min_step && step.y().abs() <= min_step {
                    cur + step
                } else {
                    dest
                };
            }
        } else {
            translation.0 = dest;
        }
        sprite.index = tiles.tiles[tiles.current];
    }
}
