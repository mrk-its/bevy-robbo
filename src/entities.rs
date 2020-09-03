use crate::components::prelude::*;
use bevy::ecs::*;

pub fn create_robbo<'a>(commands: &'a mut Commands) -> &'a mut Commands {
    commands.spawn((Robbo, MovingDir::zero(), Tiles::new(&[60])))
}

pub fn create_bird<'a>(commands: &'a mut Commands) -> &'a mut Commands {
    commands.spawn((
        Bird,
        MovingDir::new(0, 1),
        Destroyable,
        Tiles::new(&[15, 16]),
    ))
}

pub fn create_bear<'a>(commands: &'a mut Commands) -> &'a mut Commands {
    commands.spawn((Bear(false), Destroyable, Tiles::new(&[13, 14])))
}

pub fn create_black_bear<'a>(commands: &'a mut Commands) -> &'a mut Commands {
    commands.spawn((Bear(true), Destroyable, Tiles::new(&[30, 31])))
}

pub fn create_push_box<'a>(commands: &'a mut Commands) -> &'a mut Commands {
    commands.spawn((
        PushBox,
        Moveable,
        MovingDir::zero(),
        Tiles::new(&[6]),
    ))
}

pub fn create_static_box<'a>(commands: &'a mut Commands) -> &'a mut Commands {
    commands.spawn((Moveable, Tiles::new(&[20])))
}

pub fn create_wall<'a>(commands: &'a mut Commands, k: usize) -> &'a mut Commands {
    let tiles = &[&[2], &[29], &[19], &[3], &[21], &[68], &[69], &[10], &[22]];
    commands.spawn((Undestroyable, Wall, Tiles::new(tiles[k])))
}

static BULLET_H_TILES: &[u32] = &[36, 36, 36, 36, 37, 37, 37, 37];
static BULLET_V_TILES: &[u32] = &[38, 38, 38, 38, 39, 39, 39, 39];

pub fn create_bullet<'a>(commands: &'a mut Commands, kx: i32, ky: i32) -> &'a mut Commands {
    commands.spawn((
        Bullet,
        Undestroyable,
        MovingDir::new(kx, ky),
        Tiles::new(if ky == 0 { BULLET_H_TILES } else { BULLET_V_TILES }),
    ))
}

pub fn create_laser_head<'a>(commands: &'a mut Commands, kx: i32, ky: i32) -> &'a mut Commands {
    commands.spawn((
        LaserHead::default(),
        RoughUpdate,
        Undestroyable,
        MovingDir::new(kx, ky),
        Tiles::new(if ky == 0 { BULLET_H_TILES } else { BULLET_V_TILES }),
    ))
}

pub fn create_blaster_head<'a>(commands: &'a mut Commands, kx: i32, ky: i32) -> &'a mut Commands {
    commands.spawn((
        BlasterHead,
        Undestroyable,
        RoughUpdate,
        MovingDir::new(kx, ky),
        Tiles::new(if ky == 0 { BULLET_H_TILES } else { BULLET_V_TILES }),
    ))
}

pub fn create_laser_tail<'a>(commands: &'a mut Commands, _kx: i32, ky: i32) -> &'a mut Commands {
    commands.spawn((
        LaserTail,
        Undestroyable,
        Tiles::new(if ky == 0 { BULLET_H_TILES } else { BULLET_V_TILES }),
    ))
}

pub fn create_screw<'a>(commands: &'a mut Commands) -> &'a mut Commands {
    commands.spawn((Collectable::Screw, Tiles::new(&[4])))
}

pub fn create_ammo<'a>(commands: &'a mut Commands) -> &'a mut Commands {
    commands.spawn((Collectable::Ammo, Tiles::new(&[5])))
}

pub fn create_key<'a>(commands: &'a mut Commands) -> &'a mut Commands {
    commands.spawn((Collectable::Key, Tiles::new(&[42])))
}

pub fn create_ground<'a>(commands: &'a mut Commands) -> &'a mut Commands {
    commands.spawn((Destroyable, Tiles::new(&[77])))
}

pub fn create_capsule<'a>(commands: &'a mut Commands) -> &'a mut Commands {
    commands.spawn((Capsule, Moveable, Tiles::new(&[18])))
}

pub fn repair_capsule<'a>(commands: &'a mut Commands, entity: Entity) -> &'a mut Commands {
    commands.remove_one::<Moveable>(entity);
    commands.insert(entity, (Usable::Capsule, Tiles::new(&[17, 18])))
}

pub fn create_bomb<'a>(commands: &'a mut Commands) -> &'a mut Commands {
    commands.spawn((Bomb, Moveable, Destroyable, Tiles::new(&[8])))
}

pub fn create_explosion<'a>(commands: &'a mut Commands) -> &'a mut Commands {
    commands.spawn((Animation, Undestroyable, Tiles::new(&[52, 51, 50])))
}

pub fn create_questionmark<'a>(commands: &'a mut Commands) -> &'a mut Commands {
    commands.spawn((Destroyable, Moveable, Tiles::new(&[12])))
}

pub fn create_door<'a>(commands: &'a mut Commands) -> &'a mut Commands {
    commands.spawn((Usable::Door, Tiles::new(&[9])))
}

pub fn create_teleport<'a>(commands: &'a mut Commands, params: &[usize]) -> &'a mut Commands {
    commands.spawn((
        Teleport(params[0], params[1]),
        Usable::Teleport,
        Tiles::new(&[48, 49]),
    ))
}
pub fn create_eyes<'a>(commands: &'a mut Commands) -> &'a mut Commands {
    commands.spawn((Destroyable, Tiles::new(&[32, 33])))
}

static GUN_TILES: &[u32] = &[56, 53, 54, 55];

pub fn create_gun<'a>(commands: &'a mut Commands, params: &[usize]) -> &'a mut Commands {
    let index = params[0];
    let is_moveable = params[3] > 0;
    let _is_rotateable = *params.get(4).unwrap_or(&0) > 0;
    let _is_random_rotatable = *params.get(5).unwrap_or(&0) > 0;
    let gun_type = match params[2] {
        1 => GunType::Solid,
        2 => GunType::Blaster,
        _ => GunType::Burst,
    };
    commands.spawn((Tiles::new(&GUN_TILES[index..index + 1]), ));
    commands.with(
        ShootingDir::by_index(index)
            .with_propability(0.05)
            .with_gun_type(gun_type),
    );
    if is_moveable {
        commands.with(Moveable);
        commands.with(MovingDir::by_index(params[1]));
    }
    commands
}

pub fn _create_horizontal_laser<'a>(
    commands: &'a mut Commands,
    _params: &[usize],
) -> &'a mut Commands {
    commands.spawn((Tiles::new(&[53, 55]), ))
}
pub fn _create_vertical_laser<'a>(
    commands: &'a mut Commands,
    _params: &[usize],
) -> &'a mut Commands {
    commands.spawn((Tiles::new(&[54, 56]), ))
}

const MAGNET_TILES: &[u32] = &[73, 0, 72, 1];

pub fn create_magnet<'a>(commands: &'a mut Commands, index: usize) -> &'a mut Commands {
    commands.spawn((Magnet::by_index(index), Tiles::new(&MAGNET_TILES[index..index + 1]), ))
}

pub fn create_forcefield<'a>(commands: &'a mut Commands, _index: usize) -> &'a mut Commands {
    commands.spawn((Destroyable, ForceField, Tiles::new(&[45, 57])))
}
