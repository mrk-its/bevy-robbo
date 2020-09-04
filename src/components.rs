pub struct Robbo;
pub struct Bomb;
pub struct Bird;
pub struct Bear(pub bool);
pub struct Bullet;
pub struct Wall;
pub struct PushBox;
pub struct Animation;
pub struct ForceField;

pub struct Moveable;
pub struct Destroyable;
pub enum Usable {
    Door,
    Teleport,
    Capsule,
}
pub struct LaserTail;
pub struct RoughUpdate;
pub struct Capsule;
pub struct Undestroyable;

#[derive(Default)]
pub struct LaserHead {
    pub is_moving_back: bool
}
pub struct BlasterHead;

#[derive(Debug, Clone, Copy)]
pub struct Teleport(pub usize, pub usize);
#[derive(Debug, Clone, Copy)]
pub enum Collectable {
    Key,
    Screw,
    Ammo,
}

pub struct ShootingProp(pub f32);

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
pub struct Tile(pub u32);

type Int = i32;

#[derive(Debug, Default, Eq, Hash, PartialEq, Clone, Copy)]
pub struct IntVec2(pub Int, pub Int);

pub trait Int2Ops where Self::Output: Copy {
    type Output;
    fn get(&self) -> IntVec2;
    fn as_tuple(&self) -> (Int, Int) {
        (self.x(), self.y())
    }
    fn x(&self) -> Int {
        self.get().0
    }
    fn y(&self) -> Int {
        self.get().1
    }
    fn is_empty(&self) -> bool {
        self.x() == 0 && self.y() == 0
    }
    fn new(kx: Int, ky: Int) -> Self::Output;
    fn by_index(index: usize) -> Self::Output {
        //static ALL_DIRS: &[(i32, i32)] = &[(1, 0), (0, 1), (-1, 0), (0, -1)];
        static ALL_DIRS: &[(i32, i32)] = &[(0, 1), (1, 0), (0, -1), (-1, 0)];
        let (kx, ky) = ALL_DIRS[index];
        Self::new(kx, ky)
    }
    fn to_index(&self) -> usize {
        match (self.x(), self.y()) {
            (0, 1) => 0,
            (1, 0) => 1,
            (0, -1) => 2,
            (-1, 0) => 3,
            _ => 0,
        }
    }
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
    fn add<T>(&self, other: &T) -> Self::Output
    where
        T: Int2Ops,
    {
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

#[derive(Debug, Default, Eq, PartialEq, Hash, Clone, Copy)]
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

pub enum Rotatable {
    Regular,
    Random,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum GunType {
    Solid,
    Blaster,
    Burst,
}
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct ShootingDir {
    pub dir: IntVec2,
}

impl Int2Ops for ShootingDir {
    type Output = ShootingDir;
    fn new(x: Int, y: Int) -> Self {
        ShootingDir {
            dir: IntVec2(x, y),
        }
    }
    fn get(&self) -> IntVec2 {
        self.dir
    }
}

#[derive(Copy, Clone)]
pub struct Tiles {
    pub tiles: &'static [u32],
    pub current: usize,
}

impl Tiles {
    pub fn new(tiles: &'static [u32]) -> Self {
        Self { tiles, current: 0 }
    }
}


#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Magnet {
    pub dir: IntVec2,
}

impl Int2Ops for Magnet {
    type Output = Magnet;
    fn new(x: Int, y: Int) -> Self {
        Magnet {
            dir: IntVec2(x, y),
        }
    }
    fn get(&self) -> IntVec2 {
        self.dir
    }
}

pub mod prelude {
    pub use crate::components::*;
}