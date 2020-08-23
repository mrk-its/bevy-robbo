use std::ops;
pub struct Robbo;
#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
pub struct Position(pub i32, pub i32);
pub struct Tile(pub u32);
#[derive(Debug, Eq, PartialEq)]
pub enum Kind {
    Wall,
    Bird,
    LBear,
    RBear,
    Robbo,
}
#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
pub struct MovingDir(pub i32, pub i32);

pub fn rotate_clockwise(dir: MovingDir) -> MovingDir {
    MovingDir(-dir.1, dir.0)
}

pub fn rotate_counter_clockwise(dir: MovingDir) -> MovingDir {
    MovingDir(dir.1, -dir.0)
}

impl ops::Neg for MovingDir {
    type Output = MovingDir;
    fn neg(self) -> MovingDir {
        MovingDir(-self.0, -self.1)
    }
}
impl ops::Add<MovingDir> for Position {
    type Output = Position;
    fn add(self, arg: MovingDir) -> Position {
        Position(self.0 + arg.0, self.1 + arg.1)
    }
}
