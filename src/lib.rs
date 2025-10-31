#![feature(portable_simd)]

pub mod nuc;
pub mod block;
pub mod base;
pub mod baseword;

pub use nuc::NucWord;
pub use block::NucBlockVec;