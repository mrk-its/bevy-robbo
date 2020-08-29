use crate::components::{Destroyable, Int2Ops, Kind, Moveable, MovingDir, Robbo, Tiles};
use bevy::ecs::DynamicBundle;

pub fn robbo() -> impl DynamicBundle + Send + Sync + 'static {
    (
        Robbo,
        Kind::Robbo,
        // Position::new(kx, ky),
        MovingDir::zero(),
        Tiles::new(&[60, 61]),
    )
}

pub fn bird() -> impl DynamicBundle + Send + Sync + 'static {
    (
        Kind::Bird,
        MovingDir::new(0, 1),
        Destroyable,
        Tiles::new(&[15, 16]),
    )
}

pub fn lbear(kx: i32, ky: i32) -> impl DynamicBundle + Send + Sync + 'static {
    (
        Kind::LBear,
        MovingDir::new(kx, ky),
        Destroyable,
        Tiles::new(&[13, 14]),
    )
}

pub fn rbear(kx: i32, ky: i32) -> impl DynamicBundle + Send + Sync + 'static {
    (
        Kind::RBear,
        MovingDir::new(kx, ky),
        Destroyable,
        Tiles::new(&[30, 31]),
    )
}

pub fn push_box() -> impl DynamicBundle + Send + Sync + 'static {
    (
        Kind::MovingBox,
        Moveable,
        MovingDir::zero(),
        Tiles::new(&[6]),
    )
}

pub fn static_box() -> impl DynamicBundle + Send + Sync + 'static {
    (Kind::Box, Moveable, Tiles::new(&[20]))
}

pub fn wall() -> impl DynamicBundle + Send + Sync + 'static {
    (
        Kind::Wall,
        Tiles::new(&[3]),
    )
}

pub fn _bullet(kx: i32, ky: i32) -> impl DynamicBundle + Send + Sync + 'static {
    (
        Kind::Bullet,
        Moveable,
        MovingDir::new(kx, ky),
        Tiles::new(if ky == 0 { &[36, 37] } else { &[38, 39] }),
    )
}
