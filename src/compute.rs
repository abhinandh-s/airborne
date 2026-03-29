#![allow(dead_code)]
// this module provides funtions which we use for internal arithemetics
//
// All computations work through this single trait.
// We never call `f64` or `rust_decimal::Decimal` in any other module.
//
// ## Example
//
// fn compute_something(v: f64) -> f64 {
//      // do all the computations using `N` type
//          `N` will deal with the Feature Flag enabled
//
//      N::cf_to_f64() // return as f64
// }
//
use std::ops::{Add, Div, Mul, Neg, Sub};

use crate::error::{Result, StatsError};
use crate::numeric::Numeric;

mod private {
    pub trait Sealed {}
}

/// Trait is essential, as `f64` and `Decimal` have different methods.
/// A bare `type N = Decimal` can't do this job.
pub trait ComputeFloat:
    private::Sealed
    + Copy
    + PartialOrd
    + Add<Output = Self>
    + Sub<Output = Self>
    + Mul<Output = Self>
    + Div<Output = Self>
    + Neg<Output = Self>
    + std::fmt::Debug
    + std::fmt::Display
    + 'static
{
    // Consts
    fn cf_zero() -> Self;
    fn cf_one() -> Self;
    fn cf_two() -> Self;
    fn cf_three() -> Self;
    fn cf_neg_infinity() -> Self;
    fn cf_infinity() -> Self;
    fn cf_hundred() -> Self;

    // Convert from an `f64`
    fn cf_from_f64(v: f64) -> Self;
    fn cf_from_usize(v: usize) -> Self;

    fn cf_to_f64(self) -> f64;

    fn cf_is_finite(self) -> bool;
    fn cf_is_zero(self) -> bool;

    fn cf_abs(self) -> Self;
    fn cf_min(self, rhs: Self) -> Self;
    fn cf_max(self, rhs: Self) -> Self;
    fn cf_clamp(self, lo: Self, hi: Self) -> Self;

    fn cf_powi(self, exp: i32) -> Self;
    fn cf_powf(self, exp: f64) -> Self;

    fn cf_sqrt(self) -> Self;

    /// Natural logarithm.  Panics for non-positive values.
    fn cf_ln(self) -> Self;

    /// `e^self`.
    fn cf_exp(self) -> Self;

    #[inline]
    fn cf_div(self, rhs: Self) -> Self {
        self / rhs
    }

    /// Sum an iterator of `Self` values.
    fn cf_sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::cf_zero(), |acc, x| acc + x)
    }

    #[cfg(test)]
    fn cf_tol() -> Self;
}

impl private::Sealed for f64 {}

impl ComputeFloat for f64 {
    fn cf_zero() -> Self {
        0.0
    }

    fn cf_one() -> Self {
        1.0
    }

    fn cf_two() -> Self {
        2.0
    }

    fn cf_three() -> Self {
        3.0
    }

    fn cf_neg_infinity() -> Self {
        f64::NEG_INFINITY
    }

    fn cf_infinity() -> Self {
        f64::INFINITY
    }

    fn cf_hundred() -> Self {
        100.0
    }

    fn cf_from_f64(v: f64) -> Self {
        v
    }

    fn cf_from_usize(v: usize) -> Self {
        v as f64
    }

    fn cf_to_f64(self) -> f64 {
        self
    }

    fn cf_is_finite(self) -> bool {
        f64::is_finite(self)
    }

    fn cf_is_zero(self) -> bool {
        self == 0.0
    }

    fn cf_abs(self) -> Self {
        f64::abs(self)
    }

    fn cf_min(self, other: Self) -> Self {
        f64::min(self, other)
    }

    fn cf_max(self, other: Self) -> Self {
        f64::max(self, other)
    }

    fn cf_clamp(self, min: Self, max: Self) -> Self {
        f64::clamp(self, min, max)
    }

    fn cf_powi(self, n: i32) -> Self {
        f64::powi(self, n)
    }

    fn cf_powf(self, n: f64) -> Self {
        f64::powf(self, n)
    }

    fn cf_sqrt(self) -> Self {
        f64::sqrt(self)
    }

    fn cf_exp(self) -> Self {
        f64::exp(self)
    }

    #[inline]
    fn cf_ln(self) -> Self {
        f64::ln(self)
    }

    #[cfg(test)]
    fn cf_tol() -> Self {
        1e-10
    }
}

#[cfg(feature = "precision")]
mod decimal_impl {
    use super::{ComputeFloat, private};
    use rust_decimal::MathematicalOps;
    use rust_decimal::prelude::*;

    impl private::Sealed for Decimal {}

    impl ComputeFloat for Decimal {
        #[inline]
        fn cf_zero() -> Decimal {
            Decimal::ZERO
        }
        #[inline]
        fn cf_one() -> Decimal {
            Decimal::ONE
        }
        #[inline]
        fn cf_two() -> Decimal {
            Decimal::TWO
        }
        #[inline]
        fn cf_three() -> Decimal {
            Decimal::from(3u32)
        }
        // Decimal has no infinity — we use MAX / MIN as sentinels.
        #[inline]
        fn cf_neg_infinity() -> Decimal {
            Decimal::MIN
        }
        #[inline]
        fn cf_infinity() -> Decimal {
            Decimal::MAX
        }
        #[inline]
        fn cf_hundred() -> Decimal {
            Decimal::ONE_HUNDRED
        }

