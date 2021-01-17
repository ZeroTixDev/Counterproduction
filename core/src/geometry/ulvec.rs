use crate::geometry::FVec;
use crate::geometry::UVec;
use std::ops::*;

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Default, Debug, Hash)]
pub struct ULVec(pub u64, pub u64, pub u64);

impl ULVec {
    pub const fn new(x: u64, y: u64, z: u64) -> Self {
        ULVec(x, y, z)
    }

    pub fn as_f32(self) -> FVec {
        FVec::new(self.0 as f32, self.1 as f32, self.2 as f32)
    }

    pub fn sum(self) -> u64 {
        self.0 + self.1 + self.2
    }

    pub fn zero() -> Self {
        ULVec(0, 0, 0)
    }
}
impl From<UVec> for ULVec {
    fn from(x: UVec) -> Self {
        ULVec(x.x.into(), x.y.into(), x.z.into())
    }
}
impl From<[u64; 3]> for ULVec {
    fn from(x: [u64; 3]) -> Self {
        ULVec(x[0], x[1], x[2])
    }
}
impl From<(u64, u64, u64)> for ULVec {
    fn from((x, y, z): (u64, u64, u64)) -> Self {
        ULVec(x, y, z)
    }
}
impl From<ULVec> for [u64; 3] {
    fn from(x: ULVec) -> [u64; 3] {
        [x.0, x.1, x.2]
    }
}
impl From<ULVec> for (u64, u64, u64) {
    fn from(x: ULVec) -> (u64, u64, u64) {
        (x.0, x.1, x.2)
    }
}
impl From<ULVec> for FVec {
    fn from(x: ULVec) -> FVec {
        x.as_f32()
    }
}
impl Add for ULVec {
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        ULVec(self.0 + other.0, self.1 + other.1, self.2 + other.2)
    }
}
impl AddAssign for ULVec {
    fn add_assign(&mut self, other: Self) {
        self.0 += other.0;
        self.1 += other.1;
        self.2 += other.2;
    }
}
impl Sub for ULVec {
    type Output = Self;
    fn sub(self, other: Self) -> Self::Output {
        ULVec(self.0 - other.0, self.1 - other.1, self.2 - other.2)
    }
}
impl SubAssign for ULVec {
    fn sub_assign(&mut self, other: Self) {
        self.0 -= other.0;
        self.1 -= other.1;
        self.2 -= other.2;
    }
}
impl Mul for ULVec {
    type Output = u64;
    fn mul(self, other: Self) -> Self::Output {
        self.0 * other.0 + self.1 * other.1 + self.2 * other.2
    }
}
impl Mul<u64> for ULVec {
    type Output = Self;
    fn mul(self, other: u64) -> Self::Output {
        ULVec(self.0 * other, self.1 * other, self.2 * other)
    }
}
impl MulAssign<u64> for ULVec {
    fn mul_assign(&mut self, other: u64) {
        self.0 *= other;
        self.1 *= other;
        self.2 *= other;
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_vec_multiply() {
        let x = ULVec::new(1, 2, 3);
        let y = ULVec::new(4, 5, 6);
        assert_eq!(x * y, 1 * 4 + 2 * 5 + 3 * 6);
    }
    #[test]
    fn test_conversion() {
        let x = ULVec::new(1, 2, 3);
        let y = FVec::new(1.0, 2.0, 3.0);
        assert_eq!(x.as_f32(), y);
    }
}
