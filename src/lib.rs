#![allow(unused)]
// #![deny(missing_docs)]
//! # airborne
//!
//!  A Heavily-Typed Rust Statistics & Finance Library
//!
//!

mod numeric;
pub mod types;

#[macro_use] // pub(crate) but for macros,
// macros are now available for all modules bellow
pub(crate) mod compute;

#[macro_use] // pub(crate) but for macros,
mod dataset;
use std::process::Output;

pub use dataset::*;

pub mod error;
mod marker;
pub(crate) mod utils;

mod risk_metrics;
pub use risk_metrics::*;

pub mod capital;
pub mod finance;
pub mod stats;

pub use error::{Result, StatsError};
pub use marker::{Marker, Population, Sample};
pub use numeric::Numeric;

pub mod prelude {
    pub use crate::dataset::{DataSet, PopulationData, SampleData};
    pub use crate::error::{Result as StatsResult, StatsError};
    pub use crate::marker::{Population, Sample};
}
