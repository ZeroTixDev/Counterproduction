use crate::geometry::FVec;
use crate::geometry::IVec;
use std::ops::*;

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Default, Debug, Hash)]
pub struct LVec(pub i64, pub i64, pub i64);

impl LVec {
    pub fn new(x: i64, y: i64, z: i64) -> Self {
        LVec(x, y, z)
    }

    pub fn as_f32(self) -> FVec {
        FVec::new(self.0 as f32, self.1 as f32, self.2 as f32)
    }

    pub fn sum(self) -> i64 {
        self.0 + self.1 + self.2
    }

    pub fn zero() -> Self {
        LVec(0, 0, 0)
    }
}
impl From<IVec> for LVec {
    fn from(x: IVec) -> Self {
        LVec(x.x.into(), x.y.into(), x.z.into())
    }
}
impl From<[i64; 3]> for LVec {
    fn from(x: [i64; 3]) -> Self {
        LVec(x[0], x[1], x[2])
    }
}
impl From<(i64, i64, i64)> for LVec {
    fn from((x, y, z): (i64, i64, i64)) -> Self {
        LVec(x, y, z)
    }
}
impl Into<[i64; 3]> for LVec {
    fn into(self) -> [i64; 3] {
        [self.0, self.1, self.2]
    }
}
impl Into<(i64, i64, i64)> for LVec {
    fn into(self) -> (i64, i64, i64) {
        (self.0, self.1, self.2)
    }
}
impl Into<FVec> for LVec {
    fn into(self) -> FVec {
        self.as_f32()
    }
}
impl Add for LVec {
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        LVec(self.0 + other.0, self.1 + other.1, self.2 + other.2)
    }
}
impl AddAssign for LVec {
    fn add_assign(&mut self, other: Self) {
        self.0 += other.0;
        self.1 += other.1;
        self.2 += other.2;
    }
}
impl Sub for LVec {
    type Output = Self;
    fn sub(self, other: Self) -> Self::Output {
        LVec(self.0 - other.0, self.1 - other.1, self.2 - other.2)
    }
}
impl SubAssign for LVec {
    fn sub_assign(&mut self, other: Self) {
        self.0 -= other.0;
        self.1 -= other.1;
        self.2 -= other.2;
    }
}
impl Mul for LVec {
    type Output = i64;
    fn mul(self, other: Self) -> Self::Output {
        self.0 * other.0 + self.1 * other.1 + self.2 * other.2
    }
}
impl Mul<i64> for LVec {
    type Output = Self;
    fn mul(self, other: i64) -> Self::Output {
        LVec(self.0 * other, self.1 * other, self.2 * other)
    }
}
impl MulAssign<i64> for LVec {
    fn mul_assign(&mut self, other: i64) {
        self.0 *= other;
        self.1 *= other;
        self.2 *= other;
    }
}
