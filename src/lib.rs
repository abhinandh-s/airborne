//! # airborne
//!
//!  A Heavily-Typed Rust Statistics & Finance Library
//!
//!  ## Architecture
//!
//!  DataSet<T, M = Population>
//!     |
//!     |-> stats::CentralTendancy
//!     |-> stats::Dispersion
//!     |-> risk_metrics::RiskMetrics
//!
//! ## example
//!
//! ### DataSet construction
//!
//! ```rust,no_run
//! // this won't compile
//! let data = DataSet::new([1, 2, 3, 4, 5]).unwrap(); // type annotations needed for `dataset::DataSet<i32, _>`
//!                                                                // cannot satisfy `_: marker::Marker`
//! ```
//!
//! valid constructions:
//!
//! ```rust
//! fn build_data_set() {
//!     // Same same but different!
//!
//!     // build via type annotation
//!     let data_sample_01: DataSet<i32, Sample> = DataSet::new([1, 2, 3, 4, 5]).unwrap();
//!     let data_population_01: DataSet<i32, Population> = DataSet::new([1, 2, 3, 4, 5]).unwrap();
//!
//!     // Default
//!     let _data_default_population: DataSet<i32> = DataSet::new([1, 2, 3, 4, 5]).unwrap();
//! }
//! ```

// #![warn(missing_docs)]

mod numeric;

#[macro_use] // pub(crate) but for macros,
// macros are now available for all modules bellow
pub(crate) mod compute;
mod dataset;
pub mod error;
mod marker;
mod risk_metrics;
pub(crate) mod utils;

pub mod stats;

pub use stats::*;

pub use dataset::{DataSet, PopulationData, SampleData};
pub use error::{Result, StatsError};
pub use marker::{Marker, Population, Sample};
pub use numeric::Numeric;
pub use risk_metrics::*;

pub mod prelude {
    pub use crate::dataset::{DataSet, PopulationData, SampleData};
    pub use crate::error::{Result as StatsResult, StatsError};
    pub use crate::marker::{Population, Sample};
}
