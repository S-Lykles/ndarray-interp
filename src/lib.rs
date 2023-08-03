// Copyright (c) 2023 Jonas Bosse
//
// Licensed under the MIT license

//! The ndarray-interp crate provides interpolation algorithms
//! for interpolating _n_-dimesional data.
//!
//! 1D and 2D interpolation is supported. See the modules [interp1d] and [interp2d]
//!
//! # Custom interpolation strategy
//! This crate defines traits to allow implementation of user
//! defined interpolation algorithms.
//! see the `custom_strategy.rs` example.
//!

use thiserror::Error;

mod aliases;
pub mod interp1d;
pub mod interp2d;
mod vector_extensions;

pub use aliases::*;

/// Errors during Interpolator creation
#[derive(Debug, Error)]
pub enum BuilderError {
    /// Insufficient data for the chosen interpolation strategy
    #[error("{0}")]
    NotEnoughData(String),
    /// A interpolation axis is not strict monotonic rising
    #[error("{0}")]
    Monotonic(String),
    /// The lengths of interpolation axis and the
    /// corresponding data axis do not match
    #[error("{0}")]
    AxisLenght(String),
    #[error("{0}")]
    DimensionError(String),
}

/// Errors during Interpolation
#[derive(Debug, Error)]
pub enum InterpolateError {
    #[error("{0}")]
    OutOfBounds(String),
}
