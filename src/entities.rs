use crate::components::{
    Bomb, Collectable, Destroyable, GunType, Int2Ops, Kind, LaserTail, Moveable, MovingDir, Robbo,
    ShootingDir, Tiles, Usable, Wall,
};
use bevy::ecs::Commands;

pub fn robbo<'a>(commands: &'a mut Commands) -> &'a mut Commands {
    commands.spawn((Robbo, Kind::Robbo, MovingDir::zero(), Tiles::new(&[60])))
}

pub fn bird<'a>(commands: &'a mut Commands) -> &'a mut Commands {
    commands.spawn((
        Kind::Bird,
        MovingDir::new(0, 1),
        Destroyable,
        Tiles::new(&[15, 16]),
    ))
}

pub fn lbear<'a>(commands: &'a mut Commands) -> &'a mut Commands {
    commands.spawn((Kind::Bear(true), Destroyable, Tiles::new(&[13, 14])))
}

pub fn rbear<'a>(commands: &'a mut Commands) -> &'a mut Commands {
    commands.spawn((Kind::Bear(false), Destroyable, Tiles::new(&[30, 31])))
}

pub fn push_box<'a>(commands: &'a mut Commands) -> &'a mut Commands {
    commands.spawn((
        Kind::MovingBox,
        Moveable,
        MovingDir::zero(),
        Tiles::new(&[6]),
    ))
}

pub fn static_box<'a>(commands: &'a mut Commands) -> &'a mut Commands {
    commands.spawn((Kind::Box, Moveable, Tiles::new(&[20])))
}

pub fn wall<'a>(commands: &'a mut Commands, k: usize) -> &'a mut Commands {
    let tiles = &[&[2], &[29], &[19], &[3], &[21], &[68], &[69], &[10], &[22]];
    commands.spawn((Kind::Wall, Wall, Tiles::new(tiles[k])))
}

pub fn bullet<'a>(commands: &'a mut Commands, kx: i32, ky: i32) -> &'a mut Commands {
    commands.spawn((
        Kind::Bullet,
        MovingDir::new(kx, ky),
        Tiles::new(if ky == 0 { &[36, 37] } else { &[38, 39] }),
    ))
}

pub fn laser_head<'a>(commands: &'a mut Commands, kx: i32, ky: i32) -> &'a mut Commands {
    commands.spawn((
        Kind::LaserHead { moving_back: false },
        MovingDir::new(kx, ky),
        Tiles::new(if ky == 0 { &[36, 37] } else { &[38, 39] }),
    ))
}

pub fn laser_tail<'a>(commands: &'a mut Commands, kx: i32, ky: i32) -> &'a mut Commands {
    commands.spawn((
        Kind::LaserTail,
        LaserTail,
        Tiles::new(if ky == 0 { &[36, 37] } else { &[38, 39] }),
    ))
}

pub fn screw<'a>(commands: &'a mut Commands) -> &'a mut Commands {
    commands.spawn((Kind::Screw, Collectable::Screw, Tiles::new(&[4])))
}

pub fn ammo<'a>(commands: &'a mut Commands) -> &'a mut Commands {
    commands.spawn((Kind::Ammo, Collectable::Ammo, Tiles::new(&[5])))
}

pub fn key<'a>(commands: &'a mut Commands) -> &'a mut Commands {
    commands.spawn((Kind::Key, Collectable::Key, Tiles::new(&[42])))
}

pub fn ground<'a>(commands: &'a mut Commands) -> &'a mut Commands {
    commands.spawn((Kind::Ground, Destroyable, Tiles::new(&[77])))
}

pub fn capsule<'a>(commands: &'a mut Commands) -> &'a mut Commands {
    commands.spawn((Kind::Capsule, Moveable, Tiles::new(&[17, 18])))
}

pub fn bomb<'a>(commands: &'a mut Commands) -> &'a mut Commands {
    commands.spawn((Kind::Bomb, Bomb, Moveable, Destroyable, Tiles::new(&[8])))
}

pub fn questionmark<'a>(commands: &'a mut Commands) -> &'a mut Commands {
    commands.spawn((Kind::Questionmark, Destroyable, Moveable, Tiles::new(&[12])))
}

pub fn door<'a>(commands: &'a mut Commands) -> &'a mut Commands {
    commands.spawn((Kind::Door, Usable, Tiles::new(&[9])))
}

pub fn teleport<'a>(commands: &'a mut Commands) -> &'a mut Commands {
    commands.spawn((Kind::Teleport, Tiles::new(&[48, 49])))
}

pub fn eyes<'a>(commands: &'a mut Commands) -> &'a mut Commands {
    commands.spawn((Kind::Eyes, Destroyable, Tiles::new(&[32, 33])))
}

const GUN_TILES: &[u32] = &[56, 53, 54, 55];

pub fn gun<'a>(commands: &'a mut Commands, params: &[u16]) -> &'a mut Commands {
    let index = params[0] as usize;
    let is_moveable = params[3] > 0;
    let is_rotateable = *params.get(4).unwrap_or(&0) > 0;
    let is_random_rotatable = *params.get(5).unwrap_or(&0) > 0;
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
        commands.with(MovingDir::by_index(params[1] as usize));
    }
    commands
}

pub fn horizontal_laser<'a>(commands: &'a mut Commands, params: &[u16]) -> &'a mut Commands {
    commands.spawn((Kind::Gun, Tiles::new(&[53, 55])))
}
pub fn vertical_laser<'a>(commands: &'a mut Commands, params: &[u16]) -> &'a mut Commands {
    commands.spawn((Kind::Gun, Tiles::new(&[54, 56])))
}

pub fn magnet<'a>(commands: &'a mut Commands) -> &'a mut Commands {
    commands.spawn((Kind::Magnet, Tiles::new(&[0, 72, 1, 73])))
}

pub fn forcefield<'a>(commands: &'a mut Commands) -> &'a mut Commands {
    commands.spawn((Kind::ForceField, Destroyable, Tiles::new(&[45, 57])))
}
