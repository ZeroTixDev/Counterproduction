use ultraviolet::IVec3;
use ultraviolet::Isometry3;
use ultraviolet::Mat3;
use ultraviolet::Mat4;
use ultraviolet::Rotor3;
use ultraviolet::UVec3;
use ultraviolet::Vec3;
mod lmat;
mod lvec;
mod ulmat;
mod ulvec;
pub type FVec = Vec3;
pub type IVec = IVec3;
pub type UVec = UVec3;
pub type LVec = lvec::LVec;
pub type ULVec = ulvec::ULVec;
pub type Iso = Isometry3;
pub type Rot = Rotor3;
pub type FMat = Mat3;
pub type FHMat = Mat4;
pub type LMat = lmat::LMat;
pub type ULMat = ulmat::ULMat;
