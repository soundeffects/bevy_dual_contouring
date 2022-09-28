//! [Manifold Dual Contouring](http://faculty.cs.tamu.edu/schaefer/research/dualsimp_tvcg.pdf).

pub use bbox::BoundingBox;
use nalgebra as na;
use std::fmt::Debug;

mod cell_configs;
mod dc;
mod plane;
mod qef;

pub use self::dc::ManifoldDualContouring;

/// Trait which allows to convert Self to usize, since To<usize> is not implemented by f32 and f64.
pub trait AsUSize {
    /// Convert Self to usize.
    fn as_usize(&self) -> usize;
}

impl AsUSize for f32 {
    fn as_usize(&self) -> usize {
        *self as usize
    }
}

impl AsUSize for f64 {
    fn as_usize(&self) -> usize {
        *self as usize
    }
}

#[cfg(test)]
#[macro_use]
extern crate approx;
