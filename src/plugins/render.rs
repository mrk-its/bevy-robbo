use crate::components::prelude::*;
use crate::consts::*;
use crate::frame_cnt::FrameCnt;
use crate::inventory::Inventory;
use crate::levels::LevelInfo;
use bevy::render::camera::{OrthographicProjection, WindowOrigin};
use bevy::sprite::TextureAtlas;
use bevy::window::WindowResized;
use bevy::{prelude::*, reflect::TypeUuid};
use std::collections::HashSet;

const TEXTURE_ATLAS_HANDLE: HandleUntyped =
    HandleUntyped::weak_from_u64(TextureAtlas::TYPE_UUID, 0);
    
const DIGITS_ATLAS_HANDLE: HandleUntyped = 
    HandleUntyped::weak_from_u64(TextureAtlas::TYPE_UUID, 1);

#[derive(Default)]
pub struct RenderState {
    pub reader: EventReader<WindowResized>,
}

fn camera_scale(width: u32, height: u32) -> f32 {
    let scale_x = (MAX_BOARD_WIDTH as f32 * 32.0) / (width as f32);
    let scale_y = ((MAX_BOARD_HEIGHT + 2) as f32 * 32.0) / (height as f32);
    scale_x.max(scale_y)
}

fn camera_translation(width: u32, height: u32) -> Vec3 {
    let scale = camera_scale(width, height);
    let board_width = MAX_BOARD_WIDTH as f32 * 32.0;
    let board_height = (MAX_BOARD_HEIGHT + 2) as f32 * 32.0;
    Vec3::new(
        -16.0 - (width as f32 * scale - board_width) / 2.0,
        -16.0 - (height as f32 * scale - board_height) / 2.0,
        0.0,
    )
}

fn spawn_counter<T>(
    commands: &mut Commands,
    component: T,
    x_offset: u32,
    n_digits: u32,
    icon_index: u32,
) where
    T: Send + Sync + Copy + 'static,
{
    let color = Color::rgb(0.8, 0.8, 0.8);
    commands
        .spawn(SpriteSheetBundle {
            texture_atlas: TEXTURE_ATLAS_HANDLE.typed(),
            transform: Transform::from_translation(Vec3::new(x_offset as f32 * 16.0, 16.0, 0.0)),
            sprite: TextureAtlasSprite {
                index: icon_index,
                color: color,
                ..Default::default()
            },
            ..Default::default()
        })
        .with(StatusOffset(x_offset));

    for k in 0..n_digits {
        commands
            .spawn(SpriteSheetBundle {
                texture_atlas: DIGITS_ATLAS_HANDLE.typed(),
                transform: Transform::from_translation(Vec3::new(
                    (((x_offset + n_digits - k - 1) * 16) + 32 - 8) as f32,
                    16.0,
                    0.0,
                )),
                sprite: TextureAtlasSprite {
                    index: 8,
                    color: color,
                    ..Default::default()
                },
                ..Default::default()
            })
            .with_bundle((
                component,
                Digit(k),
                StatusOffset(x_offset + n_digits - k + 1),
            ));
    }
}

fn set_sprite_digit(sprite: &mut TextureAtlasSprite, value: u32, nth: u32) {
    sprite.index = ((value as u32) / (10 as u32).pow(nth)) % 10;
}

fn set_digits<T>(digits_query: &mut Query<(&Digit, &mut TextureAtlasSprite), With<T>>, value: u32)
where
    T: Send + Sync + 'static,
{
    for (digit, mut sprite) in digits_query.iter_mut() {
        set_sprite_digit(&mut *sprite, value, digit.0);
    }
}

