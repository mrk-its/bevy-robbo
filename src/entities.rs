use crate::components::{Destroyable, Int2Ops, Kind, MovingDir, Position, Robbo, Tiles, Moveable};
use bevy::ecs::DynamicBundle;

pub fn robbo(kx: i32, ky: i32) -> impl DynamicBundle + Send + Sync + 'static {
    (
        Robbo,
        Kind::Robbo,
        Position::new(kx, ky),
        MovingDir::zero(),
        Tiles::new(&[60, 61]),
    )
}

pub fn bird(x: i32, y: i32, kx: i32, ky: i32) -> impl DynamicBundle + Send + Sync + 'static {
    (
        Kind::Bird,
        Position::new(x, y),
        MovingDir::new(kx, ky),
        Destroyable,
        Tiles::new(&[15, 16]),
    )
}

pub fn lbear(x: i32, y: i32, kx: i32, ky: i32) -> impl DynamicBundle + Send + Sync + 'static {
    (
        Kind::LBear,
        Position::new(x, y),
        MovingDir::new(kx, ky),
        Destroyable,
        Tiles::new(&[13, 14]),
    )
}

pub fn rbear(x: i32, y: i32, kx: i32, ky: i32) -> impl DynamicBundle + Send + Sync + 'static {
    (
        Kind::RBear,
        Position::new(x, y),
        MovingDir::new(kx, ky),
        Destroyable,
        Tiles::new(&[30, 31]),
    )
}

pub fn moving_box(x: i32, y: i32) -> impl DynamicBundle + Send + Sync + 'static {
    (Kind::MovingBox, Position::new(x, y), Moveable, MovingDir::zero(), Tiles::new(&[6]))
}

pub fn static_box(x: i32, y: i32) -> impl DynamicBundle + Send + Sync + 'static {
    (Kind::Box, Position::new(x, y), Moveable, Tiles::new(&[20]))
}


pub fn wall(x: i32, y: i32) -> impl DynamicBundle + Send + Sync + 'static {
    (Kind::Wall, Position::new(x, y), Tiles::new(&[3]))
}

pub fn bullet(x: i32, y: i32, kx: i32, ky: i32) -> impl DynamicBundle + Send + Sync + 'static {
    (
        Kind::Bullet,
        Position::new(x, y),
        Moveable,
        MovingDir::new(kx, ky),
        Tiles::new(if ky == 0 {&[36, 37]} else {&[38, 39]}),
    )
}