        #[inline]
        fn cf_from_f64(v: f64) -> Decimal {
            Decimal::from_f64(v).expect("f64 constant not representable as Decimal")
        }
        #[inline]
        fn cf_from_usize(v: usize) -> Decimal {
            Decimal::from(v as u64)
        }
        #[inline]
        fn cf_to_f64(self) -> f64 {
            self.to_f64().expect("Decimal not representable as f64")
        }

        // Decimal is always finite (no NaN/inf in the type).
        #[inline]
        fn cf_is_finite(self) -> bool {
            true
        }
        #[inline]
        fn cf_is_zero(self) -> bool {
            self.is_zero()
        }

        #[inline]
        fn cf_abs(self) -> Decimal {
            self.abs()
        }
        #[inline]
        fn cf_min(self, r: Decimal) -> Decimal {
            self.min(r)
        }
        #[inline]
        fn cf_max(self, r: Decimal) -> Decimal {
            self.max(r)
        }
        #[inline]
        fn cf_clamp(self, lo: Decimal, hi: Decimal) -> Decimal {
            if self < lo {
                lo
            } else if self > hi {
                hi
            } else {
                self
            }
        }

        #[inline]
        fn cf_powi(self, e: i32) -> Decimal {
            // rust_decimal's powi via MathematicalOps
            MathematicalOps::powi(&self, e as i64)
        }
        #[inline]
        fn cf_powf(self, e: f64) -> Decimal {
            let exp = Decimal::from_f64(e).expect("powf exponent not representable");
            MathematicalOps::powd(&self, exp)
        }
        #[inline]
        fn cf_sqrt(self) -> Decimal {
            MathematicalOps::sqrt(&self).expect("sqrt of negative Decimal")
        }
        #[inline]
        fn cf_ln(self) -> Decimal {
            MathematicalOps::ln(&self)
        }
        #[inline]
        fn cf_exp(self) -> Decimal {
            MathematicalOps::exp(&self)
        }

        #[cfg(test)]
        fn cf_tol() -> Self {
            // Decimal::from_f64 is not const, but fine for test use
            Decimal::from_str("0.0000000000000000000000001").unwrap() // 1e-25
        }
    }
}

#[cfg(not(feature = "precision"))]
pub type N = f64;

#[cfg(feature = "precision")]
pub type N = rust_decimal::Decimal;

pub(crate) fn to_n<T: Numeric>(val: T, index: usize) -> Result<N> {
    let f = val.to_f64().ok_or(StatsError::ConversionError { index })?;

    Ok(N::cf_from_f64(f))
}

/// Convert an entire slice to `Vec<f64>`, rejecting NaN and ±∞.
pub(crate) fn to_n_vec<T: Numeric>(data: &[T]) -> Result<Vec<N>> {
    if data.is_empty() {
        return Err(StatsError::EmptyIterator);
    }
    data.iter()
        .enumerate()
        .map(|(index, &val)| {
            let f = to_n(val, index)?;

            // Only f64 can be non-finite; Decimal cannot.
            #[cfg(not(feature = "precision"))]
            if !f.cf_is_finite() {
                return Err(StatsError::InvalidValue { index });
            }
            Ok(f)
        })
        .collect()
}

/// Sort a `Vec<f64>` ascending (safe after NaN has been rejected).
pub(crate) fn sort_n_asc(mut v: Vec<N>) -> Vec<N> {
    // SAFETY: NaN values are rejected by to_f64_vec before reaching here.
    v.sort_unstable_by(|a, b| a.partial_cmp(b).expect("NaN after validation"));
    v
}

#[cfg(test)]
pub(crate) mod assert {
    use crate::compute::{ComputeFloat, N};

    pub fn approx_eq(actual: N, expected: N) -> bool {
        let diff = (actual - expected).cf_abs();
        if expected.cf_is_zero() {
            diff < N::cf_tol()
        } else {
            diff / expected.cf_abs() < N::cf_tol()
        }
    }
}

#[cfg(test)]
#[doc(hidden)]
#[macro_export]
macro_rules! assert_n_eq {
    ($actual:expr, $expected:expr) => {{
        use $crate::compute::ComputeFloat;

        let a: $crate::compute::N = $actual;
        let e = $crate::compute::N::cf_from_f64($expected);
        assert!(
            $crate::compute::assert::approx_eq(a, e),
            "assert_n_eq failed\n  actual:   {:?}\n  expected: {:?}\n  tol:      {:?}",
            a,
            e,
            $crate::compute::N::cf_tol()
        );
    }};
}

#[cfg(test)]
mod test {
    use crate::compute::{ComputeFloat, N};

    #[test]
    fn add_zero() {
        let x = N::cf_from_f64(3.7);
        assert_n_eq!(x + N::cf_zero(), 3.7);
        assert_eq!((x + N::cf_zero()).cf_to_f64(), x.cf_to_f64());
        assert_eq!((N::cf_zero() + x).cf_to_f64(), x.cf_to_f64());
    }
}

