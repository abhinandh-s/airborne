#![allow(unused)]
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
    fn cf_ln(self) -> f64 {
        f64::ln(self)
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

macro_rules! n_from_f64 {
    ($val:expr) => {{
        use $crate::compute::ComputeFloat;
        $crate::compute::N::cf_from_f64($val)
    }};
}

macro_rules! n_from_usize {
    ($val:expr) => {{
        use $crate::compute::ComputeFloat;
        $crate::compute::N::cf_from_usize($val)
    }};
}

macro_rules! n_sum {
    ($val:expr) => {{
        use $crate::compute::ComputeFloat;
        $crate::compute::N::cf_sum($val)
    }};
}

macro_rules! n_zero {
    () => {{
        use $crate::compute::ComputeFloat;
        $crate::compute::N::cf_zero()
    }};
}

/// where,
///     a = N
///     b = f64
#[doc(hidden)]
#[macro_export]
macro_rules! n_assert_eq {
    ($a:expr, $b:expr) => {{
        let a: f64 = ($a).cf_to_f64();
        let b: f64 = $b;

        #[cfg(not(feature = "precision"))]
        let tol: f64 = 1e-10_f64;
        #[cfg(feature = "precision")]
        let tol: f64 = 1e-25_f64;

        let err: f64 = (a - b).abs() / b.abs().max(1e-30);
        assert!(err < tol, "got {a}, expected {b} (rel err {err:.2e})");
    }};
}




#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_constants() {
        // Covering all constant lines (75-76, etc. in f64 / 132-157 in Decimal)
        assert_eq!(N::cf_to_f64(N::cf_zero()), 0.0);
        assert_eq!(N::cf_to_f64(N::cf_one()), 1.0);
        assert_eq!(N::cf_to_f64(N::cf_two()), 2.0);
        assert_eq!(N::cf_to_f64(N::cf_three()), 3.0);
        assert_eq!(N::cf_to_f64(N::cf_hundred()), 100.0);
        
        let inf = N::cf_infinity();
        let neg_inf = N::cf_neg_infinity();
        
        #[cfg(not(feature = "precision"))]
        {
            assert!(inf.is_infinite());
            assert!(neg_inf.is_infinite());
        }
        #[cfg(feature = "precision")]
        {
            // Decimal uses MIN/MAX as sentinels
            assert_eq!(inf, rust_decimal::Decimal::MAX);
            assert_eq!(neg_inf, rust_decimal::Decimal::MIN);
        }
    }

    #[test]
    fn test_conversions() {
        // Line 164-165 (cf_from_usize)
        let n_u = N::cf_from_usize(42);
        assert_eq!(N::cf_to_f64(n_u), 42.0);

        // n_from_f64 macro
        let n_f = n_from_f64!(3.14);
        assert_eq!(N::cf_to_f64(n_f), 3.14);
    }

    #[test]
    fn test_predicates() {
        let zero = N::cf_zero();
        let one = N::cf_one();
        
        assert!(zero.cf_is_zero());
        assert!(!one.cf_is_zero());
        assert!(one.cf_is_finite());
    }

    #[test]
    fn test_math_operations() {
        let val = n_from_f64!(2.0);
        
        // Covering lines like 209-223 (min, max, clamp, pow)
        assert_eq!(N::cf_to_f64(val.cf_abs()), 2.0);
        assert_eq!(N::cf_to_f64(val.cf_min(N::cf_one())), 1.0);
        assert_eq!(N::cf_to_f64(val.cf_max(N::cf_three())), 3.0);
        assert_eq!(N::cf_to_f64(val.cf_clamp(N::cf_zero(), N::cf_one())), 1.0);
        
        // Power and Roots
        assert_eq!(N::cf_to_f64(val.cf_powi(3)), 8.0);
        n_assert_eq!(val.cf_powf(2.0), 4.0);
        n_assert_eq!(n_from_f64!(9.0).cf_sqrt(), 3.0);
    }

    #[test]
    fn test_transcendental() {
        // Covering cf_ln and cf_exp (lines 241-246)
        let e = n_from_f64!(1.0).cf_exp();
        n_assert_eq!(e.cf_ln(), 1.0);
    }

    #[test]
    fn test_arithmetic_trait_methods() {
        let a = n_from_f64!(10.0);
        let b = n_from_f64!(2.0);
        
        // Explicitly calling the trait provided cf_div
        n_assert_eq!(a.cf_div(b), 5.0);
        // Negation
        n_assert_eq!(-a, -10.0);
    }

    #[test]
    fn test_sorting_and_vec_logic() {
        // Cover sort_n_asc and to_n_vec
        let data = vec![3.0, 1.0, 2.0];
        let n_vec = to_n_vec(&data).unwrap();
        let sorted = sort_n_asc(n_vec);
        
        assert_eq!(N::cf_to_f64(sorted[0]), 1.0);
        assert_eq!(N::cf_to_f64(sorted[1]), 2.0);
        assert_eq!(N::cf_to_f64(sorted[2]), 3.0);
    }

    #[test]
    fn test_to_n_error_paths() {
        // Cover line 271 (ConversionError) if applicable to type, 
        // but mostly invalid f64 check on line 285-288.
        #[cfg(not(feature = "precision"))]
        {
            let data = vec![f64::NAN];
            let res = to_n_vec(&data);
            assert!(res.is_err());
        }
    }

    #[test]
    fn test_macros() {
        // Cover n_zero and n_from_usize
        let z: N = n_zero!();
        let forty_two: N = n_from_usize!(42);
        
        assert!(z.cf_is_zero());
        assert_eq!(forty_two.cf_to_f64(), 42.0);
    }

#[test]
fn test_all_compute_float_methods() {
    // 1. Constants (Covers lines 75-76, 132-157)
    let _ = N::cf_two();
    let _ = N::cf_three();
    let _ = N::cf_infinity();
    let _ = N::cf_neg_infinity();

    // 2. Math functions (Covers lines 241-253)
    let val = n_from_f64!(2.0);
    let _ = val.cf_powf(2.0);
    let _ = val.cf_exp();
    let _ = val.cf_ln();
    let _ = val.cf_sqrt();
    
    // 3. Comparisons and Clamps (Covers lines 209-223)
    let _ = val.cf_max(N::cf_one());
    let _ = val.cf_min(N::cf_one());
    let _ = val.cf_clamp(N::cf_zero(), N::cf_one());
}

}
