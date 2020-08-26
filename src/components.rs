use std::ops;
use bevy::prelude::*;
pub struct Robbo;
pub struct Moveable;
pub struct Destroyable;
#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
pub struct Position(pub i32, pub i32);
#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
pub struct Tile(pub u32);
#[derive(Debug, Eq, PartialEq)]
pub enum Kind {
    Wall,
    Bird,
    LBear,
    RBear,
    Robbo,
    Box,
    MovingBox,
}
pub fn rotate_clockwise(dir: MovingDir) -> MovingDir {
    MovingDir(-dir.1, dir.0)
}

pub fn rotate_counter_clockwise(dir: MovingDir) -> MovingDir {
    MovingDir(dir.1, -dir.0)
}

impl<T> ops::Add<T> for Position where T:Dir {
    type Output = Position;
    fn add(self, arg: T) -> Self::Output {
        let (kx, ky) = arg.get();
        Position(self.0 + kx, self.1 + ky)
    }
}

pub trait Dir {
    type Output;
    fn get(&self) -> (i32, i32);
    fn new(kx: i32, ky: i32) -> Self::Output;
    fn neg(&self) -> Self::Output {
        let (kx, ky) = self.get();
        Self::new(-kx, -ky)
    }
}

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
pub struct ShotingDir(pub i32, pub i32);
impl Dir for ShotingDir {
    type Output = ShotingDir;
    fn get(&self) -> (i32, i32) {(self.0, self.1)}
    fn new(kx: i32, ky: i32) -> Self::Output {ShotingDir(kx, ky)}
}
#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
pub struct MovingDir(pub i32, pub i32);
impl Dir for MovingDir {
    type Output = MovingDir;
    fn get(&self) -> (i32, i32) {(self.0, self.1)}
    fn new(kx: i32, ky: i32) -> Self::Output {MovingDir(kx, ky)}
}

fn test() {
    let x = Vec2::new(0.0, 0.0);
    let y = -x;
}