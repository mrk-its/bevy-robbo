use crate::components::{
    Bomb, Capsule, Collectable, Destroyable, GunType, Int2Ops, Kind, LaserTail, Moveable,
    MovingDir, Robbo, ShootingDir, Teleport, Tiles, Usable, Wall,
};
use bevy::ecs::*;

pub fn create_robbo<'a>(commands: &'a mut Commands) -> &'a mut Commands {
    commands.spawn((Robbo, Kind::Robbo, MovingDir::zero(), Tiles::new(&[60])))
}

pub fn create_bird<'a>(commands: &'a mut Commands) -> &'a mut Commands {
    commands.spawn((
        Kind::Bird,
        MovingDir::new(0, 1),
        Destroyable,
        Tiles::new(&[15, 16]),
    ))
}

pub fn create_lbear<'a>(commands: &'a mut Commands) -> &'a mut Commands {
    commands.spawn((Kind::Bear(true), Destroyable, Tiles::new(&[13, 14])))
}

pub fn create_rbear<'a>(commands: &'a mut Commands) -> &'a mut Commands {
    commands.spawn((Kind::Bear(false), Destroyable, Tiles::new(&[30, 31])))
}

pub fn create_push_box<'a>(commands: &'a mut Commands) -> &'a mut Commands {
    commands.spawn((
        Kind::MovingBox,
        Moveable,
        MovingDir::zero(),
        Tiles::new(&[6]),
    ))
}

pub fn create_static_box<'a>(commands: &'a mut Commands) -> &'a mut Commands {
    commands.spawn((Kind::Box, Moveable, Tiles::new(&[20])))
}

pub fn create_wall<'a>(commands: &'a mut Commands, k: usize) -> &'a mut Commands {
    let tiles = &[&[2], &[29], &[19], &[3], &[21], &[68], &[69], &[10], &[22]];
    commands.spawn((Kind::Wall, Wall, Tiles::new(tiles[k])))
}

pub fn create_bullet<'a>(commands: &'a mut Commands, kx: i32, ky: i32) -> &'a mut Commands {
    commands.spawn((
        Kind::Bullet,
        MovingDir::new(kx, ky),
        Tiles::new(if ky == 0 { &[36, 37] } else { &[38, 39] }),
    ))
}

pub fn create_laser_head<'a>(commands: &'a mut Commands, kx: i32, ky: i32) -> &'a mut Commands {
    commands.spawn((
        Kind::LaserHead { moving_back: false },
        MovingDir::new(kx, ky),
        Tiles::new(if ky == 0 { &[36, 37] } else { &[38, 39] }),
    ))
}

pub fn create_laser_tail<'a>(commands: &'a mut Commands, _kx: i32, ky: i32) -> &'a mut Commands {
    commands.spawn((
        Kind::LaserTail,
        LaserTail,
        Tiles::new(if ky == 0 { &[36, 37] } else { &[38, 39] }),
    ))
}

pub fn create_screw<'a>(commands: &'a mut Commands) -> &'a mut Commands {
    commands.spawn((Kind::Screw, Collectable::Screw, Tiles::new(&[4])))
}

pub fn create_ammo<'a>(commands: &'a mut Commands) -> &'a mut Commands {
    commands.spawn((Kind::Ammo, Collectable::Ammo, Tiles::new(&[5])))
}

pub fn create_key<'a>(commands: &'a mut Commands) -> &'a mut Commands {
    commands.spawn((Kind::Key, Collectable::Key, Tiles::new(&[42])))
}

pub fn create_ground<'a>(commands: &'a mut Commands) -> &'a mut Commands {
    commands.spawn((Kind::Ground, Destroyable, Tiles::new(&[77])))
}

pub fn create_capsule<'a>(commands: &'a mut Commands) -> &'a mut Commands {
    commands.spawn((Kind::Capsule, Capsule, Moveable, Tiles::new(&[18])))
}

pub fn repair_capsule<'a>(commands: &'a mut Commands, entity: Entity) -> &'a mut Commands {
    commands.remove_one::<Moveable>(entity);
    commands.insert(entity, (Usable, Tiles::new(&[17, 18])))
}

pub fn create_bomb<'a>(commands: &'a mut Commands) -> &'a mut Commands {
    commands.spawn((Kind::Bomb, Bomb, Moveable, Destroyable, Tiles::new(&[8])))
}

pub fn create_questionmark<'a>(commands: &'a mut Commands) -> &'a mut Commands {
    commands.spawn((Kind::Questionmark, Destroyable, Moveable, Tiles::new(&[12])))
}

pub fn create_door<'a>(commands: &'a mut Commands) -> &'a mut Commands {
    commands.spawn((Kind::Door, Usable, Tiles::new(&[9])))
}

pub fn create_teleport<'a>(commands: &'a mut Commands, params: &[usize]) -> &'a mut Commands {
    commands.spawn((
        Kind::Teleport,
        Teleport(params[0], params[1]),
        Usable,
        Tiles::new(&[48, 49]),
    ))
}

pub fn create_eyes<'a>(commands: &'a mut Commands) -> &'a mut Commands {
    commands.spawn((Kind::Eyes, Destroyable, Tiles::new(&[32, 33])))
}

const GUN_TILES: &[u32] = &[56, 53, 54, 55];

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
    commands.spawn((Kind::Gun, Tiles::new(&GUN_TILES[index..index + 1])));
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
    commands.spawn((Kind::Gun, Tiles::new(&[53, 55])))
}
pub fn _create_vertical_laser<'a>(
    commands: &'a mut Commands,
    _params: &[usize],
) -> &'a mut Commands {
    commands.spawn((Kind::Gun, Tiles::new(&[54, 56])))
}

pub fn create_magnet<'a>(commands: &'a mut Commands) -> &'a mut Commands {
    commands.spawn((Kind::Magnet, Tiles::new(&[0, 72, 1, 73])))
}

pub fn create_forcefield<'a>(commands: &'a mut Commands) -> &'a mut Commands {
    commands.spawn((Kind::ForceField, Destroyable, Tiles::new(&[45, 57])))
}
