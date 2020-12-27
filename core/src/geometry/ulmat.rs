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
impl Into<[ULVec; 3]> for ULMat {
    fn into(self) -> [ULVec; 3] {
        [self.0, self.1, self.2]
    }
}
impl Into<(ULVec, ULVec, ULVec)> for ULMat {
    fn into(self) -> (ULVec, ULVec, ULVec) {
        (self.0, self.1, self.2)
    }
}
impl Into<FMat> for ULMat {
    fn into(self) -> FMat {
        self.as_f32()
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
        let tr = other.transpose();
        ULMat(
            ULVec(
                (self.0 * tr.0).sum(),
                (self.0 * tr.1).sum(),
                (self.0 * tr.2).sum(),
            ),
            ULVec(
                (self.1 * tr.0).sum(),
                (self.1 * tr.1).sum(),
                (self.1 * tr.2).sum(),
            ),
            ULVec(
                (self.2 * tr.0).sum(),
                (self.2 * tr.1).sum(),
                (self.2 * tr.2).sum(),
            ),
        )
    }
}
impl Mul<ULVec> for ULMat {
    type Output = ULVec;
    fn mul(self, other: ULVec) -> ULVec {
        ULVec(
            (self.0 * other).sum(),
            (self.1 * other).sum(),
            (self.2 * other).sum(),
        )
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
}
