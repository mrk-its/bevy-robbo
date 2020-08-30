use crate::components::{Destroyable, Int2Ops, Kind, Moveable, MovingDir, Robbo, Tiles, Collectable};
use bevy::ecs::{Commands, DynamicBundle};


pub fn robbo<'a>(commands: &'a mut Commands) -> &'a mut Commands {
    commands.spawn((
        Robbo,
        Kind::Robbo,
        MovingDir::zero(),
        Tiles::new(&[60, 61]),
    ))
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
    commands.spawn((
        Kind::LBear,
        Destroyable,
        Tiles::new(&[13, 14]),
    ))
}

pub fn rbear<'a>(commands: &'a mut Commands) -> &'a mut Commands {
    commands.spawn((
        Kind::RBear,
        Destroyable,
        Tiles::new(&[30, 31]),
    ))
}

pub fn push_box<'a>(commands: &'a mut Commands) -> &'a mut Commands {
    commands.spawn((
        Kind::MovingBox,
        Moveable,
        Tiles::new(&[6]),
    ))
}

pub fn static_box<'a>(commands: &'a mut Commands) -> &'a mut Commands {
    commands.spawn((Kind::Box, Moveable, Tiles::new(&[20])))
}

pub fn wall<'a>(commands: &'a mut Commands, k: usize) -> &'a mut Commands {
    let tiles = &[&[2], &[29], &[19], &[3], &[21], &[68], &[69], &[10], &[22]];
    commands.spawn((
        Kind::Wall,
        Tiles::new(tiles[k]),
    ))
}

pub fn bullet<'a>(commands: &'a mut Commands, kx: i32, ky: i32) -> &'a mut Commands {
    commands.spawn((
        Kind::Bullet,
        Moveable,
        MovingDir::new(kx, ky),
        Tiles::new(if ky == 0 { &[36, 37] } else { &[38, 39] }),
    ))
}

pub fn screw<'a>(commands: &'a mut Commands) -> &'a mut Commands {
    commands.spawn((
        Kind::Screw,
        Collectable::Screw,
        Tiles::new(&[4]),
    ))
}

pub fn ammo<'a>(commands: &'a mut Commands) -> &'a mut Commands {
    commands.spawn((
        Kind::Ammo,
        Collectable::Ammo,
        Tiles::new(&[5])
    ))
}

pub fn key<'a>(commands: &'a mut Commands) -> &'a mut Commands {
    commands.spawn((
        Kind::Key,
        Collectable::Key,
        Tiles::new(&[42])
    ))
}

pub fn ground<'a>(commands: &'a mut Commands) -> &'a mut Commands {
    commands.spawn((
        Kind::Ground,
        Destroyable,
        Tiles::new(&[77]),
    ))
}

pub fn capsule<'a>(commands: &'a mut Commands) -> &'a mut Commands {
    commands.spawn((
        Kind::Capsule,
        Moveable,
        Tiles::new(&[17, 18]),
    ))
}

pub fn bomb<'a>(commands: &'a mut Commands) -> &'a mut Commands {
    commands.spawn((
        Kind::Bomb,
        Moveable,
        Tiles::new(&[8]),
    ))
}

pub fn questionmark<'a>(commands: &'a mut Commands) -> &'a mut Commands {
    commands.spawn((
        Kind::Questionmark,
        Moveable,
        Tiles::new(&[12]),
    ))
}

pub fn door<'a>(commands: &'a mut Commands) -> &'a mut Commands {
    commands.spawn((
        Kind::Door,
        Tiles::new(&[9]),
    ))
}

pub fn teleport<'a>(commands: &'a mut Commands) -> &'a mut Commands {
    commands.spawn((
        Kind::Teleport,
        Tiles::new(&[48, 49]),
    ))
}

pub fn eyes<'a>(commands: &'a mut Commands) -> &'a mut Commands {
    commands.spawn((
        Kind::Eyes,
        Destroyable,
        Tiles::new(&[32, 33]),
    ))
}

pub fn gun<'a>(commands: &'a mut Commands) -> &'a mut Commands {
    commands.spawn((
        Kind::Gun,
        Tiles::new(&[53, 54, 55, 56]),
    ))
}

pub fn magnet<'a>(commands: &'a mut Commands) -> &'a mut Commands {
    commands.spawn((
        Kind::Magnet,
        Tiles::new(&[0, 72, 1, 73]),
    ))
}

pub fn forcefield<'a>(commands: &'a mut Commands) -> &'a mut Commands {
    commands.spawn((
        Kind::ForceField,
        Destroyable,
        Tiles::new(&[45, 57]),
    ))
}
