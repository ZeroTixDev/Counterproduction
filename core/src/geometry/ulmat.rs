#![allow(clippy::suspicious_operation_groupings)]

use crate::geometry::ulvec::ULVec;
use crate::geometry::FMat;
use std::ops::*;

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Default, Debug, Hash)]
pub struct ULMat(pub ULVec, pub ULVec, pub ULVec);

impl ULMat {
    pub fn new(x: ULVec, y: ULVec, z: ULVec) -> Self {
        ULMat(x, y, z)
    }

    pub fn transpose(self) -> Self {
        ULMat(
            ULVec(self.0 .0, self.1 .0, self.2 .0),
            ULVec(self.0 .1, self.1 .1, self.2 .1),
            ULVec(self.0 .2, self.1 .2, self.2 .2),
        )
    }

    pub fn create(row1: [u64; 3], row2: [u64; 3], row3: [u64; 3]) -> Self {
        ULMat(
            ULVec(row1[0], row2[0], row3[0]),
            ULVec(row1[1], row2[1], row3[1]),
            ULVec(row1[2], row2[2], row3[2]),
        )
    }

    pub fn as_f32(self) -> FMat {
        FMat::new(self.0.as_f32(), self.1.as_f32(), self.2.as_f32())
    }

    pub fn zero() -> Self {
        ULMat(ULVec::zero(), ULVec::zero(), ULVec::zero())
    }

    pub fn identity() -> Self {
        ULMat(ULVec(1, 0, 0), ULVec(0, 1, 0), ULVec(0, 0, 1))
    }
}
impl From<[ULVec; 3]> for ULMat {
    fn from(x: [ULVec; 3]) -> Self {
        ULMat(x[0], x[1], x[2])
    }
}
impl From<(ULVec, ULVec, ULVec)> for ULMat {
    fn from((x, y, z): (ULVec, ULVec, ULVec)) -> Self {
        ULMat(x, y, z)
    }
}
impl From<[[u64; 3]; 3]> for ULMat {
    fn from(x: [[u64; 3]; 3]) -> Self {
        ULMat(x[0].into(), x[1].into(), x[2].into())
    }
}
impl From<ULMat> for [ULVec; 3] {
    fn from(x: ULMat) -> [ULVec; 3] {
        [x.0, x.1, x.2]
    }
}
impl From<ULMat> for (ULVec, ULVec, ULVec) {
    fn from(x: ULMat) -> (ULVec, ULVec, ULVec) {
        (x.0, x.1, x.2)
    }
}
impl From<ULMat> for FMat {
    fn from(x: ULMat) -> FMat {
        x.as_f32()
    }
}
impl Add for ULMat {
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        ULMat(self.0 + other.0, self.1 + other.1, self.2 + other.2)
    }
}
impl AddAssign for ULMat {
    fn add_assign(&mut self, other: Self) {
        self.0 += other.0;
        self.1 += other.1;
        self.2 += other.2;
    }
}
impl Sub for ULMat {
    type Output = Self;
    fn sub(self, other: Self) -> Self::Output {
        ULMat(self.0 - other.0, self.1 - other.1, self.2 - other.2)
    }
}
impl SubAssign for ULMat {
    fn sub_assign(&mut self, other: Self) {
        self.0 -= other.0;
        self.1 -= other.1;
        self.2 -= other.2;
    }
}
impl Mul for ULMat {
    type Output = Self;
    fn mul(self, other: Self) -> Self::Output {
        let sa = self.0;
        let sb = self.1;
        let sc = self.2;
        let oa = other.0;
        let ob = other.1;
        let oc = other.2;
        ULMat::new(
            ULVec::new(
                (sa.0 * oa.0) + (sb.0 * oa.1) + (sc.0 * oa.2),
                (sa.1 * oa.0) + (sb.1 * oa.1) + (sc.1 * oa.2),
                (sa.2 * oa.0) + (sb.2 * oa.1) + (sc.2 * oa.2),
            ),
            ULVec::new(
                (sa.0 * ob.0) + (sb.0 * ob.1) + (sc.0 * ob.2),
                (sa.1 * ob.0) + (sb.1 * ob.1) + (sc.1 * ob.2),
                (sa.2 * ob.0) + (sb.2 * ob.1) + (sc.2 * ob.2),
            ),
            ULVec::new(
                (sa.0 * oc.0) + (sb.0 * oc.1) + (sc.0 * oc.2),
                (sa.1 * oc.0) + (sb.1 * oc.1) + (sc.1 * oc.2),
                (sa.2 * oc.0) + (sb.2 * oc.1) + (sc.2 * oc.2),
            ),
        )
    }
}
impl Mul<ULVec> for ULMat {
    type Output = ULVec;
    fn mul(self, other: ULVec) -> ULVec {
        ULVec(self.0 * other, self.1 * other, self.2 * other)
    }
}
impl Mul<u64> for ULMat {
    type Output = Self;
    fn mul(self, other: u64) -> Self::Output {
        ULMat(self.0 * other, self.1 * other, self.2 * other)
    }
}
impl MulAssign<u64> for ULMat {
    fn mul_assign(&mut self, other: u64) {
        self.0 *= other;
        self.1 *= other;
        self.2 *= other;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::geometry::FVec;
    fn test_matrixes(first: ULMat, second: ULMat) {
        let first_f32 = first.as_f32();
        let second_f32 = second.as_f32();
        assert_eq!(first_f32 * second_f32, (first * second).as_f32());
        assert_eq!(second_f32 * first_f32, (second * first).as_f32());
    }
    #[test]
    fn test_matrix_multiply() {
        test_matrixes(
            ULMat::create([1, 2, 3], [4, 5, 6], [7, 8, 9]),
            ULMat::create([9, 8, 7], [6, 5, 4], [3, 2, 1]),
        );
    }
    #[test]
    fn test_conversion() {
        let x = ULMat::new(
            ULVec::new(1, 2, 3),
            ULVec::new(4, 5, 6),
            ULVec::new(7, 8, 9),
        );
        let y = FMat::new(
            FVec::new(1.0, 2.0, 3.0),
            FVec::new(4.0, 5.0, 6.0),
            FVec::new(7.0, 8.0, 9.0),
        );
        assert_eq!(x.as_f32(), y);
    }
}
