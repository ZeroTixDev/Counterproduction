#![allow(clippy::suspicious_operation_groupings)]

use crate::geometry::lvec::LVec;
use crate::geometry::FMat;
use std::ops::*;

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Default, Debug, Hash)]
pub struct LMat(pub LVec, pub LVec, pub LVec);

impl LMat {
    pub fn new(x: LVec, y: LVec, z: LVec) -> Self {
        LMat(x, y, z)
    }

    pub fn transpose(self) -> Self {
        LMat(
            LVec(self.0 .0, self.1 .0, self.2 .0),
            LVec(self.0 .1, self.1 .1, self.2 .1),
            LVec(self.0 .2, self.1 .2, self.2 .2),
        )
    }

    pub fn create(row1: [i64; 3], row2: [i64; 3], row3: [i64; 3]) -> Self {
        LMat(
            LVec(row1[0], row2[0], row3[0]),
            LVec(row1[1], row2[1], row3[1]),
            LVec(row1[2], row2[2], row3[2]),
        )
    }

    pub fn as_f32(self) -> FMat {
        FMat::new(self.0.as_f32(), self.1.as_f32(), self.2.as_f32())
    }

    pub fn zero() -> Self {
        LMat(LVec::zero(), LVec::zero(), LVec::zero())
    }

    pub fn identity() -> Self {
        LMat(LVec(1, 0, 0), LVec(0, 1, 0), LVec(0, 0, 1))
    }
}
impl From<[LVec; 3]> for LMat {
    fn from(x: [LVec; 3]) -> Self {
        LMat(x[0], x[1], x[2])
    }
}
impl From<(LVec, LVec, LVec)> for LMat {
    fn from((x, y, z): (LVec, LVec, LVec)) -> Self {
        LMat(x, y, z)
    }
}
impl From<[[i64; 3]; 3]> for LMat {
    fn from(x: [[i64; 3]; 3]) -> Self {
        LMat(x[0].into(), x[1].into(), x[2].into())
    }
}
impl From<LMat> for [LVec; 3] {
    fn from(x: LMat) -> [LVec; 3] {
        [x.0, x.1, x.2]
    }
}
impl From<LMat> for (LVec, LVec, LVec) {
    fn from(x: LMat) -> (LVec, LVec, LVec) {
        (x.0, x.1, x.2)
    }
}
impl From<LMat> for FMat {
    fn from(x: LMat) -> FMat {
        x.as_f32()
    }
}
impl Add for LMat {
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        LMat(self.0 + other.0, self.1 + other.1, self.2 + other.2)
    }
}
impl AddAssign for LMat {
    fn add_assign(&mut self, other: Self) {
        self.0 += other.0;
        self.1 += other.1;
        self.2 += other.2;
    }
}
impl Sub for LMat {
    type Output = Self;
    fn sub(self, other: Self) -> Self::Output {
        LMat(self.0 - other.0, self.1 - other.1, self.2 - other.2)
    }
}
impl SubAssign for LMat {
    fn sub_assign(&mut self, other: Self) {
        self.0 -= other.0;
        self.1 -= other.1;
        self.2 -= other.2;
    }
}
impl Mul for LMat {
    type Output = Self;
    fn mul(self, other: Self) -> Self::Output {
        let sa = self.0;
        let sb = self.1;
        let sc = self.2;
        let oa = other.0;
        let ob = other.1;
        let oc = other.2;
        LMat::new(
            LVec::new(
                (sa.0 * oa.0) + (sb.0 * oa.1) + (sc.0 * oa.2),
                (sa.1 * oa.0) + (sb.1 * oa.1) + (sc.1 * oa.2),
                (sa.2 * oa.0) + (sb.2 * oa.1) + (sc.2 * oa.2),
            ),
            LVec::new(
                (sa.0 * ob.0) + (sb.0 * ob.1) + (sc.0 * ob.2),
                (sa.1 * ob.0) + (sb.1 * ob.1) + (sc.1 * ob.2),
                (sa.2 * ob.0) + (sb.2 * ob.1) + (sc.2 * ob.2),
            ),
            LVec::new(
                (sa.0 * oc.0) + (sb.0 * oc.1) + (sc.0 * oc.2),
                (sa.1 * oc.0) + (sb.1 * oc.1) + (sc.1 * oc.2),
                (sa.2 * oc.0) + (sb.2 * oc.1) + (sc.2 * oc.2),
            ),
        )
    }
}
impl Mul<LVec> for LMat {
    type Output = LVec;
    fn mul(self, other: LVec) -> LVec {
        LVec(self.0 * other, self.1 * other, self.2 * other)
    }
}
impl Mul<i64> for LMat {
    type Output = Self;
    fn mul(self, other: i64) -> Self::Output {
        LMat(self.0 * other, self.1 * other, self.2 * other)
    }
}
impl MulAssign<i64> for LMat {
    fn mul_assign(&mut self, other: i64) {
        self.0 *= other;
        self.1 *= other;
        self.2 *= other;
    }
}
impl Neg for LMat {
    type Output = Self;
    fn neg(self) -> Self::Output {
        LMat(-self.0, -self.1, -self.2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::geometry::FVec;
    fn test_matrixes(first: LMat, second: LMat) {
        let first_f32 = first.as_f32();
        let second_f32 = second.as_f32();
        assert_eq!(first_f32 * second_f32, (first * second).as_f32());
        assert_eq!(second_f32 * first_f32, (second * first).as_f32());
    }
    #[test]
    fn test_matrix_multiply() {
        test_matrixes(
            LMat::create([1, 2, 3], [4, 5, 6], [7, 8, 9]),
            LMat::create([9, 8, 7], [6, 5, 4], [3, 2, 1]),
        );
    }
    #[test]
    fn test_conversion() {
        let x = LMat::new(LVec::new(1, 2, 3), LVec::new(4, 5, 6), LVec::new(7, 8, 9));
        let y = FMat::new(
            FVec::new(1.0, 2.0, 3.0),
            FVec::new(4.0, 5.0, 6.0),
            FVec::new(7.0, 8.0, 9.0),
        );
        assert_eq!(x.as_f32(), y);
    }
}
