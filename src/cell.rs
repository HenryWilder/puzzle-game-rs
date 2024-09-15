use std::{ops::*, cmp::*};

/// 3D grid position.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Vector3i {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl Vector3i {
    pub fn new(x: i32, y: i32, z: i32) -> Self {
        Self { x, y, z }
    }
}

impl Neg for Vector3i {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Add for Vector3i {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl AddAssign for Vector3i {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl Sub for Vector3i {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl SubAssign for Vector3i {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl Mul for Vector3i {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

impl MulAssign for Vector3i {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

impl Div for Vector3i {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
            z: self.z / rhs.z,
        }
    }
}

impl DivAssign for Vector3i {
    fn div_assign(&mut self, rhs: Self) {
        *self = *self / rhs;
    }
}

impl Add<i32> for Vector3i {
    type Output = Self;

    fn add(self, rhs: i32) -> Self::Output {
        Self {
            x: self.x + rhs,
            y: self.y + rhs,
            z: self.z + rhs,
        }
    }
}

impl AddAssign<i32> for Vector3i {
    fn add_assign(&mut self, rhs: i32) {
        *self = *self + rhs;
    }
}

impl Sub<i32> for Vector3i {
    type Output = Self;

    fn sub(self, rhs: i32) -> Self::Output {
        Self {
            x: self.x - rhs,
            y: self.y - rhs,
            z: self.z - rhs,
        }
    }
}

impl SubAssign<i32> for Vector3i {
    fn sub_assign(&mut self, rhs: i32) {
        *self = *self - rhs;
    }
}

impl Mul<i32> for Vector3i {
    type Output = Self;

    fn mul(self, rhs: i32) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl MulAssign<i32> for Vector3i {
    fn mul_assign(&mut self, rhs: i32) {
        *self = *self * rhs;
    }
}

impl Div<i32> for Vector3i {
    type Output = Self;

    fn div(self, rhs: i32) -> Self::Output {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl DivAssign<i32> for Vector3i {
    fn div_assign(&mut self, rhs: i32) {
        *self = *self / rhs;
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction3 {
    /// Positive [`Cell3::x`].
    East,
    /// Negative [`Cell3::x`].
    West,
    /// Positive [`Cell3::y`].
    North,
    /// Negative [`Cell3::y`].
    South,
    /// Positive [`Cell3::z`].
    Up,
    /// Negative [`Cell3::z`].
    Down,
}

// Direction3 -> Cell3 is trivial.
// Cell3 -> Direction3 is complicated without enforcing integer normalization.
impl From<Direction3> for Vector3i {
    fn from(value: Direction3) -> Vector3i {
        Vector3i {
            x: match value { Direction3::East  => 1, Direction3::West  => -1, _ => 0, },
            y: match value { Direction3::North => 1, Direction3::South => -1, _ => 0, },
            z: match value { Direction3::Up    => 1, Direction3::Down  => -1, _ => 0, },
        }
    }
}

impl Neg for Direction3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        match self {
            Self::East  => Self::West,
            Self::West  => Self::East,
            Self::North => Self::South,
            Self::South => Self::North,
            Self::Up    => Self::Down,
            Self::Down  => Self::Up,
        }
    }
}

impl Add<Direction3> for Vector3i {
    type Output = Self;

    fn add(self, rhs: Direction3) -> Self::Output {
        self + Vector3i::from(rhs)
    }
}

impl AddAssign<Direction3> for Vector3i {
    fn add_assign(&mut self, rhs: Direction3) {
        *self = *self + rhs;
    }
}

impl Sub<Direction3> for Vector3i {
    type Output = Self;

    fn sub(self, rhs: Direction3) -> Self::Output {
        self + Vector3i::from(-rhs)
    }
}

impl SubAssign<Direction3> for Vector3i {
    fn sub_assign(&mut self, rhs: Direction3) {
        *self = *self - rhs;
    }
}

pub enum Axis3i {
    /// [`Vector3i::y`] and [`Vector3i::z`] change while [`Vector3i::x`] does not.
    AroundX,
    /// [`Vector3i::x`] and [`Vector3i::z`] change while [`Vector3i::y`] does not.
    AroundY,
    /// [`Vector3i::x`] and [`Vector3i::y`] change while [`Vector3i::z`] does not.
    AroundZ,
}
