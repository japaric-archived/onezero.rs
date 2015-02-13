//! Additive and multiplicative identities

#![deny(missing_docs)]
#![deny(warnings)]

/// Multiplicative identity
pub trait One {
    /// Returns one
    fn one() -> Self;
}

/// Additive identity
pub trait Zero {
    /// Returns zero
    fn zero() -> Self;
}

macro_rules! float {
    ($($ty:ty),+) => {$(
        impl One for $ty { fn one() -> $ty { 1. } }
        impl Zero for $ty { fn zero() -> $ty { 0. } })+
    }
}

float!(f32, f64);

macro_rules! int {
    ($($ty:ty),+) => {$(
        impl One for $ty { fn one() -> $ty { 1 } }
        impl Zero for $ty { fn zero() -> $ty { 0 } })+
    }
}

int!(i8, i16, i32, i64, isize, u8, u16, u32, u64, usize);
