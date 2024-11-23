use crate::dimval::DimVal;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Hash, PartialOrd)]
pub struct Vec3<T: DimVal = i64> {
    pub x: T,
    pub y: T,
    pub z: T,
}
impl<I: DimVal + Eq> Eq for Vec3<I> {}

impl<I: DimVal + Ord> Ord for Vec3<I> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl<T: DimVal> Vec3<T> {
    pub fn new(x: T, y: T, z: T) -> Self {
        Self { x, y, z }
    }
}

impl<T: DimVal + std::ops::Add> std::ops::Add for Vec3<T> {
    type Output = Vec3<T>;

    fn add(self, rhs: Vec3<T>) -> Self::Output {
        Self::Output {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl<T: DimVal + std::ops::AddAssign> std::ops::AddAssign for Vec3<T> {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z
    }
}

impl<T: DimVal + std::ops::Sub> std::ops::Sub for Vec3<T> {
    type Output = Vec3<T>;

    fn sub(self, rhs: Vec3<T>) -> Self::Output {
        Vec3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl<T: DimVal + std::ops::Mul<i64, Output = T>> std::ops::Mul<i64> for Vec3<T> {
    type Output = Vec3<T>;

    fn mul(self, scalar: i64) -> Self::Output {
        Self::Output {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
        }
    }
}

impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}