pub fn render_setup(
    commands: &mut Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load("icons32.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(32.0, 32.0), 12, 8);
    texture_atlases.set_untracked(TEXTURE_ATLAS_HANDLE, texture_atlas);

    let box_size = 32.0;
    let width = MAX_BOARD_WIDTH as f32 * box_size;
    let height = (MAX_BOARD_HEIGHT + 2) as f32 * box_size;
    let scale = camera_scale(width as u32, height as u32);
    let translation = camera_translation(width as u32, height as u32);
    commands.spawn(Camera2dBundle {
        orthographic_projection: OrthographicProjection {
            bottom: 0.0,
            top: MAX_BOARD_HEIGHT as f32 * box_size,
            left: 0.0,
            right: MAX_BOARD_WIDTH as f32 * box_size,
            window_origin: WindowOrigin::BottomLeft,
            ..Default::default()
        },
        transform: Transform::from_translation(translation)
            .mul_transform(Transform::from_scale(Vec3::new(scale, scale, scale))),
        ..Default::default()
    });

    {
        let digits_handle = asset_server.load::<Texture, _>("digits2.png");
        let digits_atlas = TextureAtlas::from_grid(digits_handle, Vec2::new(16.0, 32.0), 10, 1);
        texture_atlases.set_untracked(DIGITS_ATLAS_HANDLE, digits_atlas);

        let offs = (62 - 22) / 2;
        spawn_counter(commands, ScrewCounter, offs + 0, 2, 83);
        spawn_counter(commands, KeyCounter, offs + 6, 2, 95);
        spawn_counter(commands, AmmoCounter, offs + 12, 2, 91);
        spawn_counter(commands, LevelNumber, offs + 18, 2, 71);
    }
}

pub fn update_camera(
    mut state: ResMut<RenderState>,
    events: Res<Events<WindowResized>>,
    mut items: Query<(&mut Transform, &OrthographicProjection)>,
) {
    let event: Option<WindowResized> = state.reader.iter(&events).cloned().last();
    if let Some(event) = event {
        for (mut transform, _) in items.iter_mut() {
            let scale = camera_scale(event.width as u32, event.height as u32);
            let translation = camera_translation(event.width as u32, event.height as u32);
            *transform = Transform::from_translation(translation)
                .mul_transform(Transform::from_scale(Vec3::new(scale, scale, scale)));
        }
    }
}

pub fn update_status_bar(
    level_info: Res<LevelInfo>,
    inventory: Res<Inventory>,
    mut level_digits: Query<(&Digit, &mut TextureAtlasSprite), With<LevelNumber>>,
    mut screw_digits: Query<(&Digit, &mut TextureAtlasSprite), With<ScrewCounter>>,
    mut key_digits: Query<(&Digit, &mut TextureAtlasSprite), With<KeyCounter>>,
    mut ammo_digits: Query<(&Digit, &mut TextureAtlasSprite), With<AmmoCounter>>,
) {
    let screws_left = (level_info.screws - inventory.screws).max(0);
    set_digits(&mut level_digits, (level_info.current_level + 1) as u32);
    set_digits(&mut screw_digits, screws_left as u32);
    set_digits(&mut key_digits, inventory.keys as u32);
    set_digits(&mut ammo_digits, inventory.bullets as u32);
}

pub fn create_sprites(
    commands: &mut Commands,
    missing_sprites: Query<Entity, (Without<Transform>, With<Position>)>,
) {
    for entity in missing_sprites.iter() {
        commands.insert(
            entity,
            SpriteSheetBundle {
                texture_atlas: TEXTURE_ATLAS_HANDLE.typed(),
                transform: Transform::from_translation(Vec3::new(-1000.0, -1000.0, 0.0)),
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
        &mut Transform,
        &mut TextureAtlasSprite,
    )>,
    smooth_update_items: Query<Entity, (Without<RoughUpdate>, With<MovingDir>)>,
    smooth_update_items2: Query<Entity, (Without<RoughUpdate>, With<Moveable>)>,
) {
    let to_smooth_update: HashSet<Entity> = smooth_update_items
        .iter()
        .chain(smooth_update_items2.iter().into_iter())
        .collect();
    let box_size = 32.0;
    let min_step = box_size / (opts.key_frame_interval as f32) * 1.01;
    let trans = Vec3::new(0.0, 2.0 * box_size, 0.0);
    for (entity, position, tiles, mut transform, mut sprite) in items.iter_mut() {
        let dest = trans + Vec3::new(position.x() as f32, position.y() as f32, 0.0) * box_size;
        let cur = transform.translation;
        if cur != dest {
            if to_smooth_update.contains(&entity) {
                let steps_left = (opts.key_frame_interval
                    - ((frame_cnt.value()) % opts.key_frame_interval))
                    as f32;
                let step = (dest - cur) / steps_left;
                if step.x.abs() > 0.01 || step.y.abs() > 0.01 {
                    let dest = if step.x.abs() <= min_step && step.y.abs() <= min_step {
                        cur + step
                    } else {
                        dest
                    };
                    *transform = Transform::from_translation(dest);
                }
            } else {
                *transform = Transform::from_translation(dest);
            }
        }
        let sprite_index = tiles.tiles[tiles.current];
        if sprite.index != sprite_index {
            sprite.index = sprite_index;
        }
    }
}

pub struct RenderPlugin {
    pub vsync: bool,
}

impl Plugin for RenderPlugin {
    fn build(&self, app: &mut AppBuilder) {
        let builder = app
            .add_resource(bevy::render::pass::ClearColor(Color::rgb(0.3, 0.3, 0.5)))
            .add_resource(RenderState::default())
            .add_startup_system(render_setup.system())
            .add_stage_before(
                stage::POST_UPDATE,
                "create_sprites",
                SystemStage::parallel(),
            )
            .add_stage_before(stage::POST_UPDATE, "update_camera", SystemStage::parallel())
            .add_stage_before(
                stage::POST_UPDATE,
                "prepare_render",
                SystemStage::parallel(),
            )
            .add_system_to_stage("create_sprites", create_sprites.system())
            .add_system_to_stage("update_camera", update_camera.system())
            .add_system_to_stage("prepare_render", prepare_render.system());

        builder.add_system_to_stage("prepare_render", update_status_bar.system());
    }
}
