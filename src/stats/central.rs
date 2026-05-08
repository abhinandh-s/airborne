use std::iter::Sum;
use std::ops::Div;

use num_traits::Float;
use num_traits::FromPrimitive;
use num_traits::Num;
use num_traits::Zero;

use crate::StatsError;
use crate::compute::{ComputeFloat, N};
use crate::error::Result;
use crate::numeric::NumOps;
use crate::types::NonZeroNum;
use crate::{DataSet, Marker, Numeric};

pub trait CentralTendency {
    type Output;
    /// # Arithmetic mean
    ///
    /// **formula**: `(x̅ ) = Σxi/ n`. ie, (sum of values / number of values)
    ///
    // ## Example
    //
    // ```rust
    // use airborne::DataSet;
    // use airborne::stats::CentralTendency;
    //
    // let data: DataSet<f64> = DataSet::try_from([10.0, 20.0, 30.0]).unwrap();
    // let result = data.mean();
    //
    // assert_eq!(result, Ok(20.0));
    // ```
    //
    // # Error
    //
    // returns error if value in data set
    //     1. is finite
    //     2. can't be convert to f64
    //
    // ref: https://en.wikipedia.org/wiki/Arithmetic_mean
    fn mean(&self) -> Result<Self::Output>;
}

impl<T: Numeric, M: Marker> CentralTendency for DataSet<T, M> {
    type Output = f64;

    fn mean(&self) -> Result<f64> {
        let v = self.to_n_vec()?; // <- only conversion in the whole call chain
        Ok(mean(&v)?.cf_to_f64())
    }
}

#[deprecated]
pub(crate) fn mean_n(series: &[N]) -> N {
    let sum = N::cf_sum(series.iter().copied());
    sum / N::cf_from_usize(series.len())
}

/// # Arithmetic mean
///
/// It take `&[T]` returns `T`,
///
/// formula: mean = total / n
pub fn mean<T>(series: &[T]) -> Result<T>
where
    T: NumOps + Sum + FromPrimitive + Zero,
{
    let sum = T::sum(series.iter().copied());
    let len = NonZeroNum::from_usize(series.len())?;
    Ok(sum / *len)
}

/// impl `CentralTendency` for `Vec`, `&[T]`, `[T; N]`, `Box<[T]>`, `Box<[T; N]>`
///
/// if a type impls deref to slice this should work on those too.
///
/// # Example
///
/// ```
///  use airborne::stats::CentralTendency;
///
/// let test_series_01: [f64; 9] = [1, 2, 3, 4, 5, 6, 7, 8, 9].map(|x| x as f64);
/// let test_series_02: &[f64] = &[1, 2, 3, 4, 5, 6, 7, 8, 9].map(|x| x as f64);
/// let test_series_03 = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0];
/// let test_series_04: Box<[f64; 9]> = Box::new([1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0]);
/// let test_series_05: Box<[f64]> = Box::new([1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0]);
///
/// let res_01 = test_series_01.mean().unwrap();
/// let res_02 = test_series_02.mean().unwrap();
/// let res_03 = test_series_03.mean().unwrap();
/// let res_04 = test_series_04.mean().unwrap();
/// let res_05 = test_series_05.mean().unwrap();
///
/// assert_eq!(res_01, res_02);
/// assert_eq!(res_02, res_03);
/// assert_eq!(res_03, res_04);
/// assert_eq!(res_04, res_05);
/// assert_eq!(res_05, res_01);
/// ```
impl<T> CentralTendency for [T]
where
    T: Zero + Sum + FromPrimitive + NumOps,
{
    type Output = T;

    fn mean(&self) -> Result<Self::Output> {
        mean(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mean_basic() {
        let data = &[1.0, 2.0, 3.0];
        let result = data.mean().unwrap();
        assert_n_eq!(N::cf_from_f64(result), 2.0);
    }

    #[test]
    fn test_mean_empty() -> Result<()> {
        let test_series_01: [N; 9] = [1, 2, 3, 4, 5, 6, 7, 8, 9].map(|x| nusize!(x));
        let res_01 = test_series_01.mean().unwrap();
        assert_eq!(res_01, nf64!(5.0));
        Ok(())
    }
}
