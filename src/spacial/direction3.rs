//! A 3D cardinal direction.

use std::{ops::*, cmp::*};
use super::vector3i::Vector3i;

/// A 3D cardinal direction.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction3 {
    /// Positive [`Vector3i::x`].
    East =  0b100,
    /// Negative [`Vector3i::x`].
    West = -0b100,
    /// Positive [`Vector3i::y`].
    North =  0b010,
    /// Negative [`Vector3i::y`].
    South = -0b010,
    /// Positive [`Vector3i::z`].
    Up   =  0b001,
    /// Negative [`Vector3i::z`].
    Down = -0b001,
}

// All Vector3i methods can be converted to Vector3 methods with minimal change
// But I haven't found a need to do that yet, so eh.

// Direction3 -> Vector3i is trivial.
// Vector3i -> Direction3 is complicated without enforcing integer normalization.
impl From<Direction3> for Vector3i {
    fn from(value: Direction3) -> Vector3i {
        Vector3i {
            x: match value {
                Direction3::East =>  1,
                Direction3::West => -1,
                _ => 0,
            },
            y: match value {
                Direction3::North =>  1,
                Direction3::South => -1,
                _ => 0,
            },
            z: match value {
                Direction3::Up   =>  1,
                Direction3::Down => -1,
                _ => 0,
            },
        }
    }
}

impl Neg for Direction3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        match self {
            // Self::None  => Self::None,
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
