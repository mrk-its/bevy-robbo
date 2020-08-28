pub struct Robbo;
pub struct Moveable;
pub struct Destroyable;

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
pub struct Tile(pub u32);
#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum Kind {
    Wall,
    Bird,
    LBear,
    RBear,
    Robbo,
    Box,
    MovingBox,
    Bullet,
}

type Int = i32;

#[derive(Debug, Eq, Hash, PartialEq, Clone, Copy)]
pub struct IntVec2(pub Int, pub Int);

pub trait Int2Ops {
    type Output;
    fn get(&self) -> IntVec2;
    fn as_tuple(&self) -> (Int, Int) {
        (self.x(), self.y())
    }
    fn x(&self) -> Int {self.get().0}
    fn y(&self) -> Int {self.get().1}
    fn new(kx: Int, ky: Int) -> Self::Output;
    fn neg(&self) -> Self::Output {
        Self::new(-self.x(), -self.y())
    }
    fn rotate_clockwise(&self) -> Self::Output {
        Self::new(-self.y(), self.x())
    }
    fn rotate_counter_clockwise(&self) -> Self::Output {
        Self::new(self.y(), -self.x())
    }
    fn zero() -> Self::Output {
        Self::new(0, 0)
    }
    fn add<T>(&self, other: &T) -> Self::Output where T: Int2Ops {
        Self::new(self.x() + other.x(), self.y() + other.y())
    }
}


#[derive(Debug, Eq, Hash, PartialEq, Clone, Copy)]
pub struct Position(pub IntVec2);

impl Int2Ops for Position {
    type Output = Position;
    fn new(x: Int, y: Int) -> Self {
        Position(IntVec2(x, y))
    }
    fn get(&self) -> IntVec2 {
        self.0
    }
}
#[derive(Debug, Eq, Hash, PartialEq, Clone, Copy)]
pub struct StartPosition(pub IntVec2);

impl Int2Ops for StartPosition {
    type Output = StartPosition;
    fn new(x: Int, y: Int) -> Self {
        StartPosition(IntVec2(x, y))
    }
    fn get(&self) -> IntVec2 {
        self.0
    }
}

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
pub struct MovingDir(pub IntVec2);

impl Int2Ops for MovingDir {
    type Output = MovingDir;
    fn new(x: Int, y: Int) -> Self {
        MovingDir(IntVec2(x, y))
    }
    fn get(&self) -> IntVec2 {
        self.0
    }
}

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
pub struct ShootingDir(pub IntVec2);

impl Int2Ops for ShootingDir {
    type Output = ShootingDir;
    fn new(x: Int, y: Int) -> Self {
        ShootingDir(IntVec2(x, y))
    }
    fn get(&self) -> IntVec2 {
        self.0
    }
}

#[derive(Copy, Clone)]
pub struct Tiles {
    pub tiles: &'static [u32],
    pub current: usize,
}

impl Tiles {
    pub fn new(tiles: &'static [u32]) -> Self { Self { tiles, current: 0 } }
}
