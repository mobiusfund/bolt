use std::fmt::Debug;
use std::ops::{Add, AddAssign, Div, Mul, MulAssign, Neg, Sub, SubAssign};

use ark_serialize::{CanonicalDeserialize, CanonicalSerialize};

pub trait JoltField:
    'static
    + Sized
    + Neg<Output = Self>
    + Add<Output = Self>
    + Sub<Output = Self>
    + Mul<Output = Self>
    + Div<Output = Self>
    + AddAssign<Self>
    + SubAssign<Self>
    + MulAssign<Self>
    + core::iter::Sum<Self>
    + for<'a> core::iter::Sum<&'a Self>
    + core::iter::Product<Self>
    + for<'a> core::iter::Product<&'a Self>
    + Eq
    + Copy
    + Sync
    + Send
    + Debug
    + Default
    + CanonicalSerialize
    + CanonicalDeserialize
{
    const NUM_BYTES: usize;
    fn random<R: rand_core::RngCore>(rng: &mut R) -> Self;

    fn is_zero(&self) -> bool;
    fn is_one(&self) -> bool;
    fn zero() -> Self;
    fn one() -> Self;
    fn from_u64(n: u64) -> Option<Self>;
    fn square(&self) -> Self;
    fn from_bytes(bytes: &[u8]) -> Self;
}

pub mod ark;
pub mod binius;
